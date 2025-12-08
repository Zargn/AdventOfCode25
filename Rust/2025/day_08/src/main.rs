use std::error::Error;

mod reader;

#[allow(dead_code)]
const PART_ONE_EXPECTED_TEST_VALUE: u64 = 40;
#[allow(dead_code)]
const PART_TWO_EXPECTED_TEST_VALUE: u64 = 0;

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
*/

fn calculate_part_one(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines = reader::get_lines(data_path)?;

    Err("Not implemented!".into())
}

/*
Part Two
##################################################################################################

*/

fn calculate_part_two(data_path: &str) -> Result<u64, Box<dyn Error>> {
    let lines = reader::get_lines(data_path)?;

    Err("Not implemented!".into())
}

//

// Default controller code. Is the same between projects.
// ###############################################################################################

fn main() {
    println!("Part One Result: ");
    match calculate_part_one("data.txt") {
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
    match calculate_part_one("testdata.txt") {
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
