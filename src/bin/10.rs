use std::collections::HashSet;

advent_of_code::solution!(10);

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
}

fn trail_score(pos: Pos, map: &Vec<Vec<u8>>) -> HashSet<Pos> {
    if map[pos.row][pos.col] == 9 {
        return HashSet::from([pos]);
    }

    let mut v = HashSet::new();
    let next = map[pos.row][pos.col] + 1;
    if pos.row > 0 && map[pos.row - 1][pos.col] == next {
        let mut pos = pos;
        pos.row -= 1;
        v = v.union(&trail_score(pos, map)).copied().collect();
    }
    if pos.row + 1 < map.len() && map[pos.row + 1][pos.col] == next {
        let mut pos = pos;
        pos.row += 1;
        v = v.union(&trail_score(pos, map)).copied().collect();
    }
    if pos.col > 0 && map[pos.row][pos.col - 1] == next {
        let mut pos = pos;
        pos.col -= 1;
        v = v.union(&trail_score(pos, map)).copied().collect();
    }
    if pos.col + 1 < map[0].len() && map[pos.row][pos.col + 1] == next {
        let mut pos = pos;
        pos.col += 1;
        v = v.union(&trail_score(pos, map)).copied().collect();
    }

    v
}

pub fn part_one(input: &str) -> Option<u32> {
    let input: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let mut ans = 0;

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row][col] == 0 {
                ans += trail_score(Pos::new(row, col), &input).len() as u32;
            }
        }
    }

    Some(ans)
}

fn trail_score_p2(row: usize, col: usize, map: &Vec<Vec<u8>>) -> u32 {
    if map[row][col] == 9 {
        return 1;
    }

    let mut v = 0;
    let next = map[row][col] + 1;
    if row > 0 && map[row - 1][col] == next {
        v += trail_score_p2(row - 1, col, map);
    }
    if row + 1 < map.len() && map[row + 1][col] == next {
        v += trail_score_p2(row + 1, col, map);
    }
    if col > 0 && map[row][col - 1] == next {
        v += trail_score_p2(row, col - 1, map);
    }
    if col + 1 < map[0].len() && map[row][col + 1] == next {
        v += trail_score_p2(row, col + 1, map);
    }

    v
}

pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let mut ans = 0;

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row][col] == 0 {
                ans += trail_score_p2(row, col, &input);
            }
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
