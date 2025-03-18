use std::collections::BinaryHeap;

advent_of_code::solution!(16);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dir::North => write!(f, "^"),
            Dir::East => write!(f, ">"),
            Dir::South => write!(f, "v"),
            Dir::West => write!(f, "<"),
        }
    }
}

impl Dir {
    pub fn left(&self) -> Self {
        match self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
}

#[derive(Eq)]
struct Pos {
    r: usize,
    c: usize,
    dir: Dir,
    cost: usize,
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.c, self.dir, self.cost)
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        // self.cost.eq(&other.cost)
        self.r == other.r && self.c == other.c
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl Pos {
    pub fn new(r: usize, c: usize, dir: Dir, cost: usize) -> Self {
        Self { r, c, dir, cost }
    }

    pub fn options(&self, map: &Vec<Vec<char>>) -> Vec<Self> {
        let mut options = vec![];
        let Self { r, c, dir, cost } = self;

        // Go North
        if *dir != Dir::South && *r != 0 && map[r - 1][*c] != '#' {
            let mut cost = cost + 1;
            if *dir != Dir::North {
                cost += 1000;
            }
            options.push(Self::new(r - 1, *c, Dir::North, cost));
        }

        // Go East
        if *dir != Dir::West && c + 1 != map[*r].len() && map[*r][c + 1] != '#' {
            let mut cost = cost + 1;
            if *dir != Dir::East {
                cost += 1000;
            }
            options.push(Self::new(*r, c + 1, Dir::East, cost));
        }

        // Go South
        if *dir != Dir::North && r + 1 != map.len() && map[r + 1][*c] != '#' {
            let mut cost = cost + 1;
            if *dir != Dir::South {
                cost += 1000;
            }
            options.push(Self::new(r + 1, *c, Dir::South, cost));
        }

        // Go West
        if *dir != Dir::East && *c != 0 && map[*r][c - 1] != '#' {
            let mut cost = cost + 1;
            if *dir != Dir::West {
                cost += 1000;
            }
            options.push(Self::new(*r, c - 1, Dir::West, cost));
        }
        /*
        // Go forward
        let f = self.forward();
        if map[f.0][f.1] != '#' {
            options.push(Self::new(f.0, f.1, *dir, cost + 1));
        }
        // Turn left
        options.push(Self::new(*r, *c, dir.left(), cost + 1000));
        // Turn right
        options.push(Self::new(*r, *c, dir.right(), cost + 1000));
        */

        options
    }

    fn forward(&self) -> (usize, usize) {
        let Self { r, c, dir, .. } = self;
        match dir {
            Dir::North => (r - 1, *c),
            Dir::East => (*r, c + 1),
            Dir::South => (r + 1, *c),
            Dir::West => (*r, c - 1),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    // for row in &map {
    //     for c in row {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    let end = Pos::new(1, map[0].len() - 2, Dir::North, 0);
    println!("End: {}", map[1][map[0].len() - 2]);

    let mut heap = BinaryHeap::new();

    let mut dist: Vec<Vec<Option<usize>>> = (0..map.len())
        .map(|_| (0..map[0].len()).map(|_| None).collect())
        .collect();

    // Add start
    heap.push(Pos::new(map.len() - 2, 1, Dir::East, 0));
    println!("Start: {}", map[map.len() - 2][1]);
    dist[map.len() - 2][1] = Some(0);

    println!("Heap {:?}", heap);
    while let Some(pos) = heap.pop() {
        if pos == end {
            // Print map
            for r in 0..map.len() {
                for c in 0..map[0].len() {
                    if let Some(c) = dist[r][c] {
                        print!("{}", c % 10);
                    } else {
                        print!("{}", map[r][c]);
                    }
                }
                println!();
            }
            return Some(pos.cost);
        }
        if dist[pos.r][pos.c].is_some_and(|c| pos.cost > c) {
            continue;
        }

        for next in pos.options(&map) {
            if dist[next.r][next.c].is_none_or(|c| next.cost < c) {
                dist[next.r][next.c] = Some(next.cost);
                heap.push(next);
            }
        }
        println!("Heap {:?}", heap);
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
