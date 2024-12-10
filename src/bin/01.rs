advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut a: Vec<u32> = vec![];
    let mut b: Vec<u32> = vec![];
    let mut res: u32 = 0;
    let digit_size = input.find(' ').unwrap();

    for i in (0..input.len()).step_by(input.find('\n').unwrap()+1) {
        a.push(input[i..i+digit_size].parse().unwrap());
        b.push(input[i+3+digit_size..i+3+digit_size*2].parse().unwrap());
    }

    a.sort();
    b.sort();

    for i in 0..a.len() {
        res += a[i].abs_diff(b[i]);
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut a: Vec<u32> = vec![];
    let mut res: u32 = 0;
    let digit_size = input.find(' ').unwrap();
    let mut b: Vec<u32> = vec![0; 10usize.pow(digit_size as u32)];

    for i in (0..input.len()).step_by(input.find('\n').unwrap()+1) {
        a.push(input[i..i+digit_size].parse().unwrap());
        b[input[i+3+digit_size..i+3+digit_size*2].parse::<usize>().unwrap()] += 1;
    }

    for v in a {
        res += v * b[v as usize];
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
