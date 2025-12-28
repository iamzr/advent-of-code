advent_of_code::solution!(5);

/// Returns sorted, non-overlapping intervals
fn remove_overlapping_intervals(intervals: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    intervals.sort_by(|a, b| (a.0).cmp(&b.0));

    let mut merged = vec![intervals[0].clone()];

    for &current in intervals.iter().skip(1) {
        let last = merged.last_mut().unwrap();

        if current.0 <= last.1 {
            last.1 = last.1.max(current.1);
        } else {
            merged.push(current.clone());
        }
    }

    merged
}

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let (s1, s2) = input.split_once("\n\n").unwrap();

    let mut fresh_ingredient_ids = s1
        .lines()
        .map(|id_range| {
            let (start, end) = id_range.split_once("-").unwrap();

            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();

            (start, end)
        })
        .collect();

    // Remove overlapping intervals
    fresh_ingredient_ids = remove_overlapping_intervals(&mut fresh_ingredient_ids);

    let available_ingredient_ids = s2.lines().map(|l| l.parse::<u64>().unwrap()).collect();

    (fresh_ingredient_ids, available_ingredient_ids)
}

trait FreshnessChecker {
    fn is_ingredient_fresh(&self, ingredient_id: u64) -> bool;
}

struct SimpleFreshnessChecker {
    fresh_ingredient_ids: Vec<(u64, u64)>,
}

impl SimpleFreshnessChecker {
    pub fn new(fresh_ingredient_id_range: Vec<(u64, u64)>) -> Self {
        SimpleFreshnessChecker {
            fresh_ingredient_ids: fresh_ingredient_id_range,
        }
    }
}

impl FreshnessChecker for SimpleFreshnessChecker {
    fn is_ingredient_fresh(&self, ingredient_id: u64) -> bool {
        for &(start, end) in &self.fresh_ingredient_ids {
            println!(
                "For id={}, checking range ({}, {})",
                ingredient_id, start, end
            );
            if ingredient_id < start {
                return false;
            }

            if ingredient_id > end {
                continue;
            }

            if binary_search(ingredient_id, start, end) {
                return true;
            }
        }

        false
    }
}

fn linear_search(ingredient_id: u64, start: u64, end: u64) -> bool {
    for i in start..end + 1 {
        if ingredient_id == i {
            return true;
        }
    }

    false
}

fn binary_search(ingredient_id: u64, start: u64, end: u64) -> bool {
    let mut l = start;
    let mut r = end;

    while l <= r {
        let m = (l + r) / 2;

        if ingredient_id == m {
            return true;
        } else if ingredient_id < m {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let (fresh_ingredient_ids, available_ingredient_ids) = parse(input);

    println!("Parsed");

    let freshness_checker = SimpleFreshnessChecker::new(fresh_ingredient_ids);

    println!("Instantiated checker");

    let fresh_ids = available_ingredient_ids
        .into_iter()
        .filter(|id| {
            let r = freshness_checker.is_ingredient_fresh(*id);
            println!("for {}, fresh={}", id, r);

            r
        })
        .count();

    Some(fresh_ids as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (s1, _) = parse(input);

    println!("{:?}", s1);

    let result = s1.iter().map(|(start, end)| end - start + 1).sum();

    Some(result)
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_remove_overlapping_intervals() {
        let mut intervals = vec![(3, 5), (10, 14), (16, 20), (12, 18)];

        let result = remove_overlapping_intervals(&mut intervals);

        assert_eq!(result, vec![(3, 5), (10, 20)]);
    }
}
