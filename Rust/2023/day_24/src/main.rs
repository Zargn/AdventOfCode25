#[macro_use]
mod macros;
mod reader;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const PART_ONE_EXPECTED_TEST_VALUE: u64 = 2;
#[allow(dead_code)]
pub const PART_ONE_EXPECTED_VALUE: u64 = 0;

#[allow(dead_code)]
pub const PART_TWO_EXPECTED_TEST_VALUE: u64 = 47;
#[allow(dead_code)]
pub const PART_TWO_EXPECTED_VALUE: u64 = 0;

//

//

/*
Part One
##################################################################################################

We have a list where each row contains the starting position and velocity of a particle in 3D
space. Our goal is to figure out how many of the particles paths will intersect within a specific
area. For part one we are meant to only use the x and y axis, ignoring z.

I am thinking the way to do this is just to go through all the particles, calculating if and where
their paths meet. Then if said point is inside the test area simply add 1 to a counter.

Once all particles has been checked we should have our answer.

Note: The test area differs between the example and full data. 7 to 27 for the example, and
200000000000000 to 400000000000000 for the full data.
*/
mod part_one {
    use crate::reader;
    use std::{collections::btree_set::Intersection, error::Error};

    #[derive(Default, Debug, PartialEq)]
    struct Vector3D {
        x: f64,
        y: f64,
        z: f64,
    }

    impl Vector3D {
        fn new(x: f64, y: f64, z: f64) -> Vector3D {
            Self { x, y, z }
        }

        fn is_past(&self, target: f64) -> bool {
            self.x > target || self.y > target // || self.z > target
        }
        fn is_before(&self, target: f64) -> bool {
            self.x < target || self.y < target // || self.z < target
        }
    }

    #[derive(Default, Debug)]
    struct Particle {
        position: Vector3D,
        velocity: Vector3D,
    }

    impl Particle {
        fn parse(particle_str: &str) -> Result<Particle, Box<dyn Error>> {
            let values = particle_str
                .split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<f64>())
                .collect::<Result<Vec<_>, _>>()?;
            if values.len() != 6 {
                return Err(format!(
                    "Line: [{}] contained {} values instead of the expected 6!",
                    particle_str,
                    values.len()
                )
                .into());
            }
            Ok(Particle {
                position: Vector3D::new(values[0], values[1], values[2]),
                velocity: Vector3D::new(values[3], values[4], values[5]),
            })
        }

        fn intersection_with(&self, other: &Particle) -> Result<Vector3D, bool> {
            //println!("\nParticle A: {:?}", self);
            //println!("Particle B: {:?}", other);
            let determinant =
                (self.velocity.x * other.velocity.y) - (self.velocity.y * other.velocity.x);
            if determinant == 0.0 {
                //println!("Is parallel!");
                return Err(true);
            }

            let scaling_factor = ((other.position.x - self.position.x) * other.velocity.y
                - (other.position.y - self.position.y) * other.velocity.x)
                / determinant;

            //println!("scaling_factor: {scaling_factor}");

            let x = self.position.x + self.velocity.x * scaling_factor;
            let y = self.position.y + self.velocity.y * scaling_factor;

            //println!("point: x:{}, y:{}", x, y);

            if scaling_factor < 1.0 || negative_scaling_factor(other, self) {
                //println!("scaling_factor is less than 0!");
                return Err(false);
            }

            Ok(Vector3D::new(x, y, 0.0))
        }
    }

    fn negative_scaling_factor(a: &Particle, other: &Particle) -> bool {
        let determinant = (a.velocity.x * other.velocity.y) - (a.velocity.y * other.velocity.x);
        if determinant == 0.0 {
            return false;
        }

        let scaling_factor = ((other.position.x - a.position.x) * other.velocity.y
            - (other.position.y - a.position.y) * other.velocity.x)
            / determinant;
        scaling_factor < 1.0
    }

    fn get_test_area() -> (f64, f64) {
        #[cfg(test)]
        let (lower, upper) = (7.0, 27.0);
        #[cfg(not(test))]
        let (lower, upper) = (200000000000000.0, 400000000000000.0);
        (lower, upper)
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let particles = reader::get_lines(data_path)?
            .map(|line| Particle::parse(&line))
            .collect::<Result<Vec<_>, _>>()?;

        let mut intersections = Vec::new();

        for i in 0..particles.len() {
            //let mut parallel_count = 0;
            for a in i + 1..particles.len() {
                if let Ok(intersection) = particles[i].intersection_with(&particles[a]) {
                    intersections.push(intersection);
                }
                /*
                match particles[i].intersection_with(&particles[a]) {
                    Ok(intersection) => intersections.push(intersection),
                    Err(is_parallel) => {
                        if is_parallel {
                            parallel_count += 1;
                        }
                    }
                } // */
            }
            //println!("parallel_count: {parallel_count}");
        }

        let (lower_test_area, upper_test_area) = get_test_area();

        let mut result = 0;
        for intersection in intersections {
            //println!("Intersection: {:?}", intersection);
            if !intersection.is_before(lower_test_area) && !intersection.is_past(upper_test_area) {
                //println!("Intersection: {:?} Is inside the area!", intersection);
                result += 1;
            }
        }

        //Err("Error wooooooo!".into())
        Ok(result)
    }
}

