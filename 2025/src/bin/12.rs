advent_of_code::solution!(12);

use std::str::FromStr;

#[derive(Clone, Copy)]
struct Shape(u16); // 3x3 bitmask

impl Shape {
    fn area(self) -> usize {
        self.0.count_ones() as usize
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = 0u16;
        for (y, line) in s.lines().skip(1).enumerate() {
            for (x, c) in line.bytes().enumerate() {
                if c == b'#' {
                    mask |= 1 << (y * 3 + x);
                }
            }
        }
        Ok(Shape(mask))
    }
}

struct Problem {
    width: usize,
    height: usize,
    counts: Vec<usize>, // counts[i] = how many of shape i
}

impl FromStr for Problem {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, ()> {
        let (dims, counts) = line.split_once(": ").ok_or(())?;
        let (w, h) = dims.split_once('x').ok_or(())?;
        Ok(Problem {
            width: w.parse().map_err(|_| ())?,
            height: h.parse().map_err(|_| ())?,
            counts: counts
                .split(' ')
                .map(|n| n.parse().map_err(|_| ()))
                .collect::<Result<_, _>>()?,
        })
    }
}

struct Puzzle {
    shapes: Vec<Shape>,
    problems: Vec<Problem>,
}

impl FromStr for Puzzle {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, ()> {
        let (shapes, problems) = input.trim().rsplit_once("\n\n").ok_or(())?;
        Ok(Puzzle {
            shapes: shapes
                .split("\n\n")
                .map(str::parse)
                .collect::<Result<_, _>>()?,
            problems: problems.lines().map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

impl Puzzle {
    fn fits(&self, p: &Problem, packer: &impl PackingFinder) -> bool {
        let needed: usize = p
            .counts
            .iter()
            .zip(&self.shapes)
            .map(|(n, s)| n * s.area())
            .sum();

        if needed > p.width * p.height {
            return false;
        }

        let total: usize = p.counts.iter().sum();

        if (p.width / 3) * (p.height / 3) >= total {
            return true;
        }

        packer.pack(&self.shapes, p)
    }
}

trait PackingFinder {
    fn pack(&self, shapes: &[Shape], problem: &Problem) -> bool;
}

struct AutoRejectPackingFinder;

impl PackingFinder for AutoRejectPackingFinder {
    fn pack(&self, shapes: &[Shape], problem: &Problem) -> bool {
        false
    }
}

/// For this problem, there's no clear algoirthm on we can reach for.
///
/// My method here:
/// 1. Check if there's sufficient space to fit the presents. If not, return False, else continue.
/// 2. The presents are 3x3, so check if there's sufficient space to fit non-overlapping 3x3 boxes.
///    Then True, else continue.
/// 3. At this point, it means that a tight packing is required, so we'll have to use some method to
///    find the necessary packing.
pub fn part_one(input: &str) -> Option<u64> {
    let puzzle: Puzzle = input.parse().ok()?;
    let packer = AutoRejectPackingFinder;

    let count = puzzle
        .problems
        .iter()
        .filter(|p| puzzle.fits(p, &packer))
        .count();

    Some(count as u64)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
