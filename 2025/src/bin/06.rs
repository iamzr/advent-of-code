advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let l = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut global_result = 0;

    let len = l[0].len();

    for i in 0..len {
        let mut equation = l.iter().map(|x| x[i]).collect::<Vec<&str>>();
        let operation = equation.pop().unwrap();

        let equation = equation
            .iter()
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let local_result = match operation {
            "*" => equation.iter().copied().product::<u64>(),
            "+" => equation.iter().copied().sum::<u64>(),
            _ => panic!("Invalid operation"),
        };

        global_result += local_result;
    }

    return Some(global_result);
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
