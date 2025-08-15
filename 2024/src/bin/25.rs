advent_of_code::solution!(25);
use itertools;

#[derive(Clone)]
struct Schematic {
    pins: Vec<usize>,
}

enum Parts {
    Key(Schematic),
    Lock(Schematic),
}

fn parse(input: &str) -> (Vec<Schematic>, Vec<Schematic>) {
    let (keys, locks): (Vec<Schematic>, Vec<Schematic>) = input
        .split("\n\n")
        .map(|chunk| chunk.lines().map(|l| l.chars().collect()).collect())
        .map(|chunk: Vec<Vec<char>>| {
            let pins: Vec<usize> = (0..chunk[0].len())
                .map(|c| chunk.iter().filter(|row| row[c] == '#').count())
                .collect();

            if chunk[0].contains(&'.') {
                Parts::Key(Schematic { pins: pins })
            } else {
                Parts::Lock(Schematic { pins: pins })
            }
        })
        .fold((vec![], vec![]), |(mut ks, mut ls), item| {
            match item {
                Parts::Key(k) => ks.push(k),
                Parts::Lock(l) => ls.push(l),
            };
            (ks, ls)
        });

    return (keys, locks);
}

fn find_combos(keys: Vec<Schematic>, locks: Vec<Schematic>) -> u32 {
    let mut result = 0;
    for (k, l) in itertools::iproduct!(keys, locks) {
        let complete_pins = k
            .pins
            .iter()
            .zip(l.pins.iter())
            .filter(|(&kp, &lp)| (kp + lp <= 7))
            .count();

        if complete_pins == 5 {
            result += 1;
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let (keys, locks) = parse(input);
    let combos = find_combos(keys, locks);

    Some(combos)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (keys, locks) = parse(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(keys.len(), 3);
        assert_eq!(locks.len(), 2);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
