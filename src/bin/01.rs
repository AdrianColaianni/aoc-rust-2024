use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut a, mut b): (Vec<u32>, Vec<u32>) = (vec![], vec![]);
    let mut res: u32 = 0;

    for l in input.trim().split('\n').into_iter() {
        let mut l = l.split_whitespace();
        a.push(l.next().unwrap().parse().unwrap());
        b.push(l.next().unwrap().parse().unwrap());
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
    let mut b: HashMap<u32, u32> = HashMap::new();
    let mut res: u32 = 0;

    for l in input.trim().split('\n').into_iter() {
        let mut l = l.split_whitespace();
        a.push(l.next().unwrap().parse().unwrap());
        b.entry(l.next().unwrap().parse().unwrap())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    for v in a {
        res += v * b.get(&v).unwrap_or(&0);
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
