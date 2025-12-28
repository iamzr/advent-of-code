advent_of_code::solution!(4);

struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y)?.get(x)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let data = Matrix { data: input };

    let deltas: Vec<(isize, isize)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ];

    let mut accessible_rolls = 0;

    let m = data.data.len();
    let n = data.data[0].len();

    for row in 0..m {
        for column in 0..n {
            match data.get(column, row) {
                Some('@') => {}
                _ => continue,
            }

            let mut adjacent_rolls = 0;
            for delta in &deltas {
                let y = match row.checked_add_signed(delta.1) {
                    Some(v) => v,
                    None => continue,
                };

                let x = match column.checked_add_signed(delta.0) {
                    Some(v) => v,
                    None => continue,
                };

                match data.get(x, y) {
                    Some('@') => adjacent_rolls += 1,
                    _ => {}
                }
            }

            if adjacent_rolls < 4 {
                accessible_rolls += 1;
            }
        }
    }

    Some(accessible_rolls)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
