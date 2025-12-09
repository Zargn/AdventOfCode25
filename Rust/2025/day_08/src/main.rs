use core::f32;
use std::{cmp::Ordering, collections::BinaryHeap, error::Error, usize};

mod reader;

#[allow(dead_code)]
const PART_ONE_EXPECTED_TEST_VALUE: u64 = 40;
#[allow(dead_code)]
const PART_TWO_EXPECTED_TEST_VALUE: u64 = 25272;

/*
Part One
##################################################################################################

Now things are getting more complicated!

We have a set of 3D coordinates, and the goal is to figure out which coordinates are the closest
to each other. Then connect them together.

Figuring out which ones are closest is the easy part, just iterate through the list calculating
the difference between the coordinates. Then connect to the one which is the shortest distance
away.

The larger question is how do we handle the connections? Multiple points can be connected at the
same time as a network. And networks might end up getting connected to other networks.

One way would be to have a list of "networks". The index a network is at in the list is the ID of
the network. Then the network itself holds a list of connected coordinates. Is see two ways to
handle when two networks gets connected. Either you make the network have a list of connected
networks, or you merge the two networks into one.

We also need to have some way to tell if a coordinate is connected to a network. This could be
done using network ids with a hashmap, or stored together with the coordinate in the coordinate
list.

If we go for merging networks we would have to move all coordinates from the smaller network to
the larger one. While also updating each coordinates connected network id.

When saving the different points in the network we don't need to use the actual position. It is
better to simply use the index that point has in the coordinate list.

And I have missunderstood the entire task...

I am not supposed to chech which other junctionbox is the closes to each junctionbox.
I am supposed to find which x amount of junctionbox pairs are the closest together.
That means some junctionboxes wont be connected to anything, while some would be with multiple
others.

Time to restart.

97119 is too low
*/

#[derive(Clone, Copy, Debug)]
struct JunctionBox {
    x: f32,
    y: f32,
    z: f32,
    network_id: usize,
}

impl JunctionBox {
    fn parse(data_string: &str) -> Result<JunctionBox, Box<dyn Error>> {
        let mut parts = data_string.split(',');
        Ok(JunctionBox {
            x: parts.next().ok_or("Missing x coordinate.")?.parse()?,
            y: parts.next().ok_or("Missing y coordinate.")?.parse()?,
            z: parts.next().ok_or("Missing z coordinate.")?.parse()?,
            network_id: usize::max_value(),
        })
    }

    fn distance(&self, other: &JunctionBox) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    fn in_network(&self) -> bool {
        self.network_id != usize::max_value()
    }
}

fn merge_networks(
    networks_to_merge: (usize, usize),
    junction_boxes: &mut Vec<JunctionBox>,
    networks: &mut Vec<Vec<usize>>,
) -> usize {
    let (largest_network, smallest_network) = {
        if networks[networks_to_merge.0].len() >= networks[networks_to_merge.1].len() {
            (networks_to_merge.0, networks_to_merge.1)
        } else {
            (networks_to_merge.1, networks_to_merge.0)
        }
    };

    if networks_to_merge.0 == networks_to_merge.1 {
        return networks[largest_network].len();
    }

    for i in 0..networks[smallest_network].len() {
        let box_id = networks[smallest_network][i];
        junction_boxes[box_id].network_id = largest_network;
        networks[largest_network].push(box_id);
    }

    networks[smallest_network].clear(); // We don't delete the network since then we need to
                                        // account for later networks getting a new id.
    networks[largest_network].len()
}

#[derive(Clone, Copy)]
struct BoxPair {
    box_1_id: usize,
    box_2_id: usize,
    distance: f32,
}

impl Ord for BoxPair {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.distance as i64 * 100).cmp(&(other.distance as i64 * 100))
    }
}

