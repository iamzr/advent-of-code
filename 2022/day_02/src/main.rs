use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let first_column_options = HashMap::<&str, RockPaperScissors>::from([
        ("A", RockPaperScissors::Rock),
        ("B", RockPaperScissors::Paper),
        ("C", RockPaperScissors::Scissors),
    ]);

    let second_column_options = HashMap::<&str, RockPaperScissors>::from([
        ("X", RockPaperScissors::Rock),
        ("Y", RockPaperScissors::Paper),
        ("Z", RockPaperScissors::Scissors),
    ]);
}

enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}
