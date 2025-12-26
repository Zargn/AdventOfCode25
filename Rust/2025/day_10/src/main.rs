mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 7;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 505;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 33;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 20002;

//

//

/*
Part One
##################################################################################################

We have three types of data per line.
First one section showing the desired light pattern [..#.#] where a . is off and # is on.
Then we have x sections showing the effects of a button press. [0,2,3] where the value shows which
light index it is connected to. Pressing the button will toggle all connected lights.
Lastly we have a section of joltage requirements which wont be used in Part One {3,6,4,9}

Our task is to figure out what is the lowest number of button presses required for the lights to
match the desired light pattern.

First though is just to brute force it. Not efficient, but should be good enough for this case.

Our task is to figure out what is the lowest number of button presses required for the lights to
match the desired light pattern.

First thought is just to brute force it. Not efficient, but should be good enough for this case.

Would work a bit like pathfinding.
We would store states in a queue, where the state is a light sequence and steps needed to get
there.

Then just dequeue the oldest state and create new states based on it using all available button
combinations. Compare the result states with the desired pattern. If a match is found return the
amount of steps to get there. If not then add all states back to the queue.

#### Improvements after completion...

A smarter more efficient way of doing it would be to track how many button presses each state
takes. For example, pressing button 1 and then button 2 will yield the same result as pressing
button 2 first and then nr 1.

So instead of tracking each idividual path to get to that point we could just track that point
and how many different button presses got us there, not their order.
Add a "buttonpresses" array of u8 values to the queue. The steps value can be left alone.
We can add a hashmap that takes in the lights array as key and holds steps as a value.
Before pushing a new state to the queue check the hashmap if that light pattern has been
reached already with fewer steps.
Although it might be enough to use a hashset, since we should alwyas be taking the shorter
steps first. Meaning if the hashmap contained the light pattern the steps there would always
be the same or less. I.e. the new light pattern will never be added.

Result: Adding a hashset and discarding duplicates improved processing time by a massive
amount. In debug mode execution went from 3.200 seconds to 0.097 seconds.
Release mode went from 0.310 seconds to 0.033 seconds.

We can then
*/
mod part_one {
    use crate::reader;
    use std::{
        collections::{HashSet, VecDeque},
        error::Error,
    };

    #[derive(Default, PartialEq, Eq, Debug, Hash, Clone, Copy)]
    struct Lights {
        lights: [bool; 10],
    }

    impl Lights {
        fn from_light_pattern(data: &str) -> Result<Lights, Box<dyn Error>> {
            let trimmed_data = &data[1..data.len() - 1];
            let mut new_lights = Lights::default();
            for (i, char) in trimmed_data.chars().enumerate() {
                new_lights.lights[i] = match char {
                    '#' => true,
                    '.' => false,
                    _ => return Err("Unexpected character in light pattern!".into()),
                };
            }
            Ok(new_lights)
        }

        fn from_button(data: &&str) -> Result<Lights, Box<dyn Error>> {
            let trimmed_data = data[1..data.len() - 1].split(',');
            let mut new_lights = Lights::default();
            for i in trimmed_data {
                new_lights.lights[i.parse::<usize>()?] = true;
            }

            Ok(new_lights)
        }

        fn combine(&self, other: &Lights) -> Lights {
            let mut new_lights = Lights::default();
            for i in 0..10 {
                new_lights.lights[i] = self.lights[i] ^ other.lights[i];
            }
            new_lights
        }
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut total_steps = 0;

        for line in reader::get_lines(data_path)? {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let (mut pattern_lookup, mut parts_iter) = (HashSet::new(), parts.iter());
            let mut processing_queue = VecDeque::from(vec![(Lights::default(), 0)]);
            let button_count = parts_iter.len() - 2; // -1 for first and -1 for last elements.

            let desired_pattern =
                Lights::from_light_pattern(parts_iter.next().expect("This should never be None"))?;

            let buttons = parts_iter
                .take(button_count)
                .map(Lights::from_button)
                .collect::<Result<Vec<_>, _>>()?;

            'outer: while let Some((lights, steps)) = processing_queue.pop_front() {
                for new_lights in buttons.iter().map(|b| lights.combine(b)) {
                    if pattern_lookup.insert(new_lights) {
                        if new_lights == desired_pattern {
                            total_steps += 1 + steps;
                            break 'outer;
                        }
                        processing_queue.push_back((new_lights, steps + 1));
                    }
                }
            }
        }

