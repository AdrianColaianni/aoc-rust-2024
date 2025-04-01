use std::{collections::BinaryHeap, sync::OnceLock};

advent_of_code::solution!(18);

type Map = Vec<Vec<bool>>;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    r: usize,
    c: usize,
    cost: usize,
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) {}", self.r, self.c, self.cost)
    }
}

impl std::cmp::PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl std::cmp::Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl Pos {
    pub fn new(r: usize, c: usize, cost: usize) -> Self {
        Self { r, c, cost }
    }

    fn try_forward(&self, dir: isize, map: &Map) -> Option<Self> {
        let Self {
            mut r,
            mut c,
            mut cost,
        } = self.clone();

        match dir {
            0 => r = r.checked_sub(1)?, // North
            1 => c += 1,                // East
            2 => r += 1,                // South
            _ => c = c.checked_sub(1)?, // West
        }

        if !*map.get(r)?.get(c)? {
            return None;
        }

        cost += 1;

        Some(Self { r, c, cost })
    }

    pub fn options(&self, map: &Map) -> Vec<Self> {
        (0..4).filter_map(|d| self.try_forward(d, map)).collect()
    }
}

static SIZE: OnceLock<usize> = OnceLock::new();
static READ: OnceLock<usize> = OnceLock::new();

pub fn part_one(input: &str) -> Option<usize> {
    let size = *SIZE.get_or_init(|| 71);
    let read = *READ.get_or_init(|| 1024);
    let mut map = vec![vec![true; size]; size];
    let mut lines = input.lines();
    for _ in 0..read {
        let line = lines.next().unwrap();
        let (c, r) = line.split_once(",").unwrap();
        let r: usize = r.parse().unwrap();
        let c: usize = c.parse().unwrap();
        map[r][c] = false;
    }

    let mut cost: Vec<Vec<usize>> = (0..size)
        .map(|_| (0..size).map(|_| usize::MAX).collect())
        .collect();
    cost[0][0] = 0;

    let mut heap: BinaryHeap<Pos> = BinaryHeap::new();
    heap.push(Pos::new(0, 0, 0));

    while let Some(pos) = heap.pop() {
        if pos.r == size - 1 && pos.c == size - 1 {
            return Some(pos.cost);
        }

        if pos.cost > cost[pos.r][pos.c] {
            continue;
        }

        for next in pos.options(&map) {
            if next.cost < cost[next.r][next.c] {
                heap.push(next);
                cost[next.r][next.c] = next.cost;
            }
        }
    }

    None
}

fn valid_path(map: &Map) -> bool {
    let size = map.len();
    let mut cost: Vec<Vec<usize>> = (0..size)
        .map(|_| (0..size).map(|_| usize::MAX).collect())
        .collect();
    cost[0][0] = 0;

    let mut heap: BinaryHeap<Pos> = BinaryHeap::new();
    heap.push(Pos::new(0, 0, 0));

    while let Some(pos) = heap.pop() {
        if pos.r == size - 1 && pos.c == size - 1 {
            return true;
        }

        if pos.cost > cost[pos.r][pos.c] {
            continue;
        }

        for next in pos.options(&map) {
            if next.cost < cost[next.r][next.c] {
                heap.push(next);
                cost[next.r][next.c] = next.cost;
            }
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<String> {
    let size = *SIZE.get_or_init(|| 71);
    let lines: Vec<(usize, usize)> = input
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(c, r)| (r.parse().unwrap(), c.parse().unwrap()))
        .collect();

    // Binary search for the solution
    let mut low = 0;
    let mut high = lines.len() - 1;
    let mut mid = 0;
    let mut v = false;
    while low <= high {
        mid = (low + high) / 2;
        // Build map
        let mut map = vec![vec![true; size]; size];
        for (r, c) in &lines[0..=mid] {
            map[*r][*c] = false;
        }

        // Try path
        v = valid_path(&map);
        if !v {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    if v {
        mid += 1;
    }
    let ret = lines[mid];
    return Some(format!("{},{}", ret.1, ret.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        SIZE.get_or_init(|| 7);
        READ.get_or_init(|| 12);
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        SIZE.get_or_init(|| 7);
        READ.get_or_init(|| 12);
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_owned()));
    }
}
