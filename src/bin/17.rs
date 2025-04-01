advent_of_code::solution!(17);

fn serialize(input: &str) -> (usize, Vec<usize>) {
    let (reg, prog) = input.trim().split_once("\n\n").unwrap();
    let reg = reg.split_once("\n").unwrap().0;
    let reg_a = reg[12..].parse().unwrap();

    let prog = prog[9..].split(',').map(|i| i.parse().unwrap()).collect();

    (reg_a, prog)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut reg_a, prog) = serialize(input);
    let mut reg_b = 0;
    let mut reg_c = 0;

    let mut ip = 0;
    let mut output: Vec<String> = vec![];

    loop {
        if ip >= prog.len() {
            break;
        }
        let code = prog[ip];
        let mut and = prog[ip + 1];

        // Combo operand
        if code != 1 && code != 3 && code != 4 {
            and = match and {
                4 => reg_a,
                5 => reg_b,
                6 => reg_c,
                7 => panic!("Invalid combo operand"),
                v => v,
            };
        }

        match code {
            0 => {
                // adv: division
                reg_a = reg_a / 2_usize.pow(and as u32);
            }
            1 => {
                // bxl: bitwise XOR
                reg_b = reg_b ^ and;
            }
            2 => {
                // bst: modulo
                reg_b = and % 8;
            }
            3 => {
                // jnz: sometimes nothing
                if reg_a != 0 {
                    ip = and;
                    continue;
                }
            }
            4 => {
                // bxc: bitwirse XOR
                reg_b = reg_b ^ reg_c;
            }
            5 => {
                // out
                output.push((and % 8).to_string());
            }
            6 => {
                // bdv
                reg_b = reg_a / 2_usize.pow(and as u32);
            }
            7 => {
                // cdv
                reg_c = reg_a / 2_usize.pow(and as u32);
            }
            _ => panic!("Invalid opcode"),
        }

        ip += 2;
    }
    Some(output.join(","))
}

fn correct_run(mut reg_a: usize, prog: &Vec<usize>, output: &Vec<usize>) -> bool {
    let mut reg_b = 0;
    let mut reg_c = 0;
    let mut ip = 0;
    let mut out = 0;

    loop {
        if ip >= prog.len() {
            break;
        }
        let code = prog[ip];
        let mut and = prog[ip + 1];
        // println!("----");
        // println!("Reg: {}, {}, {}", reg_a, reg_b, reg_c);

        // Combo operand
        if code != 1 && code != 3 && code != 4 {
            and = match and {
                4 => reg_a,
                5 => reg_b,
                6 => reg_c,
                7 => panic!("Invalid combo operand"),
                v => v,
            };
        }

        // println!("{}: {}, {}", ip, code, and);

        match code {
            0 => {
                // adv: division
                reg_a = reg_a / 2_usize.pow(and as u32);
            }
            1 => {
                // bxl: bitwise XOR
                reg_b = reg_b ^ and;
            }
            2 => {
                // bst: modulo
                reg_b = and % 8;
            }
            3 => {
                // jnz: sometimes nothing
                if reg_a != 0 {
                    ip = and;
                    continue;
                }
            }
            4 => {
                // bxc: bitwirse XOR
                reg_b = reg_b ^ reg_c;
            }
            5 => {
                // out
                if output[out] != and % 8 {
                    return false;
                }
                out += 1;
            }
            6 => {
                // bdv
                reg_b = reg_a / 2_usize.pow(and as u32);
            }
            7 => {
                // cdv
                reg_c = reg_a / 2_usize.pow(and as u32);
            }
            _ => panic!("Invalid opcode"),
        }
        ip += 2;
    }
    out == output.len()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, prog) = serialize(input);
    let mut possible_reg = vec![0];

    for i in (0..prog.len()).rev() {
        let expected: Vec<usize> = prog[i..].iter().cloned().collect();
        let mut next_reg = vec![];

        for reg in possible_reg {
            for i in 0..8 {
                let reg = (reg << 3) + i;
                if correct_run(reg, &prog, &expected) {
                    next_reg.push(reg);
                }
            }
        }

        possible_reg = next_reg;
    }

    possible_reg.first().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5,7,3,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
