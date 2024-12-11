use rayon::prelude::*;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let stones: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let ans: u32 = stones
        .par_iter()
        .map(|&s| {
            let mut stones = vec![s];
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
            println!("Stone {s} resulted in {} stones", stones.len());
            stones.len() as u32
        })
        .sum();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
