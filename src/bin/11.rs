use std::collections::HashMap;

advent_of_code::solution!(11);

fn blink(mut stones: HashMap<usize, usize>, n: usize) -> HashMap<usize, usize> {
    for _ in 0..n {
        let mut next_stones = HashMap::new();
        for (stone, count) in stones {
            if stone == 0 {
                *next_stones.entry(1).or_default() += count;
                continue;
            }
            let len = stone.ilog10() + 1;
            if len % 2 == 0 {
                let f = 10_usize.pow(len / 2);
                *next_stones.entry(stone / f).or_default() += count;
                *next_stones.entry(stone % f).or_default() += count;
                continue;
            }

            *next_stones.entry(stone * 2024).or_default() += count;
        }
        stones = next_stones;
    }

    stones
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut stones: HashMap<usize, usize> = input
        .trim()
        .split(' ')
        .map(|s| (s.parse().unwrap(), 1))
        .collect();
    stones = blink(stones, 25);
    let mut count = 0;
    for (_, c) in stones {
        count += c;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut stones: HashMap<usize, usize> = input
        .trim()
        .split(' ')
        .map(|s| (s.parse().unwrap(), 1))
        .collect();
    stones = blink(stones, 75);
    let mut count = 0;
    for (_, c) in stones {
        count += c;
    }
    Some(count)
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
