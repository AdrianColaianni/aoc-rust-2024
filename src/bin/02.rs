advent_of_code::solution!(2);

fn safe_report(l: &Vec<i32>) -> bool {
    if l[0] < l[1] {
        // Pattern increasing
        for w in l.windows(2) {
            let d = w[1] - w[0];
            if d < 1 || d > 3 {
                return false;
            }
        }
    } else {
        // Pattern decreasing
        for w in l.windows(2) {
            let d = w[0] - w[1];
            if d < 1 || d > 3 {
                return false;
            }
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe = 0;

    for l in input.trim().split('\n') {
        let l: Vec<i32> = l.split(' ').map(|v| v.parse().unwrap()).collect();

        if safe_report(&l) {
            safe += 1;
        }
    }

    Some(safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe = 0;

    'rep: for l in input.trim().split('\n') {
        let l: Vec<i32> = l.split(' ').map(|v| v.parse().unwrap()).collect();

        if safe_report(&l) {
            safe += 1;
            continue;
        }

        for i in 0..l.len() {
            let mut c = l.clone();
            c.remove(i);
            if safe_report(&c) {
                safe += 1;
                continue 'rep;
            }
        }

    }

    Some(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
