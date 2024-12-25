use regex::Regex;

advent_of_code::solution!(14);

#[derive(Debug,PartialEq)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

struct Corridor {
    dimensions: (i32, i32),
    robots: Vec<Robot>,
    time: i32,
}

impl Corridor {
    pub fn set_time(&mut self, t_delta: i32) {
        for robot in &mut self.robots {
            let x = (robot.position.0 + robot.velocity.0 * t_delta).rem_euclid(self.dimensions.0);
            let y = (robot.position.1 + robot.velocity.1 * t_delta).rem_euclid(self.dimensions.1);

            robot.position = (x,y);
        }

        self.time += t_delta;
    }

    fn robots_per_quadrant(&self) -> (u32, u32, u32, u32) {
        let (mut q1, mut q2, mut q3, mut q4) = (0,0,0,0);

        for robot in &self.robots {
            let x = robot.position.0;
            let y = robot.position.1;

            if (0 <= x) && (x < (self.dimensions.0 / 2)) && (0 <= y) && (y < (self.dimensions.1 / 2)) {
                q1 += 1;
            }
            else if (x > (self.dimensions.0 / 2)) && (x < (self.dimensions.0)) && (0 <= y) && (y < self.dimensions.1 / 2) {
                q2 += 1;
            }
            else if (0 <= x) && (x < (self.dimensions.0 / 2)) && (y > (self.dimensions.1 / 2)) && (y < self.dimensions.1) {
                q3 += 1;
            }
            else if (x > (self.dimensions.0 / 2)) && (x < (self.dimensions.0)) && (y > (self.dimensions.1 / 2)) && (y < self.dimensions.1) {
                q4 += 1;
            } 
        }

        (q1, q2, q3, q4)
    }

    pub fn current_safety_factor(self) -> u32 {
        let (q1, q2, q3, q4 )= self.robots_per_quadrant();

        q1 * q2 * q3 * q4

    }
}

fn parse(input: &str) -> Vec<Robot> {
    let re = Regex::new(
        r"p=(?P<x1>-?\d+),(?P<y1>-?\d+) v=(?P<vx>-?\d+),(?P<vy>-?\d+)"
    ).unwrap();

    re.captures_iter(input).map(|captures| {
        // Extract numbers using named groups
        let x1: i32 = captures["x1"].parse().unwrap();
        let y1: i32 = captures["y1"].parse().unwrap();
        let vx: i32 = captures["vx"].parse().unwrap();
        let vy: i32 = captures["vy"].parse().unwrap();

        Robot { position: (x1, y1), velocity: (vx, vy)}
        }
    ).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let dimensions = (101, 103);
    _part_one(input, dimensions)
}

fn _part_one(input: &str, dimensions: (i32, i32)) -> Option<u32> {
    let robots = parse(input);

    let mut c = Corridor { dimensions: dimensions, robots: robots, time: 0 };

    c.set_time(100);

    Some(c.current_safety_factor())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_multiple() {
        let input = "p=111,-222 v=333,444
        p=55,-6 v=99,100";

        let expected = vec![Robot { position: (111,-222), velocity: (333,444)}, Robot { position: (55, -6), velocity: (99, 100)}];

        let result = parse(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse() {
        let input = "p=1,-2 v=3,4";

        let expected = vec![Robot { position: (1,-2), velocity: (3,4)}];

        let result = parse(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        let dimensions = (11, 7);
        let result = _part_one(&advent_of_code::template::read_file("examples", DAY), dimensions);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
