use std::collections::HashSet;
use rayon::prelude::*;

advent_of_code::solution!(6);

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_pos(&self, pos: (usize, usize)) -> (usize, usize) {
        let mut pos = pos;
        match self {
            Direction::Up => pos.0 -= 1,
            Direction::Right => pos.1 += 1,
            Direction::Down => pos.0 += 1,
            Direction::Left => pos.1 -= 1,
        }
        pos
    }
}

fn serialize(input: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c != '#').collect())
        .collect();
    let loc = input.find('^').unwrap();
    let pos = (loc / map[0].len() - 1, loc % (map[0].len() + 1));
    (map, pos)
}

fn valid_guard_pos(x: (usize, usize), lim: (usize, usize)) -> bool {
    // The edges of the map are not valid positions
    x.0 < lim.0 - 1 && x.1 < lim.1 - 1 && x.0 != 0 && x.1 != 0
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, mut pos) = serialize(input);
    let mut next_pos;
    let lim = (map.len(), map[0].len());
    let mut dir = Direction::Up;
    let mut spots = HashSet::new();

    loop {
        next_pos = dir.move_pos(pos);
        // Cannot move forward
        if !map[next_pos.0][next_pos.1] {
            dir.next();
            continue;
        }
        // Invalid guard position
        if !valid_guard_pos(next_pos, lim) {
            break;
        }
        spots.insert(pos);
        pos = next_pos;
    }

    Some(spots.len() as u32 + 2)
}

fn visisted_spots(map: &Vec<Vec<bool>>, mut pos: (usize, usize)) -> Vec<(usize, usize)> {
    let lim = (map.len(), map[0].len());
    let mut dir = Direction::Up;
    let mut spots = HashSet::new();

    loop {
        let next_pos = dir.move_pos(pos);
        if !map[next_pos.0][next_pos.1] {
            // Cannot move forward
            dir.next();
            continue;
        }
        // Invalid guard position
        if !valid_guard_pos(next_pos, lim) {
            spots.insert(pos);
            spots.insert(next_pos);
            break;
        }
        spots.insert(pos);
        pos = next_pos;
    }

    spots.into_iter().collect()
}

pub fn map_loops(map: &Vec<Vec<bool>>, mut pos: (usize, usize)) -> bool {
    let lim = (map.len(), map[0].len());
    let mut dir = Direction::Up;
    let mut spots = HashSet::new();

    loop {
        let next_pos = dir.move_pos(pos);
        if !map[next_pos.0][next_pos.1] {
            // Cannot move forward
            dir.next();
            continue;
        }
        // Invalid guard position
        if !valid_guard_pos(next_pos, lim) {
            return false;
        }
        if !spots.insert((pos, dir)) {
            // If we've been in the same place facing the same direction,
            // we've looped
            return true;
        }
        pos = next_pos;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, pos) = serialize(input);
    let vis = visisted_spots(&map, pos);

    let ans = vis.par_iter().filter(|(row, col)| {
        if (*row, *col) == pos {
            return false;
        }
        let mut map = map.clone();
        map[*row][*col] = false;
        if map_loops(&map, pos) {
            return true;
        }

        false
    }).count();

    Some(ans as u32)
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
