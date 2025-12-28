advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let input: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().to_vec()) // Use bytes instead of char because it's faster
        .collect();

    let data = input;

    let accessible_rolls = calculate_accessible_rolls(&data);

    Some(accessible_rolls)
}

fn calculate_accessible_rolls(data: &Vec<Vec<u8>>) -> u64 {
    let mut accessible_rolls = 0;

    let m = data.len();
    let n = data[0].len();

    for row in 0..m {
        for column in 0..n {
            if data[row][column] != b'@' {
                continue;
            }

            if is_accessible_roll(&data, row, column) {
                accessible_rolls += 1;
            }
        }
    }
    accessible_rolls
    }



fn is_accessible_roll(data: &Vec<Vec<u8>>, row: usize, column: usize) -> bool {
    static DELTAS: &[(isize, isize); 8] = &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ];

    let mut adjacent_rolls = 0;
    for &(dx, dy) in DELTAS {
        let x = match column.checked_add_signed(dx) {
            Some(v) => v,
            None => continue,
        };

        let y = match row.checked_add_signed(dy) {
            Some(v) => v,
            None => continue,
        };

        if data.get(y).and_then(|r| r.get(x)) == Some(&b'@') {
                adjacent_rolls += 1;
                if adjacent_rolls >= 4 {
                    return false;
                }
            }
    }

    true
}

fn prune_accessible_rolls(data: &mut Vec<Vec<u8>>) -> u64 {
    let mut accessible_rolls = 0;

    let m = data.len();
    let n = data[0].len();

    for row in 0..m {
        for column in 0..n {
            if data[row][column] != b'@' {
                continue;
            }

            if is_accessible_roll(&data, row, column) {
                data[row][column] = b'.';
                accessible_rolls += 1;
            }
        }
    }

    return accessible_rolls

}

pub fn part_two(input: &str) -> Option<u64> {
    let mut data: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().to_vec()) // Use bytes instead of char because it's faster
        .collect();

    let mut pruned_rolls = 0;
    while calculate_accessible_rolls(&data) > 0 {
        pruned_rolls += prune_accessible_rolls(&mut data);
    }

    Some(pruned_rolls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
