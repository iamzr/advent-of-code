use core::panic::PanicInfo;

advent_of_code::solution!(5);

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {

    if let Some((checks, updates))= input.split_once("\n\n") {
        let mut rules = vec![Vec::new(); 100];

        for check in checks.lines(){
            if let Some((x,y)) = check.split_once("|") {
                    let x = x.parse::<usize>().expect("Failed to parse number");
                    let y = y.parse::<usize>().expect("Failed to parse number");

                    rules[x].push(y);
                };
            // panic!("Failed to parse");
            }
            
        let updates = updates.lines().map(|s| s.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>();

        return (rules, updates);
    }

    panic!("Failed to parse");
}


fn check_updates(rules: &Vec<Vec<usize>>, updates: Vec<Vec<usize>>) -> usize {
    let mut sum = 0;
    'update: for update in updates {
        for (index, value) in update.iter().enumerate() {
            let checks = &rules[*value];

            for i in index+1..update.len() {
                let ahead = &update[i];

                if !checks.contains(ahead) {
                    println!("Failed, update {:?} had problem with {:?}", update,ahead);
                    continue 'update;
                }
            }
        }
        // Update is valid
        println!("Update was valid {:?}", update);
        sum += update[update.len() /2];
    }

    return sum;

}
    

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, updates) = parse(input);

    print!("rules {:?} \n updates: {:?}", rules, updates);
    Some(check_updates(&rules, updates))

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

        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
