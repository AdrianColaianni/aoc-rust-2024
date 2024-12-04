advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut v = 0;
    for cap in re.captures_iter(input) {
        let x: u32 = cap[1].parse().unwrap();
        let y: u32 = cap[2].parse().unwrap();
        v += x * y;
    }

    Some(v)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    let mut v = 0;
    let mut doit = true;
    for cap in re.captures_iter(input) {
        match &cap[0][0..3] {
            "do(" => doit = true,
            "don" => doit = false,
            "mul" => {
                if doit {
                    let x: u32 = cap[1].parse().unwrap();
                    let y: u32 = cap[2].parse().unwrap();
                    v += x * y;
                }
            }
            _ => panic!(),
        }
    }

    Some(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
