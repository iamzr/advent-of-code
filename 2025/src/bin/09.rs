advent_of_code::solution!(9);

type Coordinate = (i64, i64);

fn find_largest_rectangle(p1: &Coordinate, p2: &Coordinate) -> u64 {
    if p1 == p2 {
        return 0;
    }

    let delta_1 = (p2.1 - p1.1).abs() + 1;
    if p1.0 == p2.0 {
        return delta_1 as u64;
    }

    let delta_0 = (p2.0 - p1.0).abs() + 1;
    if p1.1 == p2.1 {
        return delta_0 as u64;
    }

    return (delta_0 * delta_1) as u64;
}

fn parse(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .map(|l| {
            l.split_once(",")
                .map(|(x, y)| {
                    let x = x.parse::<i64>().unwrap();
                    let y = y.parse::<i64>().unwrap();

                    return (x, y);
                })
                .unwrap()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse(input);

    let mut max = 0;
    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i + 1) {
            let area = find_largest_rectangle(p1, p2);

            max = max.max(area)
        }
    }

    return Some(max);
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