//

//

/*
Part Two
##################################################################################################

Part two complicates things a lot.
Now we will use the z axis as well.

This time the goal is to figure out a start position and velocity of a new particle that will
COLLIDE with each and every other particle. This means that the particle lines doesn't just need
to intersect, but also match up time wise as well.

Now, the interesting part is, I don't think that is actually important at first.
Regardless of when the new particle collides with the others, they are all bound by their contant
velocity. Meaning they are always travelling in a line. And what we need is to create a new line
that intersects with ALL the other lines.
But it gets better. Since we know a answer exists, we know that there HAS to be ONE line that
intersects all of them. The full data has over 200 particles. Which means there are a lot of
lines to check.
But we don't actually need to check them all.
With two lines the possible intersecting new line can be anywhere along one plane.
But with three lines that is lowered to only ONE line that successfully intersects all three
lines.

So lets pick out 3 of the particles from the data that are not travelling in parallel.
We can treat said particle lines as infinite lines since we know the intersection point has to
be somewhere in the correct direction.
We need two math formulas. One to figure out IF two 3d lines intersect. And one to figure out
the closest distance between two 3d lines.
Once we do we can find the correct line by doing this:

for i in 0...
    let startpoint = line1.position + i * line1.velocity
    let lowest_distance = MAX_int.
    for i2 in 0...
        let endpoint = line2.position + i2 * line.velocity
        let newline.velocity = endpoint - startpoint.
        let newline.posiion = startpoint.position
        if newline intersects with line3
            return newline
        else
            let distance = newline closest distance to line3
            if distance > lowest_distance
                break
            else
                lowest_distance = distance

Once the correct line has been found we need to figure out the correct start position.
To do this we need to find the intersection point with all the remaining lines.
Then find the lines with the intersection point that has the two fewest time steps.
Then get the vector between the two intersection points of said lines.
We then need to find the total steps between the two intersection points.
Finally divide the vector between them by the amount of steps meant to be there to get the
velocity of the new particle.
To get the result subtract the velocity * the amount of time steps to the first intersection
point from said intersection point. The position we get will be the start position of our new
particle.

Update:
I started implementing these plans, but noticed an issue. I heavily underestimated the size
of the values.

I have tried a bunch of things but it seems like we might have to do something else. The
values are just too big.

We should try and analyze the data and make sure it isn't some kind of trick like day 12 2025.
Start by checking the intersecting point of all the lines. We don't expect them to hit each
other, but we should check just in case.
I have a feeling the point we are looking for might be the start point of one of the particles
in the data. Or at least along the path of one.
Of course, that is just a feeling, and nothing points to it being true.

All I know is that this problem seems way too difficult to be an AOC puzzle. Making me think
there might be a hidden criteria the data follows that isn't visible in the example.
Day 12 2025 was exactly that way. The puzzle and example made it appear as a extremely hard
problem to solve, but when checking the actual data we got it was clear that the edge cases
that brought that problem from "easy" to extremely hard never actually occured in the data.
Meaning the puzzle and example showed the extremely hard problem, but only the easy solution
was needed to solve it.
*/
mod part_two {
    use crate::reader;
    use std::{
        cmp::min,
        error::Error,
        ops::{Add, Mul, Sub},
    };

