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
*/
mod part_two {
    use crate::reader;
    use std::{
        cmp::{Ordering, Reverse},
        collections::{BinaryHeap, HashMap, HashSet, VecDeque},
        error::Error,
        iter::zip,
        thread,
        time::{Duration, Instant},
        u16,
    };

    #[derive(Default, PartialEq, Eq, Debug, Hash, Clone, Copy)]
    struct Joltage {
        lights: [u16; 10],
    }

    #[derive(Clone, Copy, Default)]
    struct QueueGroup {
        jolatage: Joltage,
        cost: u16,
        steps: u64,
        joltage_increases: u16,
    }

    impl Ord for QueueGroup {
        fn cmp(&self, other: &Self) -> Ordering {
            (self.cost).cmp(&(other.cost))
            //(other.heuristic).cmp(&(self.heuristic))
        }
    }

    impl PartialOrd for QueueGroup {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for QueueGroup {
        fn eq(&self, other: &Self) -> bool {
            self.jolatage == other.jolatage
        }
    }

    impl Eq for QueueGroup {}

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

        fn subtract_and_divide(&self, other: &Joltage) -> Joltage {
            let mut new = Joltage::default();
            for i in 0..10 {
                new.lights[i] = (self.lights[i] - other.lights[i]) / 2;
            }
            new
        }

        fn can_reach(&self, other: &Joltage) -> bool {
            for i in 0..self.lights.len() {
                if self.lights[i] > other.lights[i] {
                    return false;
                }
            }
            true
        }

        fn difference(&self, other: &Joltage) -> u64 {
            let mut diff = 0;
            for i in 0..10 {
                diff += ((self.lights[i] - other.lights[i]) as u64);
                //println!("{} - {} = {}", self.lights[i], other.lights[i], diff);
            }
            diff
        }

        fn heuristic(&self, target: &Joltage) -> u16 {
            let mut furthers_from_goal = 0;
            for i in 0..self.lights.len() {
                let diff = target.lights[i] - self.lights[i];
                if diff > furthers_from_goal {
                    furthers_from_goal = diff;
                }
            }
            furthers_from_goal
        }
    }

