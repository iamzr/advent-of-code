advent_of_code::solution!(2);

fn is_valid_id(id: u64) -> bool {
    let s = id.to_string();
    let n = s.len();

    if n % 2 != 0 {
        return true;
    }

    let half = n / 2;

    &s[..half] != &s[half..]
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .split(",")
        .filter_map(|s| {
            let r = s.split_once("-").unwrap();

            let start = (r.0.parse::<u64>().unwrap(), r.1.parse::<u64>().unwrap());

            Some(start.0..start.1 + 1)
        })
        .flatten()
        .fold(0, |mut acc, id| {
            if !is_valid_id(id) {
                acc += id;
            }
            acc
        });

    Some(result)
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
