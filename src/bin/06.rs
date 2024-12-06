use core::panic;
use std::collections::HashSet;

advent_of_code::solution!(6);

fn serialize(input: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let input = input.lines();
    let mut map = vec![];
    let mut pos = (0, 0);

    for (row, line) in input.enumerate() {
        map.push(vec![]);
        for (col, char) in line.chars().enumerate() {
            map[row].push(char != '#');
            if char == '^' {
                pos = (row, col);
            }
        }
    }

    (map, pos)
}

fn add_pos(x: (usize, usize), y: (i32, i32)) -> (usize, usize) {
    ((x.0 as i32 + y.0) as usize, (x.1 as i32 + y.1) as usize)
}

fn valid_pos(x: (usize, usize), lim: (usize, usize)) -> bool {
    x.0 < lim.0 && x.1 < lim.1
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, mut pos) = serialize(input);
    let lim = (map.len(), map[0].len());
    let mut dir = (-1, 0); // Facing up
    let mut spots = HashSet::new();
    let mut tries = lim.0 * lim.1;

    loop {
        let next_pos = add_pos(pos, dir);
        if !valid_pos(next_pos, lim) {
            spots.insert(pos);
            break;
        }
        if !map[next_pos.0][next_pos.1] {
            // Cannot move forward
            dir = match dir {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => panic!("Position"),
            };
            continue;
        }
        spots.insert(pos);
        tries -= 1;
        if tries == 0 {
            return None;
        }
        pos = next_pos;
    }

    Some(spots.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans = 0;

    for (i, c) in input.chars().enumerate() {
        if c != '.' {
            continue;
        }
        // println!("Try {}", i);
        let mut test = input.to_string();
        test.replace_range(i..i+1, "#");
        if part_one(&test).is_none() {
            ans += 1;
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
