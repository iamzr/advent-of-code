use std::{fs, ops, str::FromStr};

fn main() {
    let file_path = "./test.txt";

    let contents = fs::read_to_string(file_path).unwrap();

    let values = contents.split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|row| row.parse::<Equation>().expect("Parsing issue"))
        .collect::<Vec<Equation>>();

    println!("{:?}", values);

    let ans = part_one(values);

    println!("Final answer: {}", ans);
}

#[derive(Debug, PartialEq, Eq)]
struct ParseEquationError;

#[derive(Debug)]
struct Equation {
    test: i64,
    operands: Vec<i64>
}

impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = s.split(":").collect::<Vec<&str>>();
        println!("{:?}", row);

        let test = row[0].parse::<i64>().map_err(|_| ParseEquationError)?;

        let operands= match row[1]
                    .split_whitespace()
                    .map(|num| Ok(num.parse::<i64>().map_err(|_| ParseEquationError)?))
                    .collect::<Result<Vec<i64>, ParseEquationError>>() {
                        Ok(v) => v,
                        Err(_e) => return Err(_e),
                    };

        Ok(Equation {
                test: test ,
                operands:operands,
        })

    }
}

fn part_one (equations: Vec<Equation>) -> i64 {
    let operations = vec![
        ops::Add::add,
        ops::Mul::mul,
        concat
    ];

    let mut ans = 0;

    for equation in equations {
        let v = find(equation.test, 0, 0, &equation.operands, &operations);
        println!("For target {} we found {}", equation.test, v);

        if v {
            ans += equation.test;
        }
    }

    ans
}

fn concat(a: i64, b: i64) -> i64 {
    a * 10_i64.pow(b.ilog10() + 1) + b
}

/// For a given target there's two things you can do
fn find(target: i64, curr: i64, idx: usize, l: &Vec<i64>, operations: &Vec<fn(i64, i64) -> i64>) -> bool {
    if curr > target {
        // println!("greater than target {}, curr {}", target, curr);
        return false
    }

    if curr == target {
        return true
    }


    if idx >= l.len() {
        // println!(" Got to end for target {} with curr {}", target, curr);
        return false
    }

    let n= l[idx];
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
    fn test_concating_op() {
        let n1 :i64 = 123;
        let n2 :i64 = 456;

        assert_eq!(123456, concat(n1, n2))
    }

}
