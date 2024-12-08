use std::{collections::{HashMap, HashSet}, hash::Hash};
// use rayon::prelude::*;

advent_of_code::solution!(8);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    row: usize,
    col: usize,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

// More efficient that derive(Hash)
impl Hash for Pos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let l = self.col.checked_ilog10().map(|v| v + 1).unwrap_or(0);
        (self.row * 10_usize.pow(l) + self.col).hash(state);
    }
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn add(&self, other: &Self) -> Option<Self> {
        let rd: isize = other.row as isize - self.row as isize;
        let cd: isize = other.col as isize - self.col as isize;
        self.add_delta(rd * 2, cd * 2)
    }

    fn in_bounds(&self, bounds: &Pos) -> bool {
        self.row < bounds.row && self.col < bounds.col
    }

    fn add_delta(&self, rd: isize, cd: isize) -> Option<Self> {
        Some(Self {
            row: (self.row as isize).checked_add(rd)?.try_into().ok()?,
            col: (self.col as isize).checked_add(cd)?.try_into().ok()?,
        })
    }
}

fn serialize(input: &str) -> (HashMap<char, Vec<Pos>>, Pos) {
    let mut set: HashMap<char, Vec<Pos>> = HashMap::new();
    let col_lim = input.find('\n').unwrap();
    let row_lim = input.matches('\n').count();
    let bounds = Pos::new(row_lim, col_lim);

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                let p = Pos::new(row, col);
                set.entry(c).and_modify(|v| v.push(p)).or_insert(vec![p]);
            }
        }
    }

    (set, bounds)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut ans = vec![];
    let (set, bounds) = serialize(input);

    for v in set.values() {
        for (i, first) in v.iter().enumerate() {
            for (_, second) in v.iter().enumerate().filter(|(j, _)| *j != i) {
                if let Some(res) = first.add(second) {
                    if res.in_bounds(&bounds) {
                        ans.push(res);
                    }
                }
                if let Some(res) = second.add(first) {
                    if res.in_bounds(&bounds) {
                        ans.push(res);
                    }
                }
            }
        }
    }
    ans.sort();
    ans.dedup();

    Some(ans.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ans = HashSet::new();
    let (set, bounds) = serialize(input);

    for v in set.values() {
        for (i, first) in v.iter().enumerate() {
            for (_, second) in v.iter().enumerate().filter(|(j, _)| *j != i) {
                // First dir
                let rd: isize = second.row as isize - first.row as isize;
                let cd: isize = second.col as isize - first.col as isize;

                let mut next = first.add_delta(rd, cd);
                let mut i = 2;

                while let Some(ne) = next {
                    if !ne.in_bounds(&bounds) {
                        break;
                    }
                    ans.insert(ne);
                    next = first.add_delta(rd * i, cd * i);
                    i += 1;
                }

                // Second dir
                let rd: isize = first.row as isize - second.row as isize;
                let cd: isize = first.col as isize - second.col as isize;

                let mut next = first.add_delta(rd, cd);
                let mut i = 2;

                while let Some(ne) = next {
                    if !ne.in_bounds(&bounds) {
                        break;
                    }
                    ans.insert(ne);
                    next = first.add_delta(rd * i, cd * i);
                    i += 1;
                }
            }
        }
    }

    Some(ans.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
