use std::collections::BinaryHeap;

advent_of_code::solution!(16);

type Map = Vec<Vec<char>>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dir {
    Forward,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Pos {
    r: usize,
    c: usize,
    o: usize, // Orientation: 0: North, 1: East, 2: South, 3: West
    cost: usize,
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) {} {}", self.r, self.c, self.o, self.cost)
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
        Self { r, c, o: 1, cost }
    }

    fn turn_left(&self) -> Self {
        let mut r = self.clone();
        r.o = r.o.wrapping_sub(1) % 4;
        r.cost += 1000;
        r
    }

    fn turn_right(&self) -> Self {
        let mut r = self.clone();
        r.o = r.o.wrapping_add(1) % 4;
        r.cost += 1000;
        r
    }

    fn try_forward(&self, map: &Map) -> Option<Self> {
        let Self {
            mut r,
            mut c,
            o,
            mut cost,
        } = self.clone();

        match o {
            0 => r -= 1, // North
            1 => c += 1, // East
            2 => r += 1, // South
            _ => c -= 1, // West
        }

        if map[r][c] == '#' {
            return None;
        }

        cost += 1;

        Some(Self { r, c, o, cost })
    }

    pub fn options(&self, map: &Map) -> Vec<Self> {
        let mut op = vec![];
        op.push(self.turn_left());
        op.push(self.turn_right());
        if let Some(f) = self.try_forward(map) {
            op.push(f);
        }
        op
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Map = input
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let size = map.len();

    let mut cost: Vec<Vec<[usize; 4]>> = (0..size)
        .map(|_| (0..size).map(|_| [usize::MAX; 4]).collect())
        .collect();
    cost[size - 2][1][1] = 0;

    let mut heap: BinaryHeap<Pos> = BinaryHeap::new();
    heap.push(Pos::new(size - 2, 1, 0));

    while let Some(pos) = heap.pop() {
        // println!("{:?}", heap);
        if pos.r == 1 && pos.c == size - 2 {
            // for r in 0..size {
            //     for c in 0..size {
            //         if cost[r][c] != usize::MAX {
            //             print!("{}", cost[r][c] % 10);
            //         } else {
            //             print!("{}", map[r][c]);
            //         }
            //     }
            //     println!();
            // }
            return Some(pos.cost);
        }

        if pos.cost > cost[pos.r][pos.c][pos.o] {
            continue;
        }

        for next in pos.options(&map) {
            if next.cost < cost[next.r][next.c][next.o] {
                heap.push(next);
                cost[next.r][next.c][next.o] = next.cost;
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
