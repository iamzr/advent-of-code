use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

fn parse(input:&str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse(input);
    let m = map.len();
    let n = map[0].len();

    println!("n: {}, m: {}", n, m);

    let mut d = HashMap::new();


    for (i, row) in map.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            // Add the position to the corresponding character's entry in the HashMap
            if ch == '.' {
                continue;
            }

            d.entry(ch).or_insert_with(Vec::new).push((i, j));
        }
    };
    println!("Number of nodes: {}", d.len());

    println!("List of antennas {:?}", d);

    let mut antinodes= HashSet::new();

    for (c, positions) in d {
        println!("Checking for {}", c);
        for p in positions.iter() {
            for q in positions.iter() {
                // println!("Considering {:?} {:?}", p, q );
                let (x,y) = match (q.0 as isize  - p.0 as isize, q.1 as isize - p.1 as isize) {
                    (0, 0) => {
                        // println!("continued");
                        continue;
                    }
                    (x, y) => {
                        (x,y)
                    }
                };

                // println!("with deltas {:?}", (x,y) );

                let antinode_1 = (p.0 as isize - x, p.1 as isize - y);
                // println!("Consider antinode 1: {:?}", antinode_1);
                if check_in_bounds(antinode_1, n, m) {
                        // println!("Antinode found at {:?}", antinode_1);
                        antinodes.insert(antinode_1);
                } else {
                    // println!("Antinode rejected")
                }
                

                let antinode_2 = (q.0 as isize + x, q.1 as isize + y);
                // println!("Consider antinode 2: {:?}", antinode_2);
                if check_in_bounds(antinode_2, n, m) {
                        // println!("Antinode found at {:?}", antinode_2);
                        antinodes.insert(antinode_2);
                } else {
                    // println!("Antinode rejected")
                }

            }
        }
    }



    println!("Antinodes: {:?}", antinodes);
    Some(antinodes.len())

}

fn check_in_bounds(point: (isize, isize), n: usize, m: usize) -> bool {
    (0 <= point.0) && (point.0 < (n as isize)) && ( point.1 >= 0)  && (point.1 < (m as isize)) 
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod test_checked_in_bounds {
        use super::*;

        #[test]
        fn test_1() {

            let cases = vec![
                ((2,5), true),
                ((0,0), true),
                ((11, 11), true),
                ((12, 12), false),
                ((23,44), false),
                ((-3,-2), false),
                ((-4, 6), false),
                ((5, -6), false)
            ];

            for (case, expected) in cases {
                let result = check_in_bounds(case, 12, 12);
                assert_eq!(result, expected, "Test failed for point {:?}, with in {} x {}", case, 12, 12);
            }

        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
