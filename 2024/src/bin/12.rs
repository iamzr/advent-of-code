use std::collections::HashMap;

advent_of_code::solution!(12);

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Region {
    area: u32,
    perimeter: u32,
}

impl Region {
    fn fence_price(&self) -> u32 {
        self.area * self.perimeter
    }
}
fn bfs(
    map: &Vec<Vec<char>>,
    checked: &mut Vec<Vec<bool>>,
    position: (usize, usize),
    region: &mut Region,
) {
    region.area += 1;

    let deltas: Vec<(isize, isize)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    for delta in deltas {
        let neighbour = (
            position.0.checked_add_signed(delta.0),
            position.1.checked_add_signed(delta.1),
        );

        // True if needs a fence, false otherwise
        let check: bool = match neighbour {
            (Some(x), Some(y)) => {
                if y < map.len() && (x < map[0].len()) && map[y][x] == map[position.1][position.0] {
                    if !checked[y][x] {
                        checked[y][x] = true;
                        bfs(map, checked, (x, y), region);
                    }
                    false
                } else {
                    true
                }
            }

            _ => true,
        };

        match check {
            true => region.perimeter += 1,
            false => {}
        }
    }
}

fn calculate_regions(map: &Vec<Vec<char>>) -> HashMap<char, Vec<Region>> {
    let mut regions: HashMap<char, Vec<Region>> = HashMap::new();

    let mut checked = vec![vec![false; map[0].len()]; map.len()];

    for j in 0..map.len() {
        for i in 0..map[0].len() {
            if checked[j][i] {
                continue;
            }

            let mut region = Region {
                area: 0,
                perimeter: 0,
            };

            checked[j][i] = true;
            bfs(map, &mut checked, (i, j), &mut region);

            regions
                .entry(map[j][i])
                .and_modify(|l| l.push(region.clone()))
                .or_insert(vec![region]);

            checked[j][i] = true;
        }
    }

    regions
}

fn calculate_fence_price(regions: &HashMap<char, Vec<Region>>) -> u32 {
    let mut total = 0;
    for region_list in regions.values() {
        for region in region_list.iter() {
            total += region.fence_price();
        }
    }

    total
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);

    let regions: HashMap<char, Vec<Region>> = calculate_regions(&map);

    let price = calculate_fence_price(&regions);

    Some(price)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_regions() {
        let map = parse(&advent_of_code::template::read_file("examples", DAY));

        let regions = calculate_regions(&map);

        let mut expected = HashMap::new();
        expected.insert(
            'R',
            vec![Region {
                area: 12,
                perimeter: 18,
            }],
        );
        expected.insert(
            'I',
            vec![
                Region {
                    area: 4,
                    perimeter: 8,
                },
                Region {
                    area: 14,
                    perimeter: 22,
                },
            ],
        );

        expected.insert(
            'C',
            vec![
                Region {
                    area: 14,
                    perimeter: 28,
                },
                Region {
                    area: 1,
                    perimeter: 4,
                },
            ],
        );

        for (key, expected_regions) in expected.iter() {
            assert_eq!(
                regions.get(&key),
                Some(expected_regions),
                "Incorrect for {}",
                key
            );
        }
    }

    mod test_region {
        use super::*;

        #[test]
        fn test_fence_price() {
            let r = Region {
                area: 5,
                perimeter: 12,
            };

            let price = r.fence_price();

            assert_eq!(price, 60);
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