        Ok(total_steps)
    }
}

//

//

/*
Part Two
##################################################################################################

This time we wont use the light pattern at the start of the data line, and instead use the joltage
levels at the end of the line.
THe buttons increase the listed joltage level by one. Button (1,3) would increase joltage at index
1 by 1, and index 3 by 1.

Thanks to the improvements made to Part One I think that code will be quite compatible with this
new problem. The main thing we would need to change is to replace the Lights struct with a Joltage
struct.

We would need to add some extra logic to the loop as well. With the lights a 0 could become a 1,
and 1 a 0. Meaning it could "reverse". The joltage levels can only increase, so we will want to
add a check to discard any path that leads to any joltage level above the desired one. As that
will never be the correct path.

Update:
The initial idea wont work, at least in any reasonable amount of time.
Some other approach is needed.
I was thinking pathfinding might be the key if used with a heuristic, but I think that too would
run into issues with the extreme amount of possible paths.

So at this point I have been trying to learn about a whole math concept to try and solve this.
Apparently, integer linear combination is the key?
View buttons as blocks from now on in the text.
And each target joltage in the array as a counter.

My head hurts too much to do the math solution.
I think that with proper filtering and some kind of heuristic it should be feasable to brute
force it.

This version is at least capable of solving the first line rather quickly, but then gets stuck
on the second.

Experiments...
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
c1 [3] needs b5 + b6 to be exactly 3
c2 [5] needs b2 + b6 to be exactly 5
c3 [4] needs b3 + b4 + b5 to be exactly 4
c4 [7] needs b1 + b2 + b4 to be exactly 7

b1 can be anything between: 0..7
b2 can be anything between: 0..5
b3 can be anything between: 0..4
b4 can be anything between: 0..4
b5 can be anything between: 0..3
b6 can be anything between: 0..3

c1 [3] b5 = 3, b6 = 3
c2 [5]
c3 [4]
c4 [7]

c1 = 0 - b5(3) - b6(0)
c2 = 5 - b6(0) = b2 + b6 - b6 = 5 = b2
c3 = 4 - b5(3) = b3 + b4 + b5(3) - b5(3) = 1 = b3 + b4
c4 = 7 = b1 + b2 + b3

c1 [3]
c2 [5]
c3 [4]
c4 [7]


b1 can be anything between: (7 - b2 - b4)..7
b2 can be anything between: (5 - b6 || 7 - b1 - b4)..5
b3 can be anything between: 0..4
b4 can be anything between: 0..4
b5 can be anything between: 0..3
b6 can be anything between: 0..3

[#.#..#.##] (0,1,2,5,6,7,8) (1,4,6,7,8) (0,5,7) (0,1,2,6,7) (0,1,2,3,5,7,8) (0,1,5,7) (0,1,3,7,8) {138,150,10,13,17,127,25,155,38}


The answer can never be lower than the highest joltage level.

Having 4 counters, where the desired value combination is 3,5,4,7 but all starting at 0.
Then we have 6 blocks that increase the counters at the listed indexes by one with indexes ranging from 0 to 3.
The blocks are:
(3) (1,3) (2) (2,3) (0,2) (0, 1)

How many blocks needs to be used to get the desired counter values?


Update:
After trying for ages I decided to go with a existing solution found online.
The user tenthmascot on reddit made a very good solution in python, shown at the link below.
https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/

I have translated their original (Not optimized) solution into rust, and plan to spend some time to try and
improve it myself as a exercise to compensate for not technically "solving" it myself.
*/
mod part_two {
    use crate::reader;
    use std::{collections::HashMap, error::Error, iter::zip, time::Instant};

    #[derive(Default, PartialEq, Eq, Debug, Hash, Clone, Copy)]
    struct Joltage {
        counters: [u16; 10],
    }