    /*


    from functools import cache
    from itertools import combinations
    import aocd

    def patterns(coeffs: list[tuple[int, ...]]) -> dict[tuple[int, ...], int]:
        out = {}
        num_buttons = len(coeffs)
        num_variables = len(coeffs[0])
        for pattern_len in range(num_buttons+1):
            for buttons in combinations(range(num_buttons), pattern_len):
                pattern = tuple(map(sum, zip((0,) * num_variables, *(coeffs[i] for i in buttons))))
                if pattern not in out:
                    out[pattern] = pattern_len
        return out

    def solve_single(coeffs: list[tuple[int, ...]], goal: tuple[int, ...]) -> int:
        pattern_costs = patterns(coeffs)
        @cache
        def solve_single_aux(goal: tuple[int, ...]) -> int:
            if all(i == 0 for i in goal): return 0
            answer = 1000000
            for pattern, pattern_cost in pattern_costs.items():
                if all(i <= j and i % 2 == j % 2 for i, j in zip(pattern, goal)):
                    new_goal = tuple((j - i)//2 for i, j in zip(pattern, goal))
                    answer = min(answer, pattern_cost + 2 * solve_single_aux(new_goal))
            return answer
        return solve_single_aux(goal)

    def solve(raw: str):
        answer = 0
        lines = raw.splitlines()
        for I, L in enumerate(lines, 1):
            _, *coeffs, goal = L.split()
            goal = tuple(int(i) for i in goal[1:-1].split(","))
            coeffs = [[int(i) for i in r[1:-1].split(",")] for r in coeffs]
            coeffs = [tuple(int(i in r) for i in range(len(goal))) for r in coeffs]

            subanswer = solve_single(coeffs, goal)
            print(f'Line {I}/{len(lines)}: answer {subanswer}')
            answer += subanswer
        print(answer)

    solve(open('input/10.test').read())
    solve(aocd.get_data(year=2025, day=10))




    def combinations(iterable, r):

    # combinations('ABCD', 2) → AB AC AD BC BD CD
    # combinations(range(4), 3) → 012 013 023 123

    pool = tuple(iterable)
    n = len(pool)
    if r > n:
        return
    indices = list(range(r))

    yield tuple(pool[i] for i in indices)
    while True:
        for i in reversed(range(r)):
            if indices[i] != i + n - r:
                break
        else:
            return
        indices[i] += 1
        for j in range(i+1, r):
            indices[j] = indices[j-1] + 1
        yield tuple(pool[i] for i in indices)

    */
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
            // (0..3).flat_map(|i| (0..4).map(move |j| (i, j)))
            for buttons in combinations(&blocks, pattern_len) {
                //for buttons in (0..blocks_count).map(|i| (blocks.iter().skip(i).take(pattern_len))) {
                let mut pattern = Joltage::default();
                //println!("pl: {}, {:?}\n", pattern_len, button);
                //println!("pattern_len: {}\n", pattern_len);
                buttons.iter().for_each(|j| {
                    //print!(" {:?} ", j);
                    pattern = pattern.combine(j);
                });
                //println!("\n{:?}", pattern);
                //let pattern = Joltage::default();
                out.entry(pattern).or_insert(pattern_len as u16);

                /*
                if !out.contains_key(&pattern) {
                    out.insert(pattern, pattern_len as u16);
                } // */
            }
        }

        //println!("\n\nOut:\n{:?}", out);
        out
    }

    fn solve_single_aux(pattern_costs: &HashMap<Joltage, u16>, desired_pattern: Joltage) -> u64 {
        if desired_pattern.lights.iter().sum::<u16>() == 0 {
            return 0;
        }
        let mut answer = 1000000;
        for (pattern, pattern_cost) in pattern_costs.iter() {
            if zip(pattern.lights, desired_pattern.lights)
                .map(|t| {
                    let (i, j) = (t.0, t.1);
                    //print!("| i: {}, j: {} ", i, j);
                    if i <= j && i % 2 == j % 2 {
                        0
                    } else {
                        1
                    }
                })
                .sum::<u16>()
                == 0
            {
                /*
                println!(
                    "Success: {:?} - {:?}",
                    pattern.lights, desired_pattern.lights
                ); */
                let new_goal = desired_pattern.subtract_and_divide(pattern);
                answer = answer
                    .min(*pattern_cost as u64 + (2 * solve_single_aux(pattern_costs, new_goal)));
            } else {
                /*
                println!(
                    "Failed: {:?} - {:?}",
                    pattern.lights, desired_pattern.lights
                ); */
            }
        }
        answer
    }
    /*
        def solve_single(coeffs: list[tuple[int, ...]], goal: tuple[int, ...]) -> int:
            pattern_costs = patterns(coeffs)
            @cache
            def solve_single_aux(goal: tuple[int, ...]) -> int:
                if all(i == 0 for i in goal): return 0
                answer = 1000000
                for pattern, pattern_cost in pattern_costs.items():
                    if all(i <= j and i % 2 == j % 2 for i, j in zip(pattern, goal)):
                        new_goal = tuple((j - i)//2 for i, j in zip(pattern, goal))
                        answer = min(answer, pattern_cost + 2 * solve_single_aux(new_goal))
                return answer
            return solve_single_aux(goal)
    */
    fn solve_single(blocks: Vec<Joltage>, desired_pattern: Joltage) -> u64 {
        let pattern_costs = patterns(blocks);
        solve_single_aux(&pattern_costs, desired_pattern)
    }

    fn solve(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut total_steps = 0;

        for line in reader::get_lines(data_path)? {
            let start_time = Instant::now();
            let parts = line.split(' ').collect::<Vec<&str>>();
            let mut parts_iter = parts.iter();
            let block_count = parts_iter.len() - 2; // -1 for first and -1 for last elements.

            parts_iter.next();
            let mut blocks = Vec::new();
            for _ in 0..block_count {
                blocks.push(Joltage::from_button(
                    parts_iter.next().expect("This should never fail since we ensure to only call next the correct amount of times."),
                )?);
            }

            let desired_pattern = Joltage::from_joltage_pattern(
                parts_iter.next().expect("This should never fail since we ensure to only call next the correct amount of times."),
            )?;

            let subanswer = solve_single(blocks, desired_pattern);
            //println!("Subanswer: {}", subanswer);
            total_steps += subanswer;
            println!("Completed line, steps: {}", subanswer);
            println!("In {:?} time", Instant::now() - start_time,);
        }
        //println!("Total steps: {total_steps}");
        Ok(total_steps)

        /*

            def solve(raw: str):
                answer = 0
                lines = raw.splitlines()
                for I, L in enumerate(lines, 1):
                    _, *coeffs, goal = L.split()
                    goal = tuple(int(i) for i in goal[1:-1].split(","))
                    coeffs = [[int(i) for i in r[1:-1].split(",")] for r in coeffs]
                    coeffs = [tuple(int(i in r) for i in range(len(goal))) for r in coeffs]

                    subanswer = solve_single(coeffs, goal)
                    print(f'Line {I}/{len(lines)}: answer {subanswer}')
                    answer += subanswer
                print(answer)
        */

        //todo!();
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let mut total_steps = 0;
        let mut c = 0;

        return solve(data_path);

        for (line_nr, line) in reader::get_lines(data_path)?.enumerate() {
            let start_time = Instant::now();
            let parts = line.split(' ').collect::<Vec<&str>>();
            let (mut pattern_lookup, mut parts_iter) = (HashSet::new(), parts.iter());
            let mut processing_queue = VecDeque::from(vec![(Joltage::default(), 0)]);
            let block_count = parts_iter.len() - 2; // -1 for first and -1 for last elements.

            parts_iter.next();
            let mut blocks = Vec::new();
            for _ in 0..block_count {
                blocks.push(Joltage::from_button(
                    parts_iter.next().expect("This should never fail since we ensure to only call next the correct amount of times."),
                )?);
            }

            patterns(blocks.clone());
            todo!();

            let desired_pattern = Joltage::from_joltage_pattern(
                parts_iter.next().expect("This should never fail since we ensure to only call next the correct amount of times."),
            )?;
            //let desired_joltage_sum = desired_pattern.lights.iter().sum();

            for i in 0..blocks.len() {
                let counters = Joltage::default();
            }

            let mut queue = BinaryHeap::new();
            queue.push(Reverse(QueueGroup::default()));

            'outer: while let Some(group) = queue.pop() {
                for new_lights in blocks.iter().map(|b| group.0.jolatage.combine(b)) {
                    if pattern_lookup.insert(new_lights) && new_lights.can_reach(&desired_pattern) {
                        c += 1;
                        if new_lights == desired_pattern {
                            total_steps += 1 + group.0.steps;
                            //return Ok(total_steps);
                            break 'outer;
                        }

                        //println!("Light: {new_lights:?}");
                        let joltage_sum = new_lights.lights.iter().sum(); /*
                                                                          let heuristic = new_lights.heuristic(&desired_pattern)
                                                                              + (group.0.steps as u16 / 2)
                                                                              + 1
                                                                              + (desired_pattern.difference(&new_lights) as u16 / 1); // */

                        let heuristic =
                            desired_pattern.difference(&new_lights) as u16 + group.0.steps as u16;
                        // */
                        let heuristic = desired_pattern.difference(&new_lights) as u16;
                        let cost = joltage_sum + heuristic + new_lights.heuristic(&desired_pattern)
                            + group.0.steps as u16 + 1
                            /*
                            + (heuristic as i16 - (group.0.steps as i16 + 1)).clamp(0, i16::MAX)
                                as u16 */
                                ;

                        let cost = desired_pattern.difference(&new_lights) as u16;

                        println!(
                            "estimated remaining cost: {}, current cost: {}, steps: {}, sum: {}",
                            heuristic, cost, group.0.steps, joltage_sum
                        ); // */
                           //println!("{:?}", new_lights);

                        //thread::sleep(Duration::from_millis(10));

                        queue.push(Reverse(QueueGroup {
                            jolatage: new_lights,
                            cost,
                            steps: group.0.steps + 1,
                            joltage_increases: joltage_sum,
                        })); // */

                    /*
                    queue.push(QueueGroup {
                        jolatage: new_lights,
                        heuristic: heuristic(
                            &new_lights.lights,
                            &desired_pattern.lights,
                            &blocks,
                        ),
                        steps: group.steps + 1,
                    }); // */
                    } else {
                        //println!("Skipped {:?}", new_lights);
                    }
                }
            }

            /*
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
            }*/

            println!("Completed line {}. Total Steps: {}", line_nr, total_steps);
            println!(
                "In {:?} time having searched {} combinations.",
                Instant::now() - start_time,
                c
            );
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
