use std::io::{Error, ErrorKind};

use regex::Regex;

advent_of_code::solution!(17);

fn parse(input: &str) -> (Memory, Vec<u32>) {
    let re_register = Regex::new(r"Register ([A-C]): (\d+)").unwrap();
    let re_program = Regex::new(r"Program: ([\d,]+)").unwrap();

    let mut registers = [0u32; 3];
    for cap in re_register.captures_iter(input) {
        let reg_index = match &cap[1] {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => continue,
        };
        registers[reg_index] = cap[2].parse::<u32>().unwrap_or(0);
    }

    let program: Vec<u32> = if let Some(cap) = re_program.captures(input) {
        cap[1]
            .split(',')
            .filter_map(|v| v.trim().parse::<u32>().ok())
            .collect()
    } else {
        Vec::new()
    };

    (
        Memory {
            a: registers[0],
            b: registers[1],
            c: registers[2],
        },
        program,
    )
}

type ThreeBitNumber = u32;

#[derive(Eq, Hash, PartialEq)]
struct Memory {
    a: u32,
    b: u32,
    c: u32,
}

struct Computer {
    registers: Memory,
}

impl Computer {
    fn adv(&mut self, operand: u32, pointer: &mut usize) -> Option<u32> {
        self.registers.a = self.registers.a / (2_u32.pow(operand));
        *pointer += 2;
        None
    }

    fn bxl(&mut self, operand: u32, pointer: &mut usize) -> Option<u32> {
        self.registers.b ^= operand;
        *pointer += 2;
        None
    }

    fn bst(&mut self, operand: u32, pointer: &mut usize) -> Option<u32> {
        self.registers.b = operand % 8;
        *pointer += 2;
        None
    }

    fn jnz(&self, operand: u32, pointer: &mut usize) -> Option<u32> {
        println!("Calling jnz");
        if self.registers.a == 0 {
            *pointer += 2;
            return None;
        }

        *pointer = operand as usize;
        None
    }

    fn bxc(&mut self, operand: u32, pointer: &mut usize) -> Option<u32> {
        println!("Calling bxc");
        self.registers.b ^= self.registers.c;
        *pointer += 2;
        None
    }

    fn out(&self, operand: u32, pointer: &mut usize) -> Option<u32> {
        *pointer += 2;
        Some(operand % 8)
    }

    fn bdv(&mut self, operand: u32, pointer: &mut usize) -> Option<u32> {
        self.registers.b = self.registers.a / (2_u32.pow(operand));
        *pointer += 2;
        None
    }

    fn cdv(&mut self, operand: u32, pointer: &mut usize) -> Option<u32> {
        self.registers.c = self.registers.a / (2_u32.pow(operand));
        *pointer += 2;
        None
    }

    fn get_combo_operand(&self, value: ThreeBitNumber) -> Result<u32, Error> {
        match value {
            4 => Ok(self.registers.a),
            5 => Ok(self.registers.b),
            6 => Ok(self.registers.c),
            7 => Err(Error::new(
                ErrorKind::InvalidInput,
                "This combo operand is reserved",
            )),
            v => Ok(v),
        }
    }