    impl Joltage {
        fn from_joltage_pattern(data: &str) -> Result<Joltage, Box<dyn Error>> {
            let trimmed_data = &data[1..data.len() - 1];
            let mut new_lights = Joltage::default();
            for (i, data) in trimmed_data.split(',').enumerate() {
                new_lights.counters[i] = data.parse()?;
            }

            Ok(new_lights)
        }

        fn from_button(data: &&str) -> Result<Joltage, Box<dyn Error>> {
            let trimmed_data = data[1..data.len() - 1].split(',');
            let mut new_lights = Joltage::default();
            for i in trimmed_data {
                new_lights.counters[i.parse::<usize>()?] = 1;
            }

            Ok(new_lights)
        }

        fn combine(&self, other: &Joltage) -> Joltage {
            Joltage {
                counters: std::array::from_fn(|i| self.counters[i] + other.counters[i]),
            }
        }

        fn subtract_and_divide(&self, other: &Joltage) -> Joltage {
            Joltage {
                counters: std::array::from_fn(|i| (self.counters[i] - other.counters[i]) / 2),
            }
        }
    }

    fn combinations<T: Clone>(v: &[T], k: usize) -> Vec<Vec<T>> {
        if k == 0 {
            return vec![vec![]];
        }
        if v.len() < k {
            return vec![];
        }

        let mut result = Vec::new();

        for i in 0..=v.len() - k {
            let head = v[i].clone();
            for mut tail in combinations(&v[i + 1..], k - 1) {
                let mut combo = vec![head.clone()];
                combo.append(&mut tail);
                result.push(combo);
            }
        }

        result
    }

    fn patterns(blocks: Vec<Joltage>) -> HashMap<Joltage, u16> {
        let blocks_count = blocks.len();
        let mut out = HashMap::new();

        for pattern_len in 0..blocks_count + 1 {
            for buttons in combinations(&blocks, pattern_len) {
                let mut pattern = Joltage::default();
                buttons.iter().for_each(|j| {
                    pattern = pattern.combine(j);
                });
                out.entry(pattern).or_insert(pattern_len as u16);
            }
        }

        out
    }

    fn solve_single_aux(pattern_costs: &HashMap<Joltage, u16>, desired_pattern: Joltage) -> u64 {
        if desired_pattern.counters.iter().sum::<u16>() == 0 {
            return 0;
        }
        let mut answer = 1000000;
        for (pattern, pattern_cost) in pattern_costs.iter() {
            let valid = zip(pattern.counters, desired_pattern.counters)
                .map(|(i, j)| ((i > j) as u16) | ((i ^ j) & 1))
                .sum::<u16>()
                == 0;
            if valid {
                let new_goal = desired_pattern.subtract_and_divide(pattern);
                answer = answer
                    .min(*pattern_cost as u64 + (2 * solve_single_aux(pattern_costs, new_goal)));
            }
        }
        answer
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut total_steps = 0;

        for line in reader::get_lines(data_path)? {
            let start_time = Instant::now();

            let parts = line.split(' ').collect::<Vec<&str>>();
            let mut parts_iter = parts.iter().skip(1); // Skip first element.
            let block_count = parts_iter.len() - 1; // -1 for last elements.

            let mut blocks = Vec::new();
            for _ in 0..block_count {
                blocks.push(Joltage::from_button(
                    parts_iter.next().expect("This should never fail since we ensure to only call next the correct amount of times."),
                )?);
            }

            let desired_pattern = Joltage::from_joltage_pattern(
                parts_iter.next().expect("This should never fail since we ensure to only call next the correct amount of times."),
            )?;

            let subanswer = solve_single_aux(&patterns(blocks), desired_pattern);
            total_steps += subanswer;

            println!("Completed line, steps: {}", subanswer);
            println!("In {:?} time", Instant::now() - start_time,);
        }
        Ok(total_steps)
    }
}

//

//

// Default controller code. Is the same between projects.
// ###############################################################################################

fn main() {
    println!("Running Program...");
    print!("\nPart One ");
    match part_one::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("FAILED with error:\n{}", err),
    }
    print!("\nPart Two ");
    match part_two::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("FAILED with error:\n{}", err),
    }
    println!();
}
