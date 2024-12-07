advent_of_code::solution!(7);

use rayon::prelude::*;

fn try_solve(prev: u64, vals: &Vec<u64>, mut i: usize, total: u64) -> bool {
    if i == vals.len() {
        return prev == total;
    }

    let next = vals[i];
    i += 1;

    let add = prev + next;
    let mul = prev * next;

    if i == vals.len() {
        return add == total || mul == total;
    }
    match (add > total, mul > total) {
        (true, true) => false,
        (true, false) => try_solve(mul, vals, i, total),
        (false, true) => try_solve(add, vals, i, total),
        (false, false) => try_solve(add, vals, i, total) || try_solve(mul, vals, i, total),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = input.trim();
    let ans = input.par_lines().map(|line| {
        let (total, vals) = line.split_once(": ").unwrap();
        let total: u64 = total.parse().unwrap();
        let vals: Vec<u64> = vals.split(' ').map(|v| v.parse().unwrap()).collect();

        if try_solve(vals[0], &vals, 1, total) {
            total
        } else {
            0
        }
    }).sum();

    Some(ans)
}

fn int_concat(x: u64, y: u64) -> u64 {
    let l = y.ilog10() + 1;
    x * 10_u64.pow(l) + y
}

fn try_solve_p2(prev: u64, vals: &Vec<u64>, mut i: usize, total: u64) -> bool {
    if prev > total {
        return false;
    }
    if i == vals.len() {
        return prev == total;
    }

    let next = vals[i];
    i += 1;

    let add = prev + next;
    let mul = prev * next;
    let cat = int_concat(prev, next);

    if i == vals.len() {
        return add == total || mul == total || cat == total;
    }
    return (add <= total && try_solve_p2(add, vals, i, total))
        || (mul <= total && try_solve_p2(mul, vals, i, total))
        || (cat <= total && try_solve_p2(cat, vals, i, total));
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input.trim();
    let ans = input.par_lines().map(|line| {
        let (total, vals) = line.split_once(": ").unwrap();
        let total: u64 = total.parse().unwrap();
        let vals: Vec<u64> = vals.split(' ').map(|v| v.parse().unwrap()).collect();

        if try_solve_p2(vals[0], &vals, 1, total) {
            total
        } else {
            0
        }
    }).sum();

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