    fn run(&mut self, program: Vec<u32>) -> Result<Vec<u32>, Error> {
        let mut pointer = 0;

        let mut output = vec![];
        while pointer + 1 < program.len() {
            let instruction = program[pointer];
            let literal_operand = program[pointer + 1];
            let combo_operand = match self.get_combo_operand(literal_operand) {
                Ok(v) => v,
                Err(_e) => return Err(_e),
            };

            println!("Instruction {}", instruction);
            let result = match instruction {
                0 => self.adv(combo_operand, &mut pointer),
                1 => self.bxl(literal_operand, &mut pointer),
                2 => self.bst(combo_operand, &mut pointer),
                3 => self.jnz(literal_operand, &mut pointer),
                4 => self.bxc(combo_operand, &mut pointer),
                5 => self.out(combo_operand, &mut pointer),
                6 => self.bdv(combo_operand, &mut pointer),
                7 => self.cdv(combo_operand, &mut pointer),
                _ => return Err(Error::new(ErrorKind::InvalidInput, "Invalid operand")),
            };

            match result {
                Some(v) => output.push(v),
                None => {}
            }

            println!("end of loop, with pointer at {}", pointer);
        }

        return Ok(output);
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (memory, program) = parse(input);

    let mut computer = Computer { registers: memory };

    match computer.run(program) {
        Ok(v) => Some(
            (v.iter()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join(",")),
        ),
        Err(_e) => None,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    mod instruction_tests {
        use super::*;

        #[test]
        fn test_adv() {
            let mut c = Computer {
                registers: Memory { a: 3, b: 0, c: 0 },
            };
            let mut pointer = 0;

            c.adv(2, &mut pointer);

            assert_eq!(c.registers.a, (3 / 4));
            assert_eq!(pointer, 2);
        }

        #[test]
        fn test_bdv() {
            let mut c = Computer {
                registers: Memory { a: 0, b: 3, c: 0 },
            };
            let mut pointer = 0;

            c.bdv(2, &mut pointer);

            assert_eq!(c.registers.b, (3 / 4));
            assert_eq!(pointer, 2);
        }

        #[test]
        fn test_cdv() {
            let mut c = Computer {
                registers: Memory { a: 0, b: 0, c: 3 },
            };
            let mut pointer = 0;

            c.cdv(2, &mut pointer);

            assert_eq!(c.registers.c, 3 / 4);
            assert_eq!(pointer, 2);
        }

        #[test]
        fn test_bst() {
            let mut c = Computer {
                registers: Memory { a: 0, b: 0, c: 0 },
            };
            let mut pointer = 0;

            c.bst(10, &mut pointer);

            assert_eq!(c.registers.b, 2);
            assert_eq!(pointer, 2);
        }

        #[test]
        fn test_jnz_with_a_zero() {
            let mut c = Computer {
                registers: Memory { a: 0, b: 0, c: 0 },
            };
            let mut pointer = 0;

            c.jnz(10, &mut pointer);

            assert_eq!(pointer, 2);
        }

        #[test]
        fn test_jnz_with_a_nonzero() {
            let mut c = Computer {
                registers: Memory { a: 10, b: 0, c: 0 },
            };
            let mut pointer = 0;

            c.jnz(3, &mut pointer);

            assert_eq!(pointer, 3);
        }

        #[test]
        fn test_bxc() {
            let mut c = Computer {
                registers: Memory { a: 0, b: 3, c: 5 },
            };
            let mut pointer = 0;

            c.bxc(3, &mut pointer);

            assert_eq!(c.registers.b, 3 ^ 5);
            assert_eq!(pointer, 2);
        }

        #[test]
        fn test_out() {
            let c = Computer {
                registers: Memory { a: 0, b: 3, c: 5 },
            };
            let mut pointer = 0;

            let result = c.out(9, &mut pointer);

            assert_eq!(result, Some(1));
            assert_eq!(pointer, 2);
        }
    }

    #[test]
    fn tes_run() {
        let test_cases: Vec<(
            Computer,
            Vec<u32>,
            Option<u32>,
            Option<u32>,
            Option<u32>,
            Option<Vec<u32>>,
        )> = vec![
            (
                Computer {
                    registers: Memory { a: 0, b: 0, c: 9 },
                },
                vec![2, 6],
                None,
                Some(1),
                None,
                None,
            ),
            (
                Computer {
                    registers: Memory { a: 10, b: 0, c: 0 },
                },
                vec![5, 0, 5, 1, 5, 4],
                None,
                None,
                None,
                Some(vec![0, 1, 2]),
            ),
            (
                Computer {
                    registers: Memory {
                        a: 2024,
                        b: 0,
                        c: 0,
                    },
                },
                vec![0, 1, 5, 4, 3, 0],
                Some(0),
                None,
                None,
                Some(vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]),
            ),
            (
                Computer {
                    registers: Memory { a: 0, b: 29, c: 0 },
                },
                vec![1, 7],
                None,
                Some(26),
                None,
                None,
            ),
            (
                Computer {
                    registers: Memory {
                        a: 0,
                        b: 2024,
                        c: 43690,
                    },
                },
                vec![4, 0],
                None,
                Some(44354),
                None,
                None,
            ),
        ];

        for (mut computer, program, expected_a, expected_b, expected_c, expected_output) in
            test_cases
        {
            let result = match computer.run(program) {
                Ok(o) => o,
                Err(_e) => panic!("Run failed: {}", _e),
            };

            match expected_a {
                Some(v) => assert_eq!(computer.registers.a, v),
                None => {}
            };

            match expected_b {
                Some(v) => assert_eq!(computer.registers.b, v),
                None => {}
            };

            match expected_c {
                Some(v) => assert_eq!(computer.registers.c, v),
                None => {}
            };

            match expected_output {
                Some(v) => assert_eq!(result, v),
                None => {}
            };
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
