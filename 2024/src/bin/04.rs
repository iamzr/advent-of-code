advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let values = input
        .split("\n")
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("{:?}", values);

    let mut finds = 0;
    let word = vec!['X', 'M', 'A', 'S'];

    for i in (0..values.len()) {
        for j in (0..values.len()) {
            println!("starting ({}, {})", i, j);
            finds += search_grid(&values, i as i32, j as i32, &word);
        }
    }

    println!("{:?}", finds);

    Some(finds)
}

fn search_grid(grid: &Vec<Vec<char>>, row: i32, col: i32, word: &Vec<char>) -> u32 {
    if grid[row as usize][col as usize] != word[0] {
        return 0;
    }

    // Number of rows
    let m: i32 = grid.len().try_into().unwrap();

    // Number of columns
    let n: i32 = grid[0].len().try_into().unwrap();

    let word_len: usize = word.len();

    let directions = vec![
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    let mut number_of_finds = 0;

    println!("{:?}", (row, col));
    for (x, y) in directions {
        let mut current_x: i32 = row + x;
        let mut current_y: i32 = col + y;
        let mut k = 1;

        // println!("direction {:?}", (x,y));

        'inner: while k < word_len {
            if current_x >= m || current_x < 0 || current_y >= n || current_y < 0 {
                break 'inner;
            }

            let c = grid[current_x as usize][current_y as usize];

            if c != word[k] {
                break 'inner;
            }
            println!("{} {} {} {} {} ", current_x, current_y, k, c, word[k]);

            current_x += x;
            current_y += y;
            k += 1;
        }

        if k == word_len {
            number_of_finds += 1;
            println!("Found starting at {:?}", (row, col))
        }
    }

    number_of_finds
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
