use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_input, updates) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut ans = 0;

    for line in rules_input.lines() {
        // println!("{} -> {}", &line[0..2], &line[3..5]);
        let before: u32 = line[0..2].parse().unwrap();
        let after: u32 = line[3..5].parse().unwrap();

        rules
            .entry(after)
            .and_modify(|v| v.push(before))
            .or_insert(vec![before]);
    }
    // println!("{:?}", rules);

    'update: for update in updates.lines() {
        let update: Vec<u32> = update.split(',').map(|v| v.parse().unwrap()).collect();
        let mut nono_list = vec![];
        for page in &update {
            if nono_list.contains(page) {
                continue 'update;
            }
            if let Some(v) = rules.get(page) {
                let mut v = v.clone();
                nono_list.append(&mut v);
            }
        }
        ans += update[update.len() / 2];
    }

    Some(ans)
}

// If update is valid, return none, otherwise the index of the invalid page
fn valid_update(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> Option<(usize, usize)> {
    let mut nono_list = vec![];
    let mut reason = HashMap::new();
    for (i, page) in update.iter().enumerate() {
        if nono_list.contains(page) {
            return Some((i, *reason.get(page).unwrap()));
        }
        if let Some(v) = rules.get(page) {
            for v in v {
                if reason.get(v).is_none() {
                    reason.insert(v, i);
                }
            }
            let mut v = v.clone();
            nono_list.append(&mut v);
        }
    }

    None
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

    for update in updates.lines() {
        let mut update: Vec<u32> = update.split(',').map(|v| v.parse().unwrap()).collect();
        if valid_update(&update, &rules).is_none() {
            continue;
        }
        while let Some((i, j)) = valid_update(&update, &rules) {
            update.swap(i, j);
        }
        ans += update[update.len()/2];
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
