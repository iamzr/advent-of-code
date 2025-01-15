use std::cmp::Ordering;

advent_of_code::solution!(5);

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {

    let (checks, updates)= input.split_once("\n\n").unwrap();
    
    let mut rules = vec![Vec::new(); 100];

    for check in checks.lines(){
        let (x,y) = check.split_once("|").unwrap();
        
        let x = x.parse::<usize>().expect("Failed to parse number");
        let y = y.parse::<usize>().expect("Failed to parse number");

        rules[x].push(y);
        
    };
        
    let updates = updates.lines().map(|s| s.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>();

    (rules, updates)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, updates) = parse(input);

    let compare = | x: &usize, y: &usize| !rules[*y].contains(x);

    let mut count = 0;
    for update in updates {
        if update.is_sorted_by(compare) {
            count += update[update.len() / 2];
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (rules, mut updates) = parse(input);

    let compare = | x: &usize, y: &usize| {
        if rules[*x].contains(y) {
            Ordering::Less
        } else if rules[*y].contains(x) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    let mut count = 0;
    for mut update in updates {
        if !update.is_sorted_by(|x, y| compare(x,y) != Ordering::Greater) {
            update.sort_by(compare);
            count += update[update.len() / 2];
        }
    }

    Some(count)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
