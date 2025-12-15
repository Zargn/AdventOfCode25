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
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

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
*/
mod part_two {
    use crate::reader;
    use std::{
        collections::{HashSet, VecDeque},
        error::Error,
    };

    #[derive(Default, PartialEq, Eq, Debug, Hash, Clone, Copy)]
    struct Joltage {
        lights: [u16; 10],
    }

    impl Joltage {
        fn from_joltage_pattern(data: &str) -> Result<Joltage, Box<dyn Error>> {
            let trimmed_data = &data[1..data.len() - 1];
            let mut new_lights = Joltage::default();
            for (i, data) in trimmed_data.split(',').enumerate() {
                new_lights.lights[i] = data.parse()?;
            }

            Ok(new_lights)
        }

        fn from_button(data: &&str) -> Result<Joltage, Box<dyn Error>> {
            let trimmed_data = data[1..data.len() - 1].split(',');
            let mut new_lights = Joltage::default();
            for i in trimmed_data {
                new_lights.lights[i.parse::<usize>()?] = 1;
            }

            Ok(new_lights)
        }

        fn combine(&self, other: &Joltage) -> Joltage {
            let mut new_lights = Joltage::default();
            for i in 0..10 {
                new_lights.lights[i] = self.lights[i] + other.lights[i];
            }
            new_lights
        }

        fn can_reach(&self, other: &Joltage) -> bool {
            for i in 0..self.lights.len() {
                if self.lights[i] > other.lights[i] {
                    return false;
                }
            }
            true
        }
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut total_steps = 0;

        for (line_nr, line) in reader::get_lines(data_path)?.enumerate() {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let (mut pattern_lookup, mut parts_iter) = (HashSet::new(), parts.iter());
            let mut processing_queue = VecDeque::from(vec![(Joltage::default(), 0)]);
            let button_count = parts_iter.len() - 2; // -1 for first and -1 for last elements.

            parts_iter.next();
            let mut buttons = Vec::new();
            for _ in 0..button_count {
                buttons.push(Joltage::from_button(
                    parts_iter.next().expect("This should never fail since we ensure to only call next the correct amount of times."),
                )?);
            }

            let desired_pattern = Joltage::from_joltage_pattern(
                parts_iter.next().expect("This should never fail since we ensure to only call next the correct amount of times."),
            )?;

            'outer: while let Some((lights, steps)) = processing_queue.pop_front() {
                for new_lights in buttons.iter().map(|b| lights.combine(b)) {
                    if pattern_lookup.insert(new_lights) && new_lights.can_reach(&desired_pattern) {
                        if new_lights == desired_pattern {
                            total_steps += 1 + steps;
                            break 'outer;
                        }
                        println!("Light: {new_lights:?}");
                        processing_queue.push_back((new_lights, steps + 1));
                    }
                }
            }

            println!("Completed line {line_nr}");
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