impl PartialOrd for BoxPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BoxPair {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for BoxPair {}

fn calculate_part_one(data_path: &str, connections_to_make: usize) -> Result<u64, Box<dyn Error>> {
    let lines = reader::get_lines(data_path)?;
    let mut junction_boxes: Vec<JunctionBox> = Vec::new();
    for line in lines {
        junction_boxes.push(JunctionBox::parse(&line)?);
    }

    let mut networks: Vec<Vec<usize>> = Vec::new();

    let mut junction_box_pairs: BinaryHeap<BoxPair> = BinaryHeap::new();

    for box_id in 0..junction_boxes.len() {
        let junction_box = junction_boxes[box_id];
        for other_box_id in box_id + 1..junction_boxes.len() {
            let other_box = junction_boxes[other_box_id];
            junction_box_pairs.push(BoxPair {
                box_1_id: box_id,
                box_2_id: other_box_id,
                distance: junction_box.distance(&other_box),
            });
        }
    }

    for box_pair in junction_box_pairs
        .into_sorted_vec()
        .iter()
        .take(connections_to_make)
    {
        let (box_1_id, box_2_id, distance) =
            (box_pair.box_1_id, box_pair.box_2_id, box_pair.distance);
        let mut junction_box = junction_boxes[box_1_id];
        let mut other_junction_box = junction_boxes[box_2_id];
        if junction_box.in_network() && other_junction_box.in_network() {
            merge_networks(
                (junction_box.network_id, other_junction_box.network_id),
                &mut junction_boxes,
                &mut networks,
            );
        } else if junction_box.in_network() && !other_junction_box.in_network() {
            networks[junction_box.network_id].push(box_2_id);
            other_junction_box.network_id = junction_box.network_id;
            junction_boxes[box_2_id] = other_junction_box;
        } else if !junction_box.in_network() && other_junction_box.in_network() {
            networks[other_junction_box.network_id].push(box_1_id);
            junction_box.network_id = other_junction_box.network_id;
            junction_boxes[box_1_id] = junction_box;
        } else {
            networks.push(vec![box_1_id, box_2_id]);

            junction_box.network_id = networks.len() - 1;
            junction_boxes[box_1_id] = junction_box;
            other_junction_box.network_id = networks.len() - 1;
            junction_boxes[box_2_id] = other_junction_box;
        }
    }

    let mut network_sizes = networks.iter().map(|n| n.len()).collect::<Vec<usize>>();
    network_sizes.sort();
    let mut result = 1;
    for network_size in network_sizes.iter().rev().take(3) {
        if *network_size == 0 {
            break;
        }
        result *= network_size;
    }
    Ok(result as u64)
}

/*
Part Two
##################################################################################################

Copy the part 1 solution and make the following changes:

1: Update the merge networks function to return the size of the new network.
2: Remove the connections_to_make limit from the connection loop.
3: Keep running the loop until a merge networks call returns junction_boxes.len()
    Might have to Check when adding singular boxes to existing networks as well.
4: Return the product from multiplying the x values of the latest pair of boxes.

9003684864 is too low
*/

fn calculate_part_two(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines = reader::get_lines(data_path)?;
    let mut junction_boxes: Vec<JunctionBox> = Vec::new();
    for line in lines {
        junction_boxes.push(JunctionBox::parse(&line)?);
    }

    let mut networks: Vec<Vec<usize>> = Vec::new();

    let mut junction_box_pairs: BinaryHeap<BoxPair> = BinaryHeap::new();

    for box_id in 0..junction_boxes.len() {
        let junction_box = junction_boxes[box_id];
        for other_box_id in box_id + 1..junction_boxes.len() {
            let other_box = junction_boxes[other_box_id];
            junction_box_pairs.push(BoxPair {
                box_1_id: box_id,
                box_2_id: other_box_id,
                distance: junction_box.distance(&other_box),
            });
        }
    }

    let junction_box_pairs = junction_box_pairs.into_sorted_vec();
    let mut box_pairs_iter = junction_box_pairs.iter();
    let mut box_pair: &BoxPair = &BoxPair {
        box_1_id: 0,
        box_2_id: 0,
        distance: 0.0,
    };

    for _ in 0..junction_box_pairs.len() {
        box_pair = box_pairs_iter.next().expect("This will never fail since this will never run more times than there are items in the iterator");
        let (box_1_id, box_2_id, _) = (box_pair.box_1_id, box_pair.box_2_id, box_pair.distance);
        let mut junction_box = junction_boxes[box_1_id];
        let mut other_junction_box = junction_boxes[box_2_id];

        if junction_box.in_network() && other_junction_box.in_network() {
            if merge_networks(
                (junction_box.network_id, other_junction_box.network_id),
                &mut junction_boxes,
                &mut networks,
            ) == junction_boxes.len()
            {}
        } else if junction_box.in_network() && !other_junction_box.in_network() {
            networks[junction_box.network_id].push(box_2_id);
            other_junction_box.network_id = junction_box.network_id;
            junction_boxes[box_2_id] = other_junction_box;
        } else if !junction_box.in_network() && other_junction_box.in_network() {
            networks[other_junction_box.network_id].push(box_1_id);
            junction_box.network_id = other_junction_box.network_id;
            junction_boxes[box_1_id] = junction_box;
        } else {
            networks.push(vec![box_1_id, box_2_id]);

            junction_box.network_id = networks.len() - 1;
            junction_boxes[box_1_id] = junction_box;
            other_junction_box.network_id = networks.len() - 1;
            junction_boxes[box_2_id] = other_junction_box;
        }
        if networks[junction_boxes[box_1_id].network_id].len() == junction_boxes.len()
            || networks[junction_boxes[box_2_id].network_id].len() == junction_boxes.len()
        {
            break;
        }
    }

    let result =
        junction_boxes[box_pair.box_1_id].x as u64 * junction_boxes[box_pair.box_2_id].x as u64;
    Ok(result)
}

//

// Default controller code. Is the same between projects.
// ###############################################################################################

fn main() {
    println!("Part One Result: ");
    match calculate_part_one("data.txt", 1000) {
        Ok(value) => println!("{}", value),
        Err(err) => println!("Error: {}", err),
    }
    println!("\nPart Two Result: ");
    match calculate_part_two("data.txt") {
        Ok(value) => println!("{}", value),
        Err(err) => println!("Error: {}", err),
    }
}

#[test]
fn calculate_part_one_test() {
    let expected_value = PART_ONE_EXPECTED_TEST_VALUE;
    match calculate_part_one("testdata.txt", 10) {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Part One calculation completed successfully but the result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Part One Error:\n{}", err),
    }
}

#[test]
fn calculate_part_two_test() {
    let expected_value = PART_TWO_EXPECTED_TEST_VALUE;
    match calculate_part_two("testdata.txt") {
        Ok(value) => assert_eq!(
            value, expected_value,
            "Part Two calculation completed successfully but the result was wrong! Expected: {} but received: {}",
            expected_value, value
        ),
        Err(err) => panic!("Part Two Error:\n{}", err),
    }
}
