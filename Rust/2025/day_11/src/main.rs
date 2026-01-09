mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 5;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 615;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 2;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

We have a list where each line contains one main id and x amount of other id's that is points to.
Id's are made out of three lowercase letters.

Out task is to figure out how many unique paths exists that lead from the id "you" to id "out"

At this time I do not know for sure if any infinite loops exists, but it is very likely. So some
kind of safeguard against it will probably be needed.

One way is to make a pathfinder that goes from you to out, adding to a counter for each path
found.
However I think another more interesting way to do it is to go backwards. This could be done by:

Use a hashmap with ids as keys and store a list of ids that connect to that one together with a
integer representing the possible paths from that id to out. Default is 0 except for out which
must start with 1.

With a recursive function start at id out:
Get the connected ids and paths to out for the current id.
Using a recursive function iterate through all ids connected to the current one.
    if the id has been visited before in this path then continue with the next id.
    Add the paths to out value to the paths to out value of the id.
    Call this function on that id.

Once the recursive function finishes then check the paths to out value of "you" and you should
have your answer.


To ensure no loops occur we can use a shared hashset with ids as keys.
At the start of the recursive function add the current id to the hashset.
And at the end remove it again.
To check if a id has been visited before simply check if the value exists in the hashset.
    (To decrease the amount of uses of the hashset the addition of ids to the hashset could be
     combined with the visited check)

Update:
I found that the reverse search was kind of pointless. It did work for tests, but the full data
took ages so I went back to searching the correct direction.

A number of optimizations where also added to try and get past the ages long calculation, which
works just as well for the normal search too. Although I am usure if they are actually needed
now.
*/
mod part_one {
    use crate::reader;
    use std::{
        collections::{HashMap, HashSet},
        error::Error,
    };

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;
        let mut connections: HashMap<String, (Vec<String>, u64, bool, bool)> = HashMap::new();

        for line in lines {
            let mut s = line.split(": ");
            let source = s.next().ok_or("E1: Invalid data format!")?;
            connections.entry(source.to_string()).or_default();
            for o in s.next().ok_or("")?.split(" ") {
                connections
                    .entry(source.to_string())
                    .or_default()
                    .0
                    .push(o.to_string());
            }
        }

        connections.entry("you".to_string()).or_default().1 = 1;
        connections.entry("out".to_string()).or_default();

        let mut path_trace: HashSet<String> = HashSet::new();
        // I don't think we need to add "out" since it should not point to any other id, meaning
        // it should never be possible to need to check if it has been visited.

        solver("you".to_string(), &mut path_trace, &mut connections, 0)?;

        Ok(connections.entry("out".to_string()).or_default().1)
    }

    fn solver(
        current_id: String,
        path_trace: &mut HashSet<String>,
        connections: &mut HashMap<String, (Vec<String>, u64, bool, bool)>,
        last_cost: u64,
    ) -> Result<bool, Box<dyn Error>> {
        let (connected_ids, path_count, visited, dead_end) = connections
            .get(&current_id)
            .ok_or("E2: Current id does not exist in the connections hashmap!")?
            .clone();

        if dead_end {
            //return Ok(false);
        }

        let mut dead_end = true;

        for connected_id in connected_ids {
            if path_trace.insert(connected_id.to_string()) && current_id != "out" {
                let connected_old_cost = connections
                    .get(&connected_id)
                    .ok_or(format!(
                        "E5: connected_id [{}] does not exist in the connections hashmap!",
                        connected_id
                    ))?
                    .1;
                if !visited {
                    connections
                        .get_mut(&connected_id)
                        .ok_or(format!(
                            "E3: connected_id [{}] does not exist in the connections hashmap!",
                            connected_id
                        ))?
                        .1 += path_count;
                } else {
                    connections
                        .get_mut(&connected_id)
                        .ok_or(format!(
                            "E4: connected_id [{}] does not exist in the connections hashmap!",
                            connected_id
                        ))?
                        .1 += path_count - last_cost;
                }

                if solver(connected_id, path_trace, connections, connected_old_cost)? {
                    dead_end = false;
                }
            }
        }
        connections
            .get_mut(&current_id)
            .ok_or("E6: current_id does not exist in the connections hashmap!")?
            .2 = true;
        path_trace.remove(&current_id);
        Ok(dead_end)
    }
}

