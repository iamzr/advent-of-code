use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let parts: Vec<_> = re
        .captures_iter(&input)
        .map(|cap| {
            println!("{:?}", &cap);

            cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap()
        })
        .collect();

    println!("{:?}", parts);

    let sum: i32 = parts.iter().sum();

    println!("{:?}", sum);

    Some(sum)
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
