use std::u32;

advent_of_code::solution!(10);

/// A key thing to note for this problem is that if you press any button twice, then it's exactly the same as if that button hadnt been pressed at all.
/// This means you dont need to consider pressing buttons multiple times. You only need to consider pressing each button once.
/// In other words you can just check all the different combos of buttons to see if they satisfy the light diagram, and then choose the combo with the least number of buttons pushed.
pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse(input);

    let mut total_presses = 0;
    for machine in machines {
        let result =  match solve_with_fewest_presses(machine.indicator_light_diagram, machine.wiring_schematic) {
            Some(v) => v,
            None => 0,
        };

        println!("Result for {}, {}", machine.indicator_light_diagram, result);

        total_presses += result;
    }

    Some(total_presses as u64)
}

fn parse_indicator_light_diagram(input: &str) -> usize {
    input
        .trim_matches(|c| c == '[' || c == ']')
        .chars()
        .enumerate()
        .fold(0, |mut acc, (i, x)| {

            match x {
                '#' => acc += 1 << i,
                '.' => {}
                _ => {
                    print!("Unknown input {} found in {}", x, input)
                }
            }

            acc

        })
}

fn parse_wiring_schematic(input: &str) -> Vec<usize> {
    input
        .trim_matches(|c| c == '(' || c == ')')
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
}

fn parse_joltage_requirements(input: &str) -> Vec<usize> {
    vec![]
}

struct Machine {
    indicator_light_diagram: usize,
    wiring_schematic: Vec<Vec<usize>>,
    jolt_requirements: Vec<usize>,

}

fn parse(input: &str) -> Vec<Machine> {
    // TODO: Improve this logic
    let parts: Vec<Vec<&str>> = input.lines().map(|l| l.split(" ").collect()).collect();

    let mut machines = vec![];

    for l in parts {
        let mut indicator_light_diagram = 0;
        let mut wiring_schematic = vec![];
        let mut jolt_requirements = vec![];

        for s in l {
            match s.chars().nth(0).unwrap() {
                '[' => indicator_light_diagram = parse_indicator_light_diagram(s),
                '(' => wiring_schematic.push(parse_wiring_schematic(s)),
                '{' => jolt_requirements = parse_joltage_requirements(s),
                _ => panic!("Unknown character in input {}", s)
            }
        }

        let machine = Machine { indicator_light_diagram, wiring_schematic, jolt_requirements};

        machines.push(machine);
    }

    machines
}

fn solve_with_fewest_presses(target: usize, buttons: Vec<Vec<usize>>) -> Option<u32> {
    println!("target: {}", target);
    let n = buttons.len();

    let mut result = u32::MAX;
    for mask in 1..(1 << n) {
        let combo: Vec<u32> = (0..n).map(|i| ((mask >> i) & 1) as u32).collect();

        let mut candidate = 0;
        for (b, &c) in buttons.iter().zip(&combo) {
            if c == 0 {
                continue;
            }

            for press in b {
                candidate ^= (1 << press);
            }
        }

        println!("{:?}, {}", combo, candidate);

        if candidate == target {
            let presses = combo.iter().sum();

            println!("found result {}, presses {}", candidate, presses);
            result = result.min(presses);
        }
    }

    if result == u32::MAX {
        return None;
    } else {
        return Some(result);
    }

}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;


    #[test]
    fn test_parse_light_indicator() {
        let input = "[.###.#]";
        let result = parse_indicator_light_diagram(input);

        assert_eq!(result, 46);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
