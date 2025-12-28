advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let result: u32 = input
        .lines()
        .map(|line| {
            let mut max = 0u32;
            let mut max_first = 0u32;

            for n in line.bytes().map(|b| (b - b'0') as u32) {
                let v = max_first * 10 + n;

                if v > max {
                    max = v;
                }
                if n > max_first {
                    max_first = n;
                }
            }

            max
        })
        .sum();

    Some(result as u64)
}

fn get_jolt<const WINDOW_SIZE: usize>(batteries: Vec<u64>) -> u64 {
    let mut stack: Vec<u64> = vec![];
    let mut to_remove = batteries.len() - WINDOW_SIZE;

    for battery in batteries {
        while to_remove > 0 {
            if let Some(&last) = stack.last()
                && last < battery
            {
                stack.pop();
                to_remove -= 1;
            } else {
                break;
            }
        }

        stack.push(battery);
    }

    stack.truncate(WINDOW_SIZE);

    let l = stack.len();
    if l != WINDOW_SIZE {
        panic!("Stack length is {} should be 12", l)
    }

    stack.iter().fold(0, |acc, &d| acc * 10 + d)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result: u64 = input
        .lines()
        .map(|line| {
            let batteries = line
                .bytes()
                .map(|b| (b - b'0') as u64)
                .collect::<Vec<u64>>();

            let v = get_jolt::<12>(batteries);
            v
        })
        .sum();

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
