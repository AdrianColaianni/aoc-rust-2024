use rayon::prelude::*;
use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn new(x: &Pos, y: &Pos) -> Self {
        let rd = x.row as isize - y.row as isize;
        let cd = x.col as isize - y.col as isize;
        if rd < 0 {
            Dir::Up
        } else if rd > 0 {
            Dir::Down
        } else if cd < 0 {
            Dir::Left
        } else {
            Dir::Right
        }
    }

    fn next(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn move_pos(&self, mut pos: Pos) -> Pos {
        match self {
            Dir::Up => pos.row -= 1,
            Dir::Right => pos.col += 1,
            Dir::Down => pos.row += 1,
            Dir::Left => pos.col -= 1,
        }
        pos
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    row: usize,
    col: usize,
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

// More efficient that derive(Hash)
impl std::hash::Hash for Pos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let l = self.col.checked_ilog10().map(|v| v + 1).unwrap_or(0);
        (self.row * 10_usize.pow(l) + self.col).hash(state);
    }
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn in_bounds(&self, bounds: &Pos) -> bool {
        self.row < bounds.row - 1 && self.col < bounds.col - 1 && self.row != 0 && self.col != 0
    }
}

fn serialize(input: &str) -> (Vec<Vec<bool>>, Pos) {
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c != '#').collect())
        .collect();
    let loc = input.find('^').unwrap();
    let pos = Pos::new(loc / map[0].len() - 1, loc % (map[0].len() + 1));
    (map, pos)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, mut pos) = serialize(input);
    let mut next_pos;
    let lim = Pos::new(map.len(), map[0].len());
    let mut dir = Dir::Up;
    let mut spots = HashSet::new();

    loop {
        next_pos = dir.move_pos(pos);
        if !map[next_pos.row][next_pos.col] {
            dir.next();
            continue;
        }
        // Invalid guard position
        if !next_pos.in_bounds(&lim) {
            break;
        }
        spots.insert(pos);
        pos = next_pos;
    }

    Some(spots.len() as u32 + 2)
}

fn visisted_spots(map: &Vec<Vec<bool>>, mut pos: Pos) -> Vec<Pos> {
    let lim = Pos::new(map.len(), map[0].len());
    let mut dir = Dir::Up;
    let mut spots = vec![];

    loop {
        let next_pos = dir.move_pos(pos);
        if !map[next_pos.row][next_pos.col] {
            // Cannot move forward
            dir.next();
            continue;
        }
        // Invalid guard position
        if !next_pos.in_bounds(&lim) {
            spots.push(pos);
            spots.push(next_pos);
            break;
        }
        if !spots.contains(&pos) {
            spots.push(pos);
        }
        pos = next_pos;
    }

    spots
}

fn map_loops(map: &Vec<Vec<bool>>, mut pos: Pos, mut dir: Dir) -> bool {
    let lim = Pos::new(map.len(), map[0].len());
    let start_dir = dir;
    let start_pos = pos;
    let mut tries = 7000;
    let mut spots = HashSet::new();

    loop {
        let next_pos = dir.move_pos(pos);
        if !map[next_pos.row][next_pos.col] {
            // Cannot move forward
            dir.next();
            continue;
        }
        // Invalid guard position
        if !next_pos.in_bounds(&lim) {
            return false;
        }
        if dir == start_dir && next_pos == start_pos {
            // If we've been in the same place facing the same direction,
            // we've looped
            return true;
        }
        if tries == 0 {
            if !spots.insert((pos, dir)) {
                return true;
            }
        } else {
            tries -= 1;
        }
        pos = next_pos;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, pos) = serialize(input);
    let vis = visisted_spots(&map, pos);

    let ans = vis
        .windows(2)
        .par_bridge()
        .filter(|vis| {
            let start = vis[0];
            let obstacle = vis[1];
            let dir = Dir::new(&obstacle, &start);
            let mut map = map.clone();
            map[obstacle.row][obstacle.col] = false;
            if map_loops(&map, start, dir) {
                return true;
            }

            false
        })
        .count();

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
