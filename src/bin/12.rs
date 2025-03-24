use std::collections::HashSet;
// use rayon::prelude::*;

advent_of_code::solution!(12);

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

    fn nearby(&self, limits: &Pos) -> Vec<Self> {
        let mut v = vec![];

        if self.row != 0 {
            v.push(Pos::new(self.row - 1, self.col));
        }
        if self.col != 0 {
            v.push(Pos::new(self.row, self.col - 1));
        }
        if self.row + 1 != limits.row {
            v.push(Pos::new(self.row + 1, self.col));
        }
        if self.col + 1 != limits.col {
            v.push(Pos::new(self.row, self.col + 1));
        }

        v
    }

    fn nearby_unbounded(&self) -> Vec<Self> {
        let mut v = vec![
            Pos::new(self.row + 1, self.col),
            Pos::new(self.row, self.col + 1),
        ];
        if self.row != 0 {
            v.push(Pos::new(self.row - 1, self.col));
        }
        if self.col != 0 {
            v.push(Pos::new(self.row, self.col - 1));
        }
        v
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn_right(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn turn_left(&mut self) {
        *self = match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }

    fn move_forward(&self, mut pos: Pos, bounds: &Pos) -> Option<Pos> {
        match self {
            Dir::Up => pos.row = pos.row.checked_sub(1)?,
            Dir::Right => {
                pos.col += 1;
                if pos.col == bounds.col {
                    return None;
                }
            }
            Dir::Down => {
                pos.row += 1;
                if pos.row == bounds.row {
                    return None;
                }
            }
            Dir::Left => pos.col = pos.col.checked_sub(1)?,
        }
        Some(pos)
    }

    fn wrap_left(&self, mut pos: Pos, bounds: &Pos) -> Option<Pos> {
        match self {
            Dir::Up => {
                pos.col = pos.col.checked_sub(1)?;
                pos.row = pos.row.checked_sub(1)?;
            }
            Dir::Right => {
                pos.row = pos.row.checked_sub(1)?;
                pos.col += 1;
                if pos.col == bounds.col {
                    return None;
                }
            }
            Dir::Down => {
                pos.col += 1;
                pos.row += 1;
                if pos.col == bounds.col || pos.row == bounds.row {
                    return None;
                }
            }
            Dir::Left => {
                pos.col = pos.col.checked_sub(1)?;
                pos.row += 1;
                if pos.row == bounds.row {
                    return None;
                }
            }
        }
        Some(pos)
    }

    fn move_left(&self, mut pos: Pos, bounds: &Pos) -> Option<Pos> {
        match self {
            Dir::Up => {
                pos.col = pos.col.checked_sub(1)?;
            }
            Dir::Right => {
                pos.row = pos.row.checked_sub(1)?;
            }
            Dir::Down => {
                pos.col += 1;
                if pos.col == bounds.col {
                    return None;
                }
            }
            Dir::Left => {
                pos.row += 1;
                if pos.row == bounds.row {
                    return None;
                }
            }
        }
        Some(pos)
    }
}

fn find_plot(input: &Vec<Vec<char>>, pos: Pos, bounds: &Pos) -> Vec<Pos> {
    let mut area = vec![pos];
    let c = input[pos.row][pos.col];

    let mut i = 0;
    loop {
        if i >= area.len() {
            break;
        }
        for pos in area[i].nearby(bounds) {
            if input[pos.row][pos.col] == c && !area.contains(&pos) {
                area.push(pos);
            }
        }
        i += 1;
    }

    area
}

struct Region {
    size: usize,
    sides: usize,
}

impl Region {
    fn new(area: Vec<Pos>, sides: usize) -> Self {
        Self {
            size: area.len(),
            sides,
        }
    }
}

fn bounds(area: &Vec<Pos>) -> usize {
    let mut total = 0;
    for pos in area {
        let mut sides = 4;
        for near in pos.nearby_unbounded() {
            if area.contains(&near) {
                sides -= 1;
            }
        }
        total += sides;
    }

    total
}

pub fn part_one(input: &str) -> Option<u32> {
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let limit = Pos::new(input.len(), input[0].len());

    let mut ignore: HashSet<Pos> = HashSet::new();
    let mut total = 0;

    for row in 0..limit.row {
        for col in 0..limit.col {
            let pos = Pos::new(row, col);
            if !ignore.insert(pos) {
                continue;
            }
            let area = find_plot(&input, pos, &limit);
            for pos in &area {
                ignore.insert(*pos);
            }
            total += area.len() * bounds(&area);
        }
    }

    Some(total.try_into().unwrap())
}

fn on_boarder(input: &Vec<Vec<char>>, pos: &Pos, dir: Dir, limits: &Pos) -> bool {
    let c = input[pos.row][pos.col];
    match dir {
        Dir::Up => pos.col == 0 || c != input[pos.row][pos.col - 1],
        Dir::Right => pos.row == 0 || c != input[pos.row - 1][pos.col],
        Dir::Down => pos.col + 1 == limits.col || c != input[pos.row][pos.col + 1],
        Dir::Left => pos.row + 1 == limits.col || c != input[pos.row + 1][pos.col],
    }
}

// We make a little guy who walks around and counts the sides
// Return sides and corners on sides
fn bounds_p2(input: &Vec<Vec<char>>, area: &Vec<Pos>, limits: &Pos) -> usize {
    if area.len() == 1 {
        // My program cannot handle size 1, so we hard code :)
        return 4;
    } else if area.len() == 2 {
        return 4;
    }
    let mut visited: HashSet<(Pos, Dir)> = HashSet::new();

    let mut turns = 0;
    for start in area {
        for start_dir in [Dir::Left, Dir::Right, Dir::Up, Dir::Down] {
            let mut them_turns = 0;
            let mut pos = start.clone();
            let mut dir = start_dir;
            if visited.contains(&(pos, dir)) || !on_boarder(input, &pos, dir, limits) {
                continue;
            }
            visited.insert((pos, dir));
            loop {
                if them_turns != 0 && *start == pos && start_dir == dir {
                    break;
                }
                // if them_turns == 100 {
                //     panic!("Exceeded turn limit");
                // }


                if let Some(next_pos) = dir.wrap_left(pos, limits) {
                    let mut ndir = dir;
                    ndir.turn_left();
                    let border = ndir.move_left(next_pos, limits);
                    let border = border.is_none()
                        || border.is_some_and(|p| input[p.row][p.col] != input[pos.row][pos.col]);
                    if area.contains(&next_pos) && border {
                        dir.turn_left();
                        them_turns += 1;
                        pos = next_pos;
                        if !visited.insert((pos, dir)) && *start != pos && start_dir != dir {
                            them_turns = 0;
                            break;
                        }
                        continue;
                    }
                }
                if let Some(next_pos) = dir.move_forward(pos, limits) {
                    let border = dir.move_left(next_pos, limits);
                    let border = border.is_none()
                        || border.is_some_and(|p| input[p.row][p.col] != input[pos.row][pos.col]);
                    if area.contains(&next_pos) && border {
                        pos = next_pos;
                        if !visited.insert((pos, dir)) && *start != pos && start_dir != dir {
                            them_turns = 0;
                            break;
                        }
                        continue;
                    }
                }
                // At dead end, turn around
                dir.turn_right();
                them_turns += 1;
            }
            turns += them_turns;
        }
    }

    turns
}

pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let limits = Pos::new(input.len(), input[0].len());

    let mut ignore: HashSet<Pos> = HashSet::new();
    let mut regions: Vec<Region> = vec![];

    for row in 0..limits.row {
        for col in 0..limits.col {
            let pos = Pos::new(row, col);
            if !ignore.insert(pos) {
                continue;
            }
            let area = find_plot(&input, pos, &limits);
            for pos in &area {
                ignore.insert(*pos);
            }
            let sides = bounds_p2(&input, &area, &limits);
            let region = Region::new(area, sides);
            regions.push(region);
        }
    }

    let total: usize = regions
        // .par_iter()
        .iter()
        .map(|a| a.size * a.sides)
        .sum();

    Some(total.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
