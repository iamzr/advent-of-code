use std::collections::HashMap;

advent_of_code::solution!(19);

fn possible_to_create<'a>(
    pattern: &'a str,
    towels: &Vec<&str>,
    m: &mut HashMap<&'a str, bool>,
) -> bool {
    if let Some(v) = m.get(pattern) {
        return *v;
    };

    if pattern.is_empty() {
        return true;
    }

    let mut result = false;

    'outer: for towel in towels {
        for (i, c) in towel.chars().enumerate() {
            match pattern.chars().nth(i) {
                Some(p) => {
                    if c != p {
                        continue 'outer;
                    }
                }
                None => continue 'outer,
            }
        }

        result |= possible_to_create(&pattern[towel.len()..], towels, m);
    }

    m.insert(pattern, result);

    result
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();

    let towels = lines.next().unwrap().split(", ").collect();

    lines.next();

    let lines = lines.collect();

    (towels, lines)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (towels, patterns) = parse(input);

    let mut total = 0;

    for pattern in patterns {
        let mut m = HashMap::new();
        let result = possible_to_create(pattern, &towels, &mut m);

        if result {
            total += 1
        };
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_to_create() {
        let towels = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let cases = [
            ("brwrr", true),
            ("bggr", true),
            ("gbbr", true),
            ("rrbgbr", true),
            ("ubwu", false),
            ("bwurrg", true),
            ("brgr", true),
            ("bbrgwb", false),
        ];

        for (pattern, expected) in cases.iter() {
            let mut m = HashMap::new();
            let result = possible_to_create(pattern, &towels, &mut m);

            assert_eq!(result, *expected, "{}", pattern);
        }
    }

    #[test]
    fn test_parse() {
        let input = &advent_of_code::template::read_file("examples", DAY);

        let result = parse(input);

        let towels = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let cases = vec![
            "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", r"bbrgwb",
        ];
        let expected = (towels, cases);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
