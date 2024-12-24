advent_of_code::solution!(11);

struct Stone {
    number: u64
}

impl Stone {
    fn blink(&self) -> Vec<Stone> {
        if self.number == 0 {
            return vec![Stone { number: 1}];
        }

        let s =  self.number.to_string();
        let l = s.len();
        if l % 2 == 0 {
            let left = s[.. l/2].parse::<u64>().expect("Failed to parse");
            let right= s[l/2..].parse::<u64>().expect("Failed to parse");
            
            vec![Stone {number: left}, Stone {number: right}]
        } else {
            vec![Stone {number: self.number * 2024}]
        }
    }
}

fn parse(input: &str) -> Vec<Stone> {
    input.split(" ").map(|n| Stone {number: n.parse::<u64>().expect("Failed to parse")} ).collect::<Vec<Stone>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut stones = parse(input);

    let blinks =25;

    for _ in (0..blinks) {
        let mut new_stones: Vec<Stone> = vec![];
        for stone in stones {
            let mut n = stone.blink();
            new_stones.append(&mut n);
        }

        stones = new_stones;
    }

    Some(stones.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
