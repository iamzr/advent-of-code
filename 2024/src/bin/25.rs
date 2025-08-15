advent_of_code::solution!(25);

type Schematic = [usize; 5];

fn parse(input: &str) -> (Vec<Schematic>, Vec<Schematic>) {
    let (keys, locks): (Vec<Schematic>, Vec<Schematic>) =
        input
            .split("\n\n")
            .fold((vec![], vec![]), |(mut ks, mut ls), chunk| {
                let pins = chunk
                    .chars()
                    .filter(|c| *c == '#' || *c == '.')
                    .enumerate()
                    .fold([0; 5], |mut acc, (i, c)| {
                        if c == '#' {
                            acc[i % 5] += 1
                        }
                        acc
                    });

                if &chunk[0..5] != "#####" {
                    ks.push(pins)
                } else {
                    ls.push(pins)
                }
                (ks, ls)
            });

    (keys, locks)
}

fn find_combos(keys: &Vec<Schematic>, locks: &Vec<Schematic>) -> u32 {
    keys.iter()
        .flat_map(|k| {
            locks
                .iter()
                .filter(move |l| k.iter().zip(l.iter()).all(|(&kp, &lp)| kp + lp <= 7))
        })
        .count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let (keys, locks) = parse(input);
    let combos = find_combos(&keys, &locks);

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

        assert_eq!(locks[0], [1, 6, 4, 5, 4]);
        assert_eq!(keys[0], [6, 1, 3, 2, 4]);
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
