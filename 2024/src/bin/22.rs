advent_of_code::solution!(22);

fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|v| v.parse::<u64>().expect(format!("Couldn't parse {}", v).as_str())).collect()
}

fn mix(x: u64, y:u64) -> u64 {
    x ^ y
}

fn prune(x: u64) -> u64 {
    x % 16777216 
}

fn step_1(input: u64) -> u64 {
    prune((mix(input, input * 64)))
}

fn step_2(input: u64) -> u64 {
    prune((mix(input, input / 32)))

}

fn step_3(input: u64) -> u64 {
    prune((mix(input, input * 2048)))
}

fn generate_next_secret(secret: u64) -> u64 {
    let secret = step_1(secret);

    let secret = step_2(secret);

    let secret = step_3(secret);

    secret
}

pub fn part_one(input: &str) -> Option<u64> {
    let secrets = parse(input);

    let mut sum = 0;

    for secret in secrets {
        let mut value = secret;
        for _ in (0..2000) {
            value = generate_next_secret(value);
        }

        sum += value
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
