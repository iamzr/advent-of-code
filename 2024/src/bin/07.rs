use std::{ops, str::FromStr};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<i64> {
    Some(solve(input, vec![ops::Add::add, ops::Mul::mul]))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(solve(input, vec![ops::Add::add, ops::Mul::mul, concat]))
}

fn solve(input: &str, operators: Vec<fn(i64, i64) -> i64>) -> i64 {
    let equations = input
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|row| row.parse::<Equation>().expect("Parsing issue"))
        .collect::<Vec<Equation>>();

    let mut ans = 0;

    for equation in equations {
        let v = find(equation.test, 0, 0, &equation.operands, &operators);
        println!("For target {} we found {}", equation.test, v);

        if v {
            ans += equation.test;
        }
    }

    ans
}

#[derive(Debug, PartialEq, Eq)]
struct ParseEquationError;

#[derive(Debug)]
struct Equation {
    test: i64,
    operands: Vec<i64>,
}

impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = s.split(":").collect::<Vec<&str>>();
        println!("{:?}", row);

        let test = row[0].parse::<i64>().map_err(|_| ParseEquationError)?;

        let operands = match row[1]
            .split_whitespace()
            .map(|num| Ok(num.parse::<i64>().map_err(|_| ParseEquationError)?))
            .collect::<Result<Vec<i64>, ParseEquationError>>()
        {
            Ok(v) => v,
            Err(_e) => return Err(_e),
        };

        Ok(Equation {
            test: test,
            operands: operands,
        })
    }
}

fn concat(a: i64, b: i64) -> i64 {
    a * 10_i64.pow(b.ilog10() + 1) + b
}

/// For a given target there's two things you can do
fn find(
    target: i64,
    curr: i64,
    idx: usize,
    l: &Vec<i64>,
    operations: &Vec<fn(i64, i64) -> i64>,
) -> bool {
    if curr > target {
        // println!("greater than target {}, curr {}", target, curr);
        return false;
    }

    if curr == target {
        return true;
    }

    if idx >= l.len() {
        // println!(" Got to end for target {} with curr {}", target, curr);
        return false;
    }

    let n = l[idx];
    // println!("{}", n);

    let mut res = false;

    for op in operations {
        res = res || find(target, op(curr, n), idx + 1, l, operations);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    mod op_tests {
        use super::*;
        #[test]
        fn test_concat() {
            let n1: i64 = 123;
            let n2: i64 = 456;

            assert_eq!(123456, concat(n1, n2))
        }
    }
}
