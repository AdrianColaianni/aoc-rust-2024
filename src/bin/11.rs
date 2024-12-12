use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn blink(s: usize, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    if s == 0 {
        return blink(1, depth - 1);
    }

    let len = s.ilog10() + 1;
    if len % 2 == 0 {
        let f = 10_usize.pow(len / 2);
        return blink(s / f, depth - 1) + blink(s % f, depth - 1);
    }

    return blink(s * 2024, depth - 1);
}

pub fn part_one(input: &str) -> Option<u32> {
    let ans: usize = input
        .trim()
        .split(' ')
        .map(|s| blink(s.parse().unwrap(), 25))
        .sum();
    Some(ans as u32)
}

pub fn blink_vec(s: usize, depth: usize) -> Vec<usize> {
    if depth == 0 {
        return vec![s];
    }

    let mut stones = vec![];
    if s == 0 {
        stones.append(&mut blink_vec(1, depth - 1));
        return stones;
    }

    let len = s.ilog10() + 1;
    if len % 2 == 0 {
        let f = 10_usize.pow(len / 2);
        stones.append(&mut blink_vec(s / f, depth - 1));
        stones.append(&mut blink_vec(s % f, depth - 1));
        return stones;
    }

    stones.append(&mut blink_vec(s * 2024, depth - 1));
    stones
}

pub fn part_two(input: &str) -> Option<usize> {
    let stones: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    // Stores what result of 25 blinks
    let mut twenty_five_blinks: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut first = vec![];

    // hash of all stones from first run, with result and number of times those stones appear
    let mut second: HashMap<usize, (Vec<usize>, usize)> = HashMap::new();
    let mut third: HashMap<usize, (Vec<usize>, usize)> = HashMap::new();

    // First 25
    for stone in stones {
        let mut b = twenty_five_blinks
            .entry(stone)
            .or_insert_with(|| blink_vec(stone, 25))
            .clone();
        first.append(&mut b);
    }

    // Second 25
    for stone in first {
        second
            .entry(stone)
            .and_modify(|(_, c)| *c += 1)
            .or_insert_with(|| {
                let b = twenty_five_blinks
                    .entry(stone)
                    .or_insert_with(|| blink_vec(stone, 25))
                    .clone();
                (b, 1)
            });
    }

    // Third 25
    for v in second.into_values() {
        for stone in v.0 {
            third
                .entry(stone)
                .and_modify(|(_, c)| *c += v.1)
                .or_insert_with(|| {
                    let b = twenty_five_blinks
                        .entry(stone)
                        .or_insert_with(|| blink_vec(stone, 25))
                        .clone();
                    (b, v.1)
                });
        }
    }

    let mut ans = 0;
    for (v, c) in third.values() {
        ans += c * v.len();
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
