use std::fs::read_to_string;

fn main() {
    let filename: &str = "input.txt";


    for line in read_to_string(filename).unwrap().lines() {
        for c in line.chars() {
            if c.is_ascii_punctuation {

            }
        }

    }
}
