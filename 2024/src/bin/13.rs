use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(13);

fn parse(input: &str) -> Vec<GameConfig> {
    let re = Regex::new(r"Button A: X[+=](\d+), Y[+=](\d+)\nButton B: X[+=](\d+), Y[+=](\d+)\nPrize: X[+=](\d+), Y[+=](\d+)").unwrap();

    let configs = re
        .captures_iter(input)
        .map(|c| GameConfig {
            button_a_delta: (c[1].parse::<u32>().unwrap(), c[2].parse::<u32>().unwrap()),
            button_b_delta: (c[3].parse::<u32>().unwrap(), c[4].parse::<u32>().unwrap()),
            prize: (c[5].parse::<u32>().unwrap(), c[6].parse::<u32>().unwrap()),
        })
        .collect();

    configs
}

#[derive(Debug, PartialEq)]
struct GameConfig {
    button_a_delta: (u32, u32),
    button_b_delta: (u32, u32),
    prize: (u32, u32),
}

trait FewestTokensStrategy {
    fn execute(&self, config: &GameConfig) -> Option<u32>;
}

struct TopDownMemoizationStrategy {}

impl FewestTokensStrategy for TopDownMemoizationStrategy {
    fn execute(&self, config: &GameConfig) -> Option<u32> {
        let prize = config.prize;
        let mut m = HashMap::new();
        TopDownMemoizationStrategy::fewest_tokens_from_position(config, prize, &mut m)
    }
}

impl TopDownMemoizationStrategy {
    fn fewest_tokens_from_position(
        config: &GameConfig,
        position: (u32, u32),
        m: &mut HashMap<(u32, u32), Option<u32>>,
    ) -> Option<u32> {
        if let Some(p) = m.get(&position) {
            return *p;
        }

        if position == (0, 0) {
            return Some(0);
        }

        // Fpr current position you can either come from an A press or a B press
        //

        let x = position.0.checked_sub(config.button_a_delta.0);
        let y = position.1.checked_sub(config.button_a_delta.1);

        let tokens_from_a = match (x, y) {
            (Some(x), Some(y)) => {
                TopDownMemoizationStrategy::fewest_tokens_from_position(config, (x, y), m)
            }
            (None, _) => None,
            (_, None) => None,
        };

        let x1 = position.0.checked_sub(config.button_b_delta.0);
        let y2 = position.1.checked_sub(config.button_b_delta.1);

        let tokens_from_b = match (x1, y2) {
            (Some(x), Some(y)) => {
                TopDownMemoizationStrategy::fewest_tokens_from_position(config, (x, y), m)
            }
            (None, _) => None,
            (_, None) => None,
        };

        let value = match (tokens_from_a, tokens_from_b) {
            (Some(a), Some(b)) => {
                if a < b {
                    Some(a + 3)
                } else {
                    Some(b + 1)
                }
            }
            (Some(a), None) => Some(a + 3),
            (None, Some(b)) => Some(b + 1),
            (None, None) => None,
        };

        m.insert(position, value);

        value
    }
}

struct GameMachine<T: FewestTokensStrategy> {
    config: GameConfig,
    fewest_token_strategy: T,
}

impl<T: FewestTokensStrategy> GameMachine<T> {
    fn fewest_tokens(&self) -> Option<u32> {
        self.fewest_token_strategy.execute(&self.config)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let configs = parse(input);

    let tokens = configs
        .into_iter()
        .map(|c| {
            let machine = GameMachine {
                config: c,
                fewest_token_strategy: TopDownMemoizationStrategy {},
            };

            match machine.fewest_tokens() {
                Some(t) => t,
                None => 0,
            }
        })
        .sum();

    Some(tokens)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_game_machine {
        use crate::GameMachine;

        #[test]
        fn test_fewest_tokens() {
            let machine = GameMachine {
                config: crate::GameConfig {
                    button_a_delta: (94, 34),
                    button_b_delta: (22, 67),
                    prize: (8400, 5400),
                },
            };

            let result = machine.fewest_tokens();

            assert_eq!(result, Some(280));
        }
    }

    #[test]
    fn test_parse() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";

        let config = GameConfig {
            button_a_delta: (94, 34),
            button_b_delta: (22, 67),
            prize: (8400, 5400),
        };

        let result = parse(input);

        assert_eq!(result[0], config);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
