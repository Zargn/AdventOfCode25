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

Update again:
I still stand by what I said earlier, but some of the tricks are probably still good. Mainly the
dead end caching.

Update 3:
I have realised that the amount of paths that exists from "svr" to out is much much higher than
there are from "you". Meaning some optimization or rethinking is required.
At the moment I am thinking maybe it is a good idea to split the calculation into parts.
We only care about paths from svr to out that pass through "dac" and "fft". No matter which is
first. So, would it be benefitial to search in steps instead? First find how many paths exist
from "svr" to "dac" without passing out/fft, and then from "svr" to "fft" without passing
out/dac. Then find the amount of paths from dac to fft, and fft to dac.

Basically we want a recursive seach function that finds all paths from "start" to "goal" that
doesn't pass any ids that match those in a "blocked" list. The blocked list could be made by
simply adding the blocked ids to the path_trace HashSet before starting the function. That
would make it so each time any blocked id is reached it is already marked as part of the path,
and therefor can't be added again.

Then we just add the count of the two possible path sequences together.
svr->dac->fft->out and
svr->fft->dac->out

Update 4:
Once again I underestimated the amount of possible paths.
I think chaching results is a must, but the way I did it before is not good enough.
If we cache how many paths exist from each node we visit to the goal we should be able to
ensure we don't calculate any nodes twice.
If a we check a node and it has a path value, we return that value instead of continuing.

Results:
160 is too low
4643476320 is too low ??? What!? Quite understandable that the calculation took ages if there
are more than 4643476320 paths...
*/
mod part_two {
    use crate::{reader, PART_ONE_EXPECTED_VALUE};
    use std::{
        collections::{HashMap, HashSet},
        error::Error,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct DeviceId(u16);

    impl DeviceId {
        fn new(id_string: &str) -> Result<DeviceId, Box<dyn Error>> {
            let mut multiplier = 1;
            let mut result = 0;
            for c in id_string.chars().rev() {
                if !c.is_ascii_lowercase() {
                    return Err(format!(
                        "Invalid charcther [{}] in data string [{}]!",
                        c, id_string
                    )
                    .into());
                }
                result += (c as u8 - b'a') as u16 * multiplier;
                multiplier *= 26;
            }
            Ok(DeviceId(result))
        }
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let lines = reader::get_lines(data_path)?;
        let mut devices: HashMap<DeviceId, (Vec<DeviceId>, Option<u64>)> = HashMap::new();

        for line in lines {
            let mut parts = line.split(": ");
            let source = DeviceId::new(parts.next().ok_or("E1: Invalid data format!")?)?;
            let (connections, _) = devices.entry(source).or_default();
            for part in parts.next().ok_or("")?.split(" ") {
                connections.push(DeviceId::new(part)?);
            }
        }

        let svr = DeviceId::new("svr")?;
        let dac = DeviceId::new("dac")?;
        let fft = DeviceId::new("fft")?;
        let out = DeviceId::new("out")?;

        println!(
            "\nPart One task should return 615, we got: {}\n",
            solver(
                DeviceId::new("you")?,
                out,
                &mut HashSet::new(),
                &mut devices.clone()
            )?
        );

        assert_eq!(
            PART_ONE_EXPECTED_VALUE,
            count_paths(vec![DeviceId::new("you")?, out], &devices)?
        );

        let mut result = 0;

        result += count_paths(vec![svr, dac, fft, out], &devices)?;
        result += count_paths(vec![svr, fft, dac, out], &devices)?;

        println!(
            "Paths test: {}",
            count_paths(vec![DeviceId::new("you")?, out], &devices)?
        );

        Ok(result)
    }

    fn count_paths(
        mandatory_nodes_in_order: Vec<DeviceId>,
        devices: &HashMap<DeviceId, (Vec<DeviceId>, Option<u64>)>,
    ) -> Result<u64, Box<dyn Error>> {
        let mut paths = 1;
        for i in 0..mandatory_nodes_in_order.len() - 1 {
            let mut path_trace: HashSet<DeviceId> = HashSet::new();
            for node in mandatory_nodes_in_order.iter().take(i) {
                path_trace.insert(*node);
            }
            for node in mandatory_nodes_in_order.iter().skip(i + 2) {
                path_trace.insert(*node);
            }

            println!("path_trace: {:?}", path_trace);
            let r = solver(
                mandatory_nodes_in_order[i],
                mandatory_nodes_in_order[i + 1],
                &mut path_trace,
                &mut devices.clone(),
            )?;
            println!(
                "Found {} paths between {:?} and {:?}",
                r,
                mandatory_nodes_in_order[i],
                mandatory_nodes_in_order[i + 1]
            );
            paths *= r;
        }

        Ok(paths)
    }

    fn solver(
        current: DeviceId,
        goal: DeviceId,
        path_trace: &mut HashSet<DeviceId>,
        connections: &mut HashMap<DeviceId, (Vec<DeviceId>, Option<u64>)>,
    ) -> Result<u64, Box<dyn Error>> {
        let (connected_ids, paths_to_goal) = connections
            .get(&current)
            .ok_or(format!(
                "E2: [{:?}] does not exist in the connections hashmap!",
                current
            ))?
            .clone();

        if let Some(paths) = paths_to_goal {
            return Ok(paths);
        }

        let mut result = 0;
        for connected_id in connected_ids {
            if connected_id == goal {
                result += 1;
            } else if path_trace.insert(connected_id) {
                result += solver(connected_id, goal, path_trace, connections)?;
            }
        }

        connections
            .get_mut(&current)
            .ok_or(format!(
                "E3: [{:?}] does not exist in the connections hashmap!",
                current
            ))?
            .1 = Some(result);

        path_trace.remove(&current);

        Ok(result)
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
