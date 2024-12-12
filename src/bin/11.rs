use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..25 {
        let mut i = 0;
        loop {
            if i >= stones.len() {
                break;
            }
            let s = stones[i];
            if s == 0 {
                stones[i] = 1;
                i += 1;
                continue;
            }
            let len = s.ilog10() + 1;
            if len % 2 == 0 {
                let f = 10_usize.pow(len / 2);
                stones[i] = s % f;
                stones.insert(i, s / f);
                i += 2;
                continue;
            }

            stones[i] *= 2024;
            i += 1;
        }
    }

    Some(stones.len() as u32)
}

pub fn blink(s: usize, blinks: usize) -> Vec<usize> {
    let mut stones = vec![s];
    for _ in 0..blinks {
        let mut i = 0;
        loop {
            if i >= stones.len() {
                break;
            }
            let s = stones[i];
            if s == 0 {
                stones[i] = 1;
                i += 1;
                continue;
            }
            let len = s.ilog10() + 1;
            if len % 2 == 0 {
                let f = 10_usize.pow(len / 2);
                stones[i] = s % f;
                stones.insert(i, s / f);
                i += 2;
                continue;
            }

            stones[i] *= 2024;
            i += 1;
        }
    }

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
            .or_insert_with(|| blink(stone, 25))
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
                    .or_insert_with(|| blink(stone, 25))
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
                        .or_insert_with(|| blink(stone, 25))
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
        assert_eq!(result, None);
    }
}
