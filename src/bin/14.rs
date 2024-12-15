advent_of_code::solution!(14);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    row: isize,
    col: isize,
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
        (self.row * 10_isize.pow(l) + self.col).hash(state);
    }
}

impl Pos {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn add_vel(&mut self, vel: &Pos, lim: &Pos) {
        self.row += vel.row;
        if self.row < 0 {
            self.row = lim.row + self.row;
        } else if self.row >= lim.row {
            self.row = self.row % lim.row;
        }
        self.col += vel.col;
        if self.col < 0 {
            self.col = lim.col + self.col;
        } else if self.col >= lim.col {
            self.col = self.col % lim.col;
        }
    }
}

fn build_bot(input: &str) -> (Pos, Pos) {
    let mut input = input.split(' ');
    let pos = input.next().unwrap();
    let ps = pos.find(',').unwrap();
    let vel = input.next().unwrap();
    let vs = vel.find(',').unwrap();
    let pos = Pos::new(pos[ps+1..].parse().unwrap(), pos[2..ps].parse().unwrap());
    let vel = Pos::new(vel[vs+1..].parse().unwrap(), vel[2..vs].parse().unwrap());
    (pos, vel)
}

pub fn part_one(input: &str) -> Option<u32> {
    let limits = Pos::new(103, 101);
    let rs = (limits.row-1)/2;
    let cs = (limits.col-1)/2;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;


    for line in input.trim().lines() {
        let (mut bot, vel) = build_bot(line);
        // println!("{:?} {:?}", bot, vel);

        for _ in 0..100 {
            bot.add_vel(&vel, &limits);
            // println!("{}: {:?}", i+1, bot);
        }

        if bot.row < rs && bot.col < cs {
            q1 += 1;
        } else if bot.row < rs && bot.col > cs {
            q2 += 1;
        } else if bot.row > rs && bot.col < cs {
            q3 += 1;
        } else if bot.row > rs && bot.col > cs {
            q4 += 1;
        }
    }

    Some(q1 * q2 * q3 * q4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let limits = Pos::new(103, 101);
    let mut bots = vec![];

    for line in input.trim().lines() {
        bots.push(build_bot(line));
    }

    let mut s = 0;
    loop {
        s += 1;
        let mut board = [[0; 101]; 103];
        let mut all_ones = true;

        // Move bots
        for (bot, vel) in &mut bots {
            bot.add_vel(vel, &limits);
            board[bot.row as usize][bot.col as usize] += 1;
            if board[bot.row as usize][bot.col as usize] == 2 {
                all_ones = false;
            }
        }

        if all_ones {
            break;
        }
    }

    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    // Part 2 cannot be tested
}