//

//

/*
Part Two
##################################################################################################

We now need to go from "svr" to "out", but only count the paths that pass both "dac" and "fft".

This should actually be fairly easy with the Part One code if we just modify the start point, and
then any time "out" is reached we simply check if the visited hashset contains both "dac" and
"fft". If it does then add the path count to the end.
However this causes potential issues since a valid path might combine with a non valid one. In
those cases we need to ensure we only count the valid paths and ignore the invalid ones.

My first thought is to use the dead end boolean. If a path is a dead end we should "clean up" the
path count until we reach a path that is valid. Basically, subtract the invalid path cost from
the existing path cost.

Edit:
I just realised I probably overcomplicated both part one and two...
I try to cache the path counts to decrease the amount of work needed, but I still iterate through
every possible path. Meaning I might as well just not cache anything and instead add one to a
counter each time "out" is reached... :/
*/
mod part_two {
    use crate::reader;
    use std::{
        collections::{HashMap, HashSet},
        error::Error,
    };

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;
        let mut connections: HashMap<String, (Vec<String>, u64, bool, bool)> = HashMap::new();

        for line in lines {
            let mut s = line.split(": ");
            let source = s.next().ok_or("E1: Invalid data format!")?;
            connections.entry(source.to_string()).or_default();
            for o in s.next().ok_or("")?.split(" ") {
                connections
                    .entry(source.to_string())
                    .or_default()
                    .0
                    .push(o.to_string());
            }
        }

        connections.entry("svr".to_string()).or_default().1 = 1;
        connections.entry("out".to_string()).or_default();

        let mut path_trace: HashSet<String> = HashSet::new();
        // I don't think we need to add "out" since it should not point to any other id, meaning
        // it should never be possible to need to check if it has been visited.

        solver("svr".to_string(), &mut path_trace, &mut connections, 0)?;

        Ok(connections.entry("out".to_string()).or_default().1)
    }

    fn solver(
        current_id: String,
        path_trace: &mut HashSet<String>,
        connections: &mut HashMap<String, (Vec<String>, u64, bool, bool)>,
        last_cost: u64,
    ) -> Result<bool, Box<dyn Error>> {
        let (connected_ids, path_count, visited, dead_end) = connections
            .get(&current_id)
            .ok_or("E2: Current id does not exist in the connections hashmap!")?
            .clone();

        if dead_end {
            return Ok(false);
        }

        let mut dead_end = true;

        for connected_id in connected_ids {
            if (path_trace.insert(connected_id.to_string()) && current_id != "out")
                && (connected_id != "out"
                    || (connected_id == "out"
                        && path_trace.contains("dac")
                        && path_trace.contains("fft")))
            {
                let connected_old_cost = connections
                    .get(&connected_id)
                    .ok_or(format!(
                        "E5: connected_id [{}] does not exist in the connections hashmap!",
                        connected_id
                    ))?
                    .1;
                if !visited {
                    connections
                        .get_mut(&connected_id)
                        .ok_or(format!(
                            "E3: connected_id [{}] does not exist in the connections hashmap!",
                            connected_id
                        ))?
                        .1 += path_count;
                } else {
                    connections
                        .get_mut(&connected_id)
                        .ok_or(format!(
                            "E4: connected_id [{}] does not exist in the connections hashmap!",
                            connected_id
                        ))?
                        .1 += path_count - last_cost;
                }

                if solver(connected_id, path_trace, connections, connected_old_cost)? {
                    dead_end = false;
                }
            }
        }
        connections
            .get_mut(&current_id)
            .ok_or("E6: current_id does not exist in the connections hashmap!")?
            .2 = true;
        path_trace.remove(&current_id);
        Ok(dead_end)
    }
}

//

//

// Default controller code. Is the same between projects.
// ###############################################################################################

fn main() {
    print!("Running Program...\n\nPart One ");
    match part_one::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}", value),
        Err(err) => println!("FAILED with error:\n{}", err),
    }
    print!("\nPart Two ");
    match part_two::calculate("data.txt") {
        Ok(value) => println!("Result:\n{}\n", value),
        Err(err) => println!("FAILED with error:\n{}\n", err),
    }
}
