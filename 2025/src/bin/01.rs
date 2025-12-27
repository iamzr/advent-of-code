use std::str::FromStr;

advent_of_code::solution!(1);

type Number = i32;

struct Dial {
    current_value: Number,
}

impl Dial {
    fn new() -> Self {
        Dial { current_value: 50 }
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    distance: Number,
}

impl FromStr for Rotation {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (first, second) = input.split_at(1);

        let direction = match first {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err("Invalid direction"),
        };

        let distance: Number = second.parse().map_err(|_| "Invalid distance")?;

        Ok(Rotation {
            direction,
            distance,
        })
    }
}

impl Dial {
    fn rotate(&mut self, rotation: Rotation) -> u64 {
        let sign = self.current_value.signum();

        match rotation.direction {
            Direction::Left => self.current_value -= rotation.distance,
            Direction::Right => self.current_value += rotation.distance,
        };

        let new_sign = self.current_value.signum();

        let mut crosses = 0;
        if new_sign != sign {
            crosses += 1;
        } else if self.current_value >= 100 {
            crosses += self.current_value / 100;
        }

        // println!("curr {} crosses here {}", self.current_value, crosses);

        self.current_value = self.current_value.rem_euclid(100);

        return crosses as u64;
    }
}

fn parse(input: &str) -> Result<Vec<Rotation>, &'static str> {
    input.lines().map(|line| Rotation::from_str(line)).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (zeros, _) = solve(input);

    Some(zeros)
}

fn solve(input: &str) -> (u64, u64) {
    let rotations = parse(input).ok().unwrap();

    let mut dial = Dial::new();

    let mut zeros = 0;
    let mut crosses = 0;

    for rotation in rotations {
        // println!("Rotating {:?} by {}", rotation.direction, rotation.distance);
        let c = dial.rotate(rotation);

        if dial.current_value != 0 {
            crosses += c;
        }

        if dial.current_value == 0 {
            zeros += 1;
        }

        // println!("Current value: {}, zeros: {}, crosses: {}", dial.current_value, zeros, crosses);
    }

    return (zeros, crosses);
}

pub fn part_two(input: &str) -> Option<u64> {
    let (zeros, crosses) = solve(input);

    Some(zeros + crosses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_rotation_crosses() {
        let mut dial = Dial::new();

        let rotation = Rotation {
            direction: Direction::Left,
            distance: 1000,
        };

        let crosses = dial.rotate(rotation);
        assert_eq!(crosses, 10);
    }
}
