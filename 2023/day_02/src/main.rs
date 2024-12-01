use std::fs::read_to_string;

fn main() {
    let filename = "input.txt";

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut sum = 0;

    'game: for line in read_to_string(filename).unwrap().lines() {
        let parts: Vec<&str> = line.split(":").collect();

        let game_id: Vec<_> = parts[0].split(" ").collect();

        // println!("{:?}", game_id);

        let games: Vec<&str> = parts[1].split(";").collect();

        // println!("{}", games[0]);

        for game in games {
            let balls: Vec<&str> = game.split(",").collect();

            for ball in balls {
                // println!("{}", ball);

                let no_of_balls: Vec<&str> = ball.split(" ").collect();
                let max = match no_of_balls[2] {
                    "red" => max_red,
                    "green" => max_green,
                    "blue" => max_blue,
                    _ => 0,
                };

                // println!("{:?}", no_of_balls);

                if no_of_balls[1].parse::<i32>().unwrap() > max {
                    continue 'game;
                }
            }
        }

        sum = sum + game_id[1].parse::<i32>().unwrap();
        println!("{}, {}", sum, game_id[1])
    }

    println!("Sum  is {}", sum)
}
