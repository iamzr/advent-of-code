use std::fs;
use regex::Regex;
fn main() {
    let file_path = "./input.txt"; 

    let contents = fs::read_to_string(file_path)
        .unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let parts: Vec<_> = re.captures_iter(&contents).map(|cap| 
        {
        println!("{:?}", &cap);

        cap[1].parse::<i32>().unwrap()* cap[2].parse::<i32>().unwrap()
    }).collect();

    println!("{:?}", parts);

    let sum: i32 = parts.iter().sum();

    println!("{:?}", sum);
    
}
