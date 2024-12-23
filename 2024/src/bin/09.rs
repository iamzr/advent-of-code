advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<usize> {
    input.chars().map(|c| c.to_digit(10).expect(format!("Cant parse {character}", character=c).as_str()) as usize).collect::<Vec<usize>>()
}

fn parse_with_dots(input: &str) -> Vec<Option<usize>> {
    input.chars().map(|c| match c {
        '.' => None,
        c => Some(c.to_digit(10).expect(format!("Cant parse {character}", character=c).as_str()) as usize)
    }).collect::<Vec<Option<usize>>>()
}

fn generate_blocks(input: Vec<usize>) -> Vec<Option<usize>>{
    input.chunks(2).enumerate().flat_map(|(i, v)| {
        let mut w: Vec<Option<usize>> = vec![Some(i); v[0]];
        if v.len() > 1 {
            w.extend(vec![None; v[1]].iter());
        }
        w
    }
    ).collect::<Vec<Option<usize>>>()
 }

fn compact_blocks(mut input: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut l = 0;
    let mut r = input.len() - 1;

    while l < r {
        if input[l] == None {
            while input[r] == None && r > 0 {
                r -= 1;
            }
            input[l] = input[r];
            input[r] = None;
            r -= 1;
        }
        l +=1;
    }
    input

}

fn calculate_checksum(input: Vec<Option<usize>>) -> usize {
    input.iter().enumerate().map(|(i, &n)| {match n {
        Some(x) => i  * x,
        None => { 0 }
    }} ).sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);

    let blocks = generate_blocks(input);

    let blocks = compact_blocks(blocks);

    let checksum = calculate_checksum(blocks);

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_checksum() {
        let input = parse_with_dots("0099811188827773336446555566..............");

        let result = calculate_checksum(input);

        assert_eq!(result, 1928)

    }

    #[test]
    fn test_generate_blocks() {
        let input = "2333133121414131402".chars().map(|d| d.to_digit(10).expect("Failed to parse") as usize).collect::<Vec<usize>>();

        let expected = "00...111...2...333.44.5555.6666.777.888899".chars().map(|d| match d {
            '.' => None,
            x => Some(x.to_digit(10).expect("Failed to parse") as usize) 
        }).collect::<Vec<Option<usize>>>();

        let result = generate_blocks(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_blocks_2() {
        let input = "12345".chars().map(|d| d.to_digit(10).expect("Failed to parse") as usize).collect::<Vec<usize>>();

        let expected = "0..111....22222".chars().map(|d| match d {
            '.' => None,
            x => Some(x.to_digit(10).expect("Failed to parse") as usize) 
        }).collect::<Vec<Option<usize>>>();

        let result = generate_blocks(input);

        assert_eq!(result, expected);
    }


    #[test]
    fn test_compact_blocks() {
        let input: Vec<Option<usize>> = parse_with_dots("0..111....22222");

        let expected = parse_with_dots("022111222......");

        let result = compact_blocks(input);

        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_compact_blocks_2() {
        let input: Vec<Option<usize>> = parse_with_dots("00...111...2...333.44.5555.6666.777.888899");

        let expected = parse_with_dots("0099811188827773336446555566..............");

        let result = compact_blocks(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
