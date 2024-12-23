advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let values = input.split("\n").collect::<Vec<&str>>();
    println!("{:?}", values);
    let reports: Vec<Vec<i32>> = values
        .iter()
        .map(|row| {
            row.split_whitespace()
                .map(|num| num.parse::<i32>().expect("Invalid Number"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    println!("{:?}", reports);

    let mut ans = vec![0; reports.len()];

    for (index, levels) in reports.into_iter().enumerate() {
        let mut prev = levels[0];

        let mut completed = true;

        let mut total_sign: i32 = 0;
        for curr in &levels[1..] {
            print!("{}, {}\n", prev, curr);
            let diff = curr - prev;
            let sign = match diff {
                n if n > 0 => 1,
                n if n < 0 => -1,
                _ => 0,
            };

            if sign == 0 {
                completed = false;
                println!("broke because not-increasing or decreasing");
                break;
            }

            total_sign += sign;

            if diff.abs() > 3 {
                completed = false;
                println!("broke because diff greater than 3");
                break;
            }
            prev = *curr;
        }

        println!("{:?} {:?}", total_sign, levels.len());
        if total_sign.abs() != levels.len() as i32 - 1 {
            println!("broke because sign");
            continue;
        }
        if completed {
            ans[index] = 1;
        }
    }

    println!("{:?}", ans);

    let result: i32 = ans.iter().sum();

    println!("{:?}", result);

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
