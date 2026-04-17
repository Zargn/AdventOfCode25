#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 6440;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 250232501;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 5905;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

Time for some playing cards.

The data consists of rows where each row contains a hand and a bid. Our goal is to calculate the
total winnings of all hands.

We get the winnings of each row by multiplying the bid with the rank of the hand compared to all
others. Meaning the least valuable hand has rank 1, and the most valuable has a rank equal to the
amount of data rows (I.e. hands) in the data.

Value is determined using these rules:

Each hand has 5 characters with values in the following range ordered from most to least value.
A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2

Each hand has one of the following patterns again ordered from most to least valuable.
Five of a kind: 5X1
Four of a kind: 4x1 1x1
Full house: 3x1 2x1
Three of a kind: 3x1 1x1 1x1
Two pair: 2x1 2x1 1x1
One pair: 2x1 1x1 1x1
High card: five different cards.

The rank of two hands with the same pattern is decided by comparing characters left to right.
The first hand to have a higher value character during comparison gets the higher rank.



So, we need to read each hand order them in a list based on their value.
We could do this two ways. Either we calculate the rank of each hand as we read it, updating
the order as we go.
Or we read each hand recording it's pattern and characters, then placing each processed hand
in a list. We can then create our own ordering implementation where we do the above checks to
order them. Then we can use a library sorting algortihm to automatically sort the list.

Once we have our sorted list of hands we can just go through them multiplying their bids with
their rank, then adding it to a sum to get the result.
*/
mod part_one {
    use crate::reader;
    use std::error::Error;

    #[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
    enum HandType {
        FiveOfKind,
        FourOfKind,
        FullHouse,
        ThreeOfKind,
        TwoPair,
        OnePair,
        HighCard,
    }

    impl HandType {
        fn from_cards(cards: &[u8; 13]) -> Result<HandType, Box<dyn Error>> {
            let mut card_groups = Vec::new();
            for card in cards {
                if *card != 0 {
                    card_groups.push(card);
                }
            }

            Ok(match card_groups.len() {
                5 => HandType::HighCard,
                4 => HandType::OnePair,
                3 => {
                    for card_count in card_groups {
                        match card_count {
                            3 => return Ok(HandType::ThreeOfKind),
                            2 => return Ok(HandType::TwoPair),
                            _ => {}
                        }
                    }
                    return Err(format!("Could not determine type of {cards:?}").into());
                }
                2 => {
                    if *card_groups[0] == 1 || *card_groups[0] == 4 {
                        HandType::FourOfKind
                    } else {
                        HandType::FullHouse
                    }
                }
                1 => HandType::FiveOfKind,
                _ => return Err(format!("Invalid group count {}!", card_groups.len()).into()),
            })
        }
    }

    #[derive(Debug)]
    struct Hand {
        raw_cards: [u8; 5],
        hand_type: HandType,
        bid: u64,
    }

    impl Hand {
        fn from_str(str: &str) -> Result<Hand, Box<dyn Error>> {
            let mut parts = str.split(' ');
            let (cards, raw_cards) =
                get_cards(parts.next().expect("There is always at least 1 part!"))?;
            let bid = parts.next().ok_or("Missing bid string!")?.parse::<u64>()?;

            let hand_type = HandType::from_cards(&cards)?;

            Ok(Hand {
                raw_cards,
                hand_type,
                bid,
            })
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.hand_type
                .cmp(&other.hand_type)
                .then(self.raw_cards.cmp(&other.raw_cards).reverse())
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.hand_type == other.hand_type && self.raw_cards == other.raw_cards
        }
    }

    impl Eq for Hand {}

    fn get_cards(str: &str) -> Result<([u8; 13], [u8; 5]), Box<dyn Error>> {
        let mut cards = [0; 13];
        let mut raw_cards = [0; 5];
        for (i, c) in str.chars().enumerate() {
            let Some(kind) = get_kind(c) else {
                return Err(format!("Invalid char {c} in card string!").into());
            };

            cards[kind as usize] += 1;
            raw_cards[i] = kind;
        }

        Ok((cards, raw_cards))
    }

    fn get_kind(c: char) -> Option<u8> {
        // Since the lowest card value is 2 we can shift all ids two steps to the left.
        Some(match c {
            c if c.is_ascii_digit() => c.to_digit(10)? as u8 - 2,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => return None,
        })
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut hands = Vec::new();
        for line in reader::get_lines(data_path)? {
            hands.push(Hand::from_str(&line)?);
        }

        hands.sort();
        hands.reverse();

        let result = hands
            .iter()
            .enumerate()
            .map(|(i, hand)| hand.bid * (i + 1) as u64)
            .sum();
        Ok(result)
    }
}

//

//

/*
Part Two
##################################################################################################

Part two seems to be very similar just with one imporant change.
Card 'J' is now a "joker" card that will "mimic" whatever card would make the hand type the most
valuable. Meaning if a hand was earlier a ThreeOfKind but it also contained a 'J' then the 'J'
would take the value of the existing ThreeOfKind. Making the hand a FourOfKind instead.

When ordering hands if the kind is the same and the cards have to be compared directly, then 'J'
is still itself, but with it's value has been changed to be the lowest of all cards. Making the
new order this: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J

Now, this should be fairly easy to implement as a patch on the part one code.
First we would update the "get_kind" function. Since we represent the cards as a integer between
0 and 12 we need to do some slight adjustments. Start by changing the offset we apply to ascii
digits from -2 to -1. This leaves '0' unused. Then change 'J' to return a 0 instead of 9.
And finally change 'T' to return 9 instead of 8 to make space for the offset change from earlier.

Once this is done we need to update the code that creates the card groups used in the
HandType::from_cards method. First ensure we iterate through the cards from highest value to
lowest while skipping the final card slot for 'J'.
Then check how many 'J' cards exist. If there are any then check through the card groups, and
increase the largest card group by the amount of 'J' cards there are.

This will result in the hand type being the best possible using any available joker, while still
allowing the normal ordering to sort the hands correctly.

Once the above has been done then the result should match the expected values for Part Two.
*/
mod part_two {
    use crate::reader;
    use std::error::Error;

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;

        Err("NotImplemented: This problem has not been solved yet!".into())
    }
}

//

//

// Default controller code. Is the same between projects.
// ###############################################################################################

fn main() {
    println!("Running Program...");

    if cfg!(feature = "bench") {
        println!("Benchmarks are enabled!\n");
    }

    println!("\nPart One {}\n", {
        match benchmark!("calculate", { part_one::calculate("data.txt") }) {
            Ok(value) => format!("Result:\n{}", value),
            Err(err) => format!("FAILED with error:\n{}", err),
        }
    });
    println!("\nPart Two {}\n", {
        match benchmark!("calculate", { part_two::calculate("data.txt") }) {
            Ok(value) => format!("Result:\n{}", value),
            Err(err) => format!("FAILED with error:\n{}", err),
        }
    });
}
