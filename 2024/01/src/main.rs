use std::fs;
use std::collections::HashSet;
use counter::Counter;

fn main() {
    let file_path = "./input.txt"; 

    let contents = fs::read_to_string(file_path)
        .unwrap();
    
    let values: Vec<_> = contents.split_whitespace()
        .collect();

    let s1: Vec<i32> = values.iter().step_by(2).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let s2: Vec<i32> = values.iter().skip(1).step_by(2).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    part_2(s1, s2);

}

fn part_1(mut s1: Vec<i32>, mut s2: Vec<i32>) {

    s1.sort();
    s2.sort();

    let diff: i32 = s1.into_iter().zip(s2).map(|(x,y)| (y - x).abs()).sum();

    println!("{:?}", diff); 
}

fn part_2(mut s1: Vec<i32>, mut s2: Vec<i32>) {
    let counter = s2.iter().collect::<Counter<_>>();
    
    println!("{:?}", counter);

    let unique_values: HashSet<&i32> = s1.iter().collect::<HashSet<_>>();

    println!("{:?}", unique_values);

    let keys: HashSet<&i32> = counter.keys().cloned().collect::<HashSet<_>>();

    let intersection = keys.intersection(&unique_values);

    let mut ans = 0;

    for value in s1 {
        ans += value * counter[&value] as i32
    };

    println!("{:?}", ans);




}