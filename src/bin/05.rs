use std::{
    cmp::Ordering,
    collections::HashMap,
};

advent_of_code::solution!(5);

fn valid_update(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut nono_list = vec![];
    for page in update {
        if nono_list.contains(page) {
            return false;
        }
        if let Some(v) = rules.get(page) {
            let mut v = v.clone();
            nono_list.append(&mut v);
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_input, updates) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut ans = 0;

    for line in rules_input.lines() {
        let before: u32 = line[0..2].parse().unwrap();
        let after: u32 = line[3..5].parse().unwrap();

        rules
            .entry(after)
            .and_modify(|v| v.push(before))
            .or_insert(vec![before]);
    }

    for update_line in updates.lines() {
        let update: Vec<u32> = (0..update_line.len())
            .step_by(3)
            .map(|i| update_line[i..i + 2].parse().unwrap())
            .collect();

        if valid_update(&update, &rules) {
            ans += update[update.len() / 2];
        }
    }

    Some(ans)
}

fn sort_update(x: &u32, y: &u32, rules: &HashMap<u32, Vec<u32>>) -> Ordering {
    if rules.get(x).is_some_and(|v| v.contains(y)) {
        Ordering::Greater
    } else if rules.get(y).is_some_and(|v| v.contains(x)) {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules_input, updates) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut ans = 0;

    for line in rules_input.lines() {
        let before: u32 = line[0..2].parse().unwrap();
        let after: u32 = line[3..5].parse().unwrap();

        rules
            .entry(after)
            .and_modify(|v| v.push(before))
            .or_insert(vec![before]);
    }

    for update_line in updates.lines() {
        let mut update: Vec<u32> = (0..update_line.len())
            .step_by(3)
            .map(|i| update_line[i..i + 2].parse().unwrap())
            .collect();
        if valid_update(&update, &rules) {
            continue;
        }
        update.sort_by(|x, y| sort_update(x, y, &rules));
        ans += update[update.len() / 2];
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
