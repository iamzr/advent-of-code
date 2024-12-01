use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";

    let max_calories = find_max_calories(filename);
    println!("{max_calories}");

    let sum_of_top_three = sum_of_top_three_calorie_holders(filename);
    println!("{sum_of_top_three}");
}

fn sum_of_top_three_calorie_holders(filename: &str) -> i32 {
    let mut current_calories = 0;
    let mut top_calories = vec![0; 3];

    // read the file line by line
    for line in read_to_string(filename).unwrap().lines() {
        if line.is_empty() {
            'inner: for c in &top_calories {
                if current_calories > *c {
                    top_calories.pop();
                    top_calories.push(current_calories);
                    top_calories.sort_by(|a, b| b.cmp(a));

                    break 'inner;
                }
            }

            current_calories = 0;
        } else {
            current_calories = current_calories + line.parse::<i32>().unwrap();
        }
    }

    top_calories.iter().sum()
}

fn find_max_calories(filename: &str) -> i32 {
    let mut current_calories = 0;
    let mut max_calories = 0;

    // read the file line by line
    for line in read_to_string(filename).unwrap().lines() {
        if line.is_empty() {
            if current_calories > max_calories {
                max_calories = current_calories;
            }

            current_calories = 0;
        } else {
            current_calories = current_calories + line.parse::<i32>().unwrap();
        }
    }

    max_calories
}
