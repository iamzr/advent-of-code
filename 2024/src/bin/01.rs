use counter::Counter;
use std::collections::HashSet;

advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let values: Vec<_> = input.split_whitespace().collect();

    let s1: Vec<i32> = values
        .iter()
        .step_by(2)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let s2: Vec<i32> = values
        .iter()
        .skip(1)
        .step_by(2)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    (s1, s2)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (mut s1, mut s2) = parse(input);

    s1.sort();
    s2.sort();

    let diff: i32 = s1.into_iter().zip(s2).map(|(x, y)| (y - x).abs()).sum();

    Some(diff)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (mut s1, mut s2) = parse(input);

    let counter = s2.iter().collect::<Counter<_>>();

    println!("{:?}", counter);

    let unique_values: HashSet<&i32> = s1.iter().collect::<HashSet<_>>();

    println!("{:?}", unique_values);

    // let keys: HashSet<&i32> = counter.keys().cloned().collect::<HashSet<_>>();

    // let intersection = keys.intersection(&unique_values);

    let mut ans = 0;

    for value in s1 {
        ans += value * counter[&value] as i32
    }

    println!("{:?}", ans);

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
