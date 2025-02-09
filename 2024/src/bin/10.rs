advent_of_code::solution!(10);

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("Char not a digit"))
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn next_steps(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let deltas = vec![(1, 0), (-1, 0), (0, -1), (0, 1)];

    let mut result = vec![];
    for delta in deltas {
        let x = pos.0.checked_add_signed(delta.0);
        let y = pos.1.checked_add_signed(delta.1);

        let x = match x {
            Some(x) => x,
            None => continue,
        };

        let y = match y {
            Some(y) => {
                y
            }
            None => continue,
        };

        let pos = (x,y);

        if !is_valid_position(grid, pos) {
            continue;
        }

        result.push((x, y));
    }

    result
}

fn is_valid_position(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> bool {
    if (pos.0 >= grid[0].len()) || (pos.1 >= grid.len()) {
        return false;
    }
    true
}

fn find_paths(
    grid: &Vec<Vec<u32>>,
    pos: (usize, usize),
    counter: &mut u32,
    achieved: &mut Vec<(usize, usize)>,
) {
    // Base cases
    // 1. Found a 9
    if grid[pos.1][pos.0] == 9 {
        if !achieved.contains(&pos) {
            achieved.push(pos);
            *counter += 1;
        }

        return;
    }

    // 2. Reach edge of the grid
    if !is_valid_position(grid, pos) {
        return;
    }

    // Not a base case, so loop over all the next possible positions
    for next_pos in next_steps(grid, pos) {
        match grid[next_pos.1][next_pos.0].checked_sub(grid[pos.1][pos.0]) {
            None => continue,
            Some(x) => {
                if x != 1 {
                    continue;
                }
            }
        }

        find_paths(grid, next_pos, counter, achieved);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);

    let mut global_counter = 0;

    for i in 0..grid[0].len() {
        for j in 0..grid.len() {
            if grid[j][i] == 0 {
                let mut counter = 0;

                let mut achieved = vec![];
                find_paths(&grid, (i, j), &mut counter, &mut achieved);

                global_counter += counter;
            }
        }
    }

    Some(global_counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_next_steps() {
        let pos = (5, 5);

        let grid = vec![vec![0; 8]; 8];

        let steps = next_steps(&grid, pos);
        let steps = steps.iter().copied().collect::<HashSet<(usize, usize)>>();

        let mut expected = HashSet::new();
        expected.insert((4, 5));
        expected.insert((6, 5));
        expected.insert((5, 4));
        expected.insert((5, 6));

        assert_eq!(steps, expected);
    }

    #[test]
    fn test_find_paths() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let grid = parse(&input);

        let zeros = vec![((2, 0), 5), ((4, 0), 6), ((4, 2), 5), ((6, 4), 3)];

        for (pos, expected) in zeros {
            let mut counter = 0;

            let mut achieved = vec![];

            find_paths(&grid, pos, &mut counter, &mut achieved);

            assert_eq!(counter, expected);
        }
    }

    #[test]
    fn test_is_valid_position() {
        let grid = vec![vec![0; 8]; 8];


        let positions = vec![
            ( (0,0), true),
            ( (8,0), false),
            ((7,0), true),
            ((0,8), false),
            ((0,7), true),
            ((8,8), false),
            ((7,7), true),
        ];
        
        for (pos, expected) in positions {
            assert_eq!(is_valid_position(&grid, pos), expected, "Failed for {:?}", pos);
        }
    }
}
