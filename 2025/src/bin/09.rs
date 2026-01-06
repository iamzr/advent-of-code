advent_of_code::solution!(9);

type Coordinate = (i64, i64);

fn find_largest_rectangle(p1: &Coordinate, p2: &Coordinate) -> u64 {
    if p1 == p2 {
        return 0;
    }

    let delta_1 = (p2.1 - p1.1).abs() + 1;
    if p1.0 == p2.0 {
        return delta_1 as u64;
    }

    let delta_0 = (p2.0 - p1.0).abs() + 1;
    if p1.1 == p2.1 {
        return delta_0 as u64;
    }

    return (delta_0 * delta_1) as u64;
}

fn parse(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .map(|l| {
            l.split_once(",")
                .map(|(x, y)| {
                    let x = x.parse::<i64>().unwrap();
                    let y = y.parse::<i64>().unwrap();

                    return (x, y);
                })
                .unwrap()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let Coordinates = parse(input);

    let mut max = 0;
    for (i, p1) in Coordinates.iter().enumerate() {
        for p2 in Coordinates.iter().skip(i + 1) {
            let area = find_largest_rectangle(p1, p2);

            max = max.max(area)
        }
    }

    return Some(max);
}

#[derive(PartialEq)]
enum Orientation {
    Collinear,
    Clockwise,
    AntiClockwise,
}

fn orientation(a: Coordinate, b: Coordinate, c: Coordinate) -> Orientation {
    let val = (b.1 - a.1) * (c.0 - b.0) - (b.0 - a.0) * (c.1 - b.1);

    if val == 0 {
        Orientation::Collinear
    } else if val > 0 {
        Orientation::Clockwise
    } else {
        Orientation::AntiClockwise
    }
}

fn on_segment(a: Coordinate, b: Coordinate, c: Coordinate) -> bool {
    b.0 >= a.0.min(c.0) && b.0 <= a.0.max(c.0) && b.1 >= a.1.min(c.1) && b.1 <= a.1.max(c.1)
}

fn segments_intersect(p1: Coordinate, p2: Coordinate, q1: Coordinate, q2: Coordinate) -> bool {
    let o1 = orientation(p1, p2, q1);
    let o2 = orientation(p1, p2, q2);
    let o3 = orientation(q1, q2, p1);
    let o4 = orientation(q1, q2, p2);

    // General case
    if o1 != o2 && o3 != o4 {
        // // check that intersection is **not just an endpoint**
        if !(p1 == q1 || p1 == q2 || p2 == q1 || p2 == q2) {
            return true;
        }
    }

    false
}

fn is_valid_rectangle(boundaries: &Vec<Coordinate>, rectangle: (&Coordinate, &Coordinate)) -> bool {
    let low_x = rectangle.0.0.min(rectangle.1.0);
    let high_x = rectangle.0.0.max(rectangle.1.0);
    let low_y = rectangle.0.1.min(rectangle.1.1);
    let high_y = rectangle.0.1.max(rectangle.1.1);

    let points = vec![
        (low_x, low_y),
        (low_x, high_y),
        (high_x, low_y),
        (high_x, high_y),
    ];

    let lines: Vec<(&Coordinate, &Coordinate)> = points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| points.iter().skip(i + 1).map(move |p2| (p1, p2)))
        .collect();

    let boundaries = boundaries.iter().zip(boundaries.iter().cycle().skip(1));

    for (&a, &b) in boundaries {
        for (c, d) in &lines {
            if segments_intersect(a, b, **c, **d) {
                // println!("Boundary {:?}, {:?} is intersected by the edge {:?}, {:?}", a,b,c,d);
                return false;
            }
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u64> {
    let coordinates = parse(input);

    let mut max = 0;
    for (i, p1) in coordinates.iter().enumerate() {
        for p2 in coordinates.iter().skip(i + 1) {
            // println!("Check Rectangle {:?} {:?}", p1, p2);
            if !is_valid_rectangle(&coordinates, (p1, p2)) {
                // println!("Rectangle {:?} {:?} is invalid", p1, p2);
                continue;
            }

            let area = find_largest_rectangle(p1, p2);

            max = max.max(area)
        }
    }

    return Some(max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