    #[derive(Default, Debug, PartialEq, Clone, Copy)]
    struct F64Vector3D {
        x: f64,
        y: f64,
        z: f64,
    }
    impl F64Vector3D {
        fn dot(&self, other: &F64Vector3D) -> f64 {
            let dot = self.x * other.x + self.y * other.y + self.z * other.z;
            //println!("dot: {}", dot);
            dot
        }
    }
    impl Add<F64Vector3D> for F64Vector3D {
        type Output = F64Vector3D;
        fn add(self, rhs: F64Vector3D) -> Self::Output {
            F64Vector3D {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }
    impl Sub<F64Vector3D> for F64Vector3D {
        type Output = F64Vector3D;
        fn sub(self, rhs: F64Vector3D) -> Self::Output {
            F64Vector3D {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
            }
        }
    }
    impl Mul<f64> for F64Vector3D {
        type Output = F64Vector3D;
        fn mul(self, rhs: f64) -> Self::Output {
            F64Vector3D {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }

    #[derive(Default, Debug, PartialEq, Clone, Copy)]
    struct Vector3D {
        x: i128,
        y: i128,
        z: i128,
    }

    impl Vector3D {
        fn new(x: i128, y: i128, z: i128) -> Vector3D {
            Self { x, y, z }
        }

        fn dot(&self, other: &Vector3D) -> i128 {
            let dot = self.x * other.x + self.y * other.y + self.z * other.z;
            //println!("dot: {}", dot);
            dot
        }

        fn to_f64_vector(&self) -> F64Vector3D {
            F64Vector3D {
                x: self.x as f64,
                y: self.y as f64,
                z: self.z as f64,
            }
        }

        fn cross(&self, other: &Vector3D) -> Vector3D {
            todo!();
        }
    }

    impl Add<Vector3D> for Vector3D {
        type Output = Vector3D;
        fn add(self, rhs: Vector3D) -> Self::Output {
            Vector3D {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }
    impl Sub<Vector3D> for Vector3D {
        type Output = Vector3D;
        fn sub(self, rhs: Vector3D) -> Self::Output {
            Vector3D {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
            }
        }
    }
    impl Mul<i128> for Vector3D {
        type Output = Vector3D;
        fn mul(self, rhs: i128) -> Self::Output {
            Vector3D {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }
    impl Mul<f64> for Vector3D {
        type Output = Vector3D;
        fn mul(self, rhs: f64) -> Self::Output {
            Vector3D {
                x: (self.x as f64 * rhs) as i128,
                y: (self.y as f64 * rhs) as i128,
                z: (self.z as f64 * rhs) as i128,
            }
        }
    }

    #[derive(Default, Debug)]
    struct Particle {
        position: Vector3D,
        velocity: Vector3D,
    }

    impl Particle {
        fn new(position: Vector3D, velocity: Vector3D) -> Particle {
            Self { position, velocity }
        }
        fn parse(particle_str: &str) -> Result<Particle, Box<dyn Error>> {
            let values = particle_str
                .split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i128>())
                .collect::<Result<Vec<_>, _>>()?;
            if values.len() != 6 {
                return Err(format!(
                    "Line: [{}] contained {} values instead of the expected 6!",
                    particle_str,
                    values.len()
                )
                .into());
            }
            Ok(Particle {
                position: Vector3D::new(values[0], values[1], values[2]),
                velocity: Vector3D::new(values[3], values[4], values[5]),
            })
        }

        fn closest_distance_to(&self, other: &Particle) -> Option<f64> {
            let diff = (self.position - other.position).to_f64_vector();

            let p1 = self.position.to_f64_vector();
            let v1 = self.velocity.to_f64_vector();

            let p2 = other.position.to_f64_vector();
            let v2 = other.velocity.to_f64_vector();

            /*
            let a = self.velocity.dot(&self.velocity);
            let b = self.velocity.dot(&other.velocity);
            let c = other.velocity.dot(&other.velocity);
            let d = self.velocity.dot(&diff);
            let e = other.velocity.dot(&diff);
            // */

            let a = v1.dot(&v1);
            let b = v1.dot(&v2);
            let c = v2.dot(&v2);
            let d = v1.dot(&diff);
            let e = v2.dot(&diff);

            /*
            let denom = a * c - b * b;
            if denom == 0 {
                return None;
            }*/

            let denom = a * c - b * b;
            if 0.1 > denom && denom > -0.1 {
                return None;
            }

            let t = (b * e - c * d) as f64 / denom as f64;
            let s = (a * e - b * d) as f64 / denom as f64;
            //println!("s: {s}");

            /*
            let closest_a = self.position + self.velocity * t;
            let closest_b = other.position + other.velocity * s;
            */

            /*
            let r = diff + v1 * t - v2 * s;
            let distance = r.dot(&r).sqrt();
            Some(distance) // */

            let closest_a = p1 + (v1 * t);
            let closest_b = p2 + (v2 * s);

            let r = closest_a - closest_b;
            Some((r.x.powf(2.0) + r.y.powf(2.0) + r.z.powf(2.0)).sqrt())
            //Some(((r.x.pow(2) + r.y.pow(2) + r.z.pow(2)) as f64).sqrt()) // */
        }

        fn position_after(&self, time: i128) -> Vector3D {
            self.position + self.velocity * time
        }
    }

    /*

    for i in 0...
        let startpoint = line1.position + i * line1.velocity
        let lowest_distance = MAX_int.
        for i2 in 0...
            let endpoint = line2.position + i2 * line.velocity
            let newline.velocity = endpoint - startpoint.
            let newline.posiion = startpoint.position
            if newline intersects with line3
                return newline
            else
                let distance = newline closest distance to line3
                if distance > lowest_distance
                    break
                else
                    lowest_distance = distance

    */

    fn get_connecting_line(
        line1: &Particle,
        line2: &Particle,
        line3: &Particle,
    ) -> Result<Particle, Box<dyn Error>> {
        let mut i = 1;
        let mut c = f64::MAX;
        let mut old_i2 = -1;
        let mut prev_i = -1;
        let mut prev_c = c;
        loop {
            let startpoint = line1.position_after(i);
            let mut lowest_distance = f64::MAX;
            let mut lowest_distance_i2 = 0;
            let mut i2 = 0; //1273196186888;
                            //let mut i2 = 392415090000; //1273196186888;

            let mut orig_distance = -1.0;
            let mut prev_i2 = 0;

            let mut l_c = c;
            loop {
                let endpoint = line2.position_after(i2);
                let newline = Particle::new(startpoint, endpoint - startpoint);
                /*
                println!(
                    "Startpoint: {:?}\nEndpoint: {:?}\nDirection: {:?}",
                    startpoint, endpoint, newline.velocity
                ); // */

                if let Some(distance) = newline.closest_distance_to(line3) {
                    //println!("{}", distance);
                    if distance < l_c {
                        l_c = distance;
                        //println!("New shortest: {}", c);
                    } // */
                    if distance == 0.0 {
                        println!("Found connecting line!");
                        println!("newline pos: {:?}", newline.position);
                        println!("newline vel: {:?}", newline.velocity);
                        return Ok(newline);
                    } else if distance > lowest_distance {
                        println!(
                            "Passed closest point at a distance of: {}\nwith line1[{}] and line2[{}]\nC: {}",
                            lowest_distance, i, i2, c
                        );

                        //println!("newline pos: {:?}", newline.position);
                        //println!("newline vel: {:?}", newline.velocity);

                        /*
                        if c > 10000.0 {
                            //i += c as i128;
                            //default_i2 = (i2 as f64 * 0.50) as i128;
                            //i += 10;
                        } // */
                        if distance <= c {
                            old_i2 = lowest_distance_i2;
                            //c = distance;
                        }

                        break;
                    } else if orig_distance == -1.0 {
                        orig_distance = distance;
                    } else {
                        if old_i2 == -1 {
                            if distance > 10000.0 {
                                i2 += 100000;
                            } // */
                            i2 += 1;
                        } else {
                            let left_to_closest = ((distance - c) / (orig_distance - c));
                            //println!("Ltc: {left_to_closest} d: {distance} c: {c}");
                            let new_steps = ((old_i2 - prev_i2) as f64 * left_to_closest) / 10.0;
                            //println!("Ns: {new_steps}");

                            //let t = ((c - distance) * (1.0 - (i2 as f64 / prev_i2 as f64))) as i128;
                            //println!("t: {t}");
                            i2 += new_steps.max(1.0) as i128;
                        }

                        //i2 += 0 + (distance * 0.000001) as i128;
                        lowest_distance = distance;
                        lowest_distance_i2 = i2;
                        prev_i2 = i2;
                    }
                }
                i2 += 1;
            }

            prev_c = c;
            c = l_c;

            let prev_step_size = i - prev_i;

            let d_diff = prev_c - c;
            let multiplier = 1.0 - (d_diff as f64 / c as f64);

            i += 1;
        }
    }

    pub fn calculate(data_path: &str) -> Result<u64, Box<dyn Error>> {
        let particles = reader::get_lines(data_path)?
            .map(|line| Particle::parse(&line))
            .collect::<Result<Vec<_>, _>>()?;

        let mut link = Particle::new(
            particles[0].position,
            particles[0].position - particles[1].position,
        );

        let newline = Particle::new(
            Vector3D {
                x: 347900056116335,
                y: 153201919784951,
                z: 326681280941887,
            },
            Vector3D {
                x: -57589298833377,
                y: 174645336898139,
                z: 50955022631101,
            },
        );

        let newline = Particle::new(
            Vector3D {
                x: 347900052479899,
                y: 153201931689133,
                z: 326681276790861,
            },
            Vector3D {
                x: -57589295012981,
                y: 174645325363921,
                z: 50955027021275,
            },
        );

        /*
        let l1 = Particle::new(Vector3D::new(0, 0, 1), Vector3D::new(1, 0, 0));
        let l2 = Particle::new(Vector3D::new(0, 0, 0), Vector3D::new(0, 1, 0));

        if let Some(distance) = l1.closest_distance_to(&l2) {
            println!("Distance: {distance}");
        } // */

        //println!("{:?}", link);
        link.position.x += 1;

        //println!("{:?}", link);

        println!(
            "Line1: \n{:?}\nLine2: \n{:?}\nNewLine: \n{:?}\n",
            particles[0], particles[1], link
        );

        if let Some(distance) = link.closest_distance_to(&particles[1]) {
            println!("Distance: {distance}");
        }

        let mut c = f64::MAX;
        for i in 0..particles.len() {
            if let Some(distance) = newline.closest_distance_to(&particles[i]) {
                //println!("Distance: {distance}");
            }

            for i2 in i + 1..particles.len() {
                if let Some(distance) = particles[i].closest_distance_to(&particles[i2]) {
                    if distance < c {
                        //println!("New closest_distance is between line {} and line {} at a distance of {}", i, i2, distance);
                        c = distance;
                    }
                    /*
                    if distance < 10000000000.0 {
                        //println!("Line {i} passes line {i2} at a distance of {distance}");
                    }*/
                } else {
                    //println!("Line {i} and line {i2} is parallel!");
                }
            }
        } // */
        println!("Done!");

        let link = get_connecting_line(&particles[0], &particles[1], &particles[2])?;
        for i in 0..particles.len() {
            if let Some(distance) = link.closest_distance_to(&particles[i]) {
                if distance == 0.0 {
                    println!("Link - line{i}, Successful!");
                } else {
                    println!("Link - line{i}, Failed! {distance}");
                }
                //println!("Distance: {distance}");
            } else {
                println!("Link is parallel with line {i}");
            }
        }

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
