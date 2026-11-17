advent_of_code::solution!(12);

struct Problem {
    length: usize,
    width: usize,

    // the number for each shape
    presents: HashMap<&Shape, usize>,
}

impl Problem {
    fn area_available(&self) -> usize {
        self.length * self.width
    }

    fn area_of_shapes(&self) -> usize {
        self.presents
            .iter()
            .map(|(shape, count)| shape.area * count)
            .sum()
    }

    fn non_overlapping_boxes(&self) -> usize {
        (self.length / 3) * (self.width / 3)
    }
}

struct Shape {
    area: usize,
}

trait PackingFinder {
    fn pack(region: &Problem) -> bool;
}

struct AutoRejectPackingFinder;

impl PackingFinder for AutoRejectPackingFinder {
    fn pack(region: &Problem) -> bool {
        false
    }
}

fn parse(input: &str) -> (Vec<Problem>, Vec<Shape>) {
    let (shapes_input, tree_input) = input.rsplit_once("\n\n").unwrap();

    let shapes = shape_part
        .split("\n\n")
        .map(|block| {
            let mut mask = 0u16;
            // skip the "0:" header line
            for (y, line) in block.lines().skip(1).enumerate() {
                for (x, c) in line.bytes().enumerate() {
                    if c == b'#' {
                        mask |= 1 << (y * 3 + x);
                    }
                }
            }
            mask
        })
        .collect();

    let problems = problem_part
        .lines()
        .map(|line| {
            let (dims, counts) = line.split_once(": ").unwrap();
            let (w, h) = dims.split_once('x').unwrap();
            let mut it = counts.split(' ').map(|n| n.parse().unwrap());
            Problem {
                width: w.parse().unwrap(),
                height: h.parse().unwrap(),
                counts: std::array::from_fn(|_| it.next().unwrap()),
            }
        })
        .collect();

    (problems, shapes)
}

///
/// For this problem, there's no clear algoirthm on we can reach for.
///
/// My method here:
/// 1. Check if there's sufficient space to fit the presents. If not, return False, else continue.
/// 2. The presents are 3x3, so check if there's sufficient space to fit non-overlapping 3x3 boxes.
///    Then True, else continue.
/// 3. If there's
pub fn part_one(input: &str) -> Option<u64> {
    let (problems, shapes) = parse(input);

    let packing_finder = AutoRejectPackingFinder;

    let count = problems
        .iter()
        .filter(|problem| problem.area_available() >= problem.area_of_shapes())
        .filter(|problem| {
            problem.non_overlapping_boxes() >= problem.shapes.values().sum()
                || packing_finder.pack(problem)
        })
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
