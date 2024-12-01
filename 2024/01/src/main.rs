use std::fs;

fn main() {
    let file_path = "./input.txt"; 

    let contents = fs::read_to_string(file_path)
        .unwrap();
    
    let values: Vec<_> = contents.split_whitespace()
        .collect();

    let mut s1: Vec<i32> = values.iter().step_by(2).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut s2: Vec<i32> = values.iter().skip(1).step_by(2).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    s1.sort();
    s2.sort();

    let diff: i32 = s1.into_iter().zip(s2).map(|(x,y)| (y - x).abs()).sum();

    println!("{:?}", diff); 
}