use std::collections::HashSet;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut prev = HashSet::new(); // holds the list of beams in the previous line

    let mut total_splits = 0;

    for line in input.lines() {
        let mut curr = prev.clone();
        let mut splits = 0;
        for (i, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    curr.insert(i);
                }
                '.' => {}
                '^' => {
                    if !prev.contains(&i) {
                        continue;
                    }

                    splits += 1;
                    curr.remove(&i);

                    match i.checked_sub(1) {
                        Some(v) => {
                            curr.insert(v);
                        }
                        None => {}
                    }
                    curr.insert(i + 1);
                }
                _ => panic!("Invalid character found"),
            }
        }

        total_splits += splits;

        prev = curr;
    }

    Some(total_splits as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
