advent_of_code::solution!(7);

fn try_solve(prev: u128, mut vals: Vec<u128>, total: u128) -> bool {
    if prev > total {
        return false;
    }
    if vals.is_empty() {
        return prev == total;
    }

    let next = vals.remove(0);
    return try_solve(prev + next, vals.clone(), total) || try_solve(prev * next, vals, total);

}

pub fn part_one(input: &str) -> Option<u128> {
    let mut ans: u128 = 0;
    let input = input.trim();
    for line in input.lines() {
        let (total, vals) = line.split_once(": ").unwrap();
        let total: u128 = total.parse().unwrap();
        let mut vals: Vec<u128> = vals.split(' ').map(|v| v.parse().unwrap()).collect();
        let prev = vals.remove(0);

        if try_solve(prev, vals, total) {
            ans = ans.checked_add(total)?;
        }
    }

    Some(ans)
}

fn int_concat(x: u128, y: u128) -> u128 {
    let mut v = x.to_string();
    v.push_str(&y.to_string());
    v.parse().unwrap()
}

fn try_solve_p2(prev: u128, mut vals: Vec<u128>, total: u128) -> bool {
    if prev > total {
        return false;
    }
    if vals.is_empty() {
        return prev == total;
    }

    let next = vals.remove(0);
    return try_solve_p2(prev + next, vals.clone(), total) || try_solve_p2(prev * next, vals.clone(), total) || try_solve_p2(int_concat(prev, next), vals, total);

}

pub fn part_two(input: &str) -> Option<u128> {
    let mut ans: u128 = 0;
    let input = input.trim();
    for line in input.lines() {
        let (total, vals) = line.split_once(": ").unwrap();
        let total: u128 = total.parse().unwrap();
        let mut vals: Vec<u128> = vals.split(' ').map(|v| v.parse().unwrap()).collect();
        let prev = vals.remove(0);

        if try_solve_p2(prev, vals, total) {
            ans = ans.checked_add(total)?;
        }
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
