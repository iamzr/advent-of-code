use std::fs::read_to_string;

fn main() {
    let filename: &str = "input.txt";

    let sum = find_and_sum_calibration_values_1(filename);

    println!("{}", sum);
}

// fn find_and_sum_calibration_values_2() {
//     let mut sum = 0;
//     for line in read_to_string(filename).unwrap().lines() {
//         let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
//         let calibration_value = digits.first().unwrap().to_digit(10).unwrap() * 10
//             + digits.last().unwrap().to_digit(10).unwrap();

//         sum = sum + calibration_value;
//     }
// }

fn find_and_sum_calibration_values_1(filename: &str) -> u32 {
    let mut sum = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        let calibration_value = digits.first().unwrap().to_digit(10).unwrap() * 10
            + digits.last().unwrap().to_digit(10).unwrap();

        sum = sum + calibration_value;
    }

    sum
}
