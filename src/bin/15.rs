advent_of_code::solution!(15);

#[derive(Debug)]
struct Bot {
    row: usize,
    col: usize,
    row_lim: usize,
    col_lim: usize,
}

impl Bot {
    fn new(row: usize, col: usize, row_lim: usize, col_lim: usize) -> Self {
        Self {
            row,
            col,
            row_lim,
            col_lim,
        }
    }

    fn can_move(&self, dir: Dir) -> bool {
        match dir {
            Dir::Left => self.col != 0,
            Dir::Right => self.col < self.col_lim - 1,
            Dir::Up => self.row != 0,
            Dir::Down => self.row < self.row_lim - 1,
        }
    }

    fn move_pos(&self, dir: Dir) -> (usize, usize) {
        match dir {
            Dir::Left => (self.row, self.col - 1),
            Dir::Right => (self.row, self.col + 1),
            Dir::Up => (self.row - 1, self.col),
            Dir::Down => (self.row + 1, self.col),
        }
    }

    fn move_bot(&mut self, dir: Dir) {
        match dir {
            Dir::Left => self.col -= 1,
            Dir::Right => self.col += 1,
            Dir::Up => self.row -= 1,
            Dir::Down => self.row += 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Cell {
    Empty,
    Box,
    BoxLeft,
    BoxRight,
    Wall,
}

impl Cell {
    fn is_box(&self) -> bool {
        match self {
            Cell::Empty => false,
            Cell::Wall => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    Down,
    Left,
    Right,
    Up,
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::Down => "↓",
                Dir::Left => "←",
                Dir::Right => "→",
                Dir::Up => "↑",
            }
        )
    }
}

impl From<char> for Dir {
    fn from(value: char) -> Self {
        match value {
            'v' => Dir::Down,
            '<' => Dir::Left,
            '>' => Dir::Right,
            // While technically risky, this gives a significant speed boost
            _ => Dir::Up,
            // '^' => Dir::Up,
            // _ => panic!("Invalid pos: {}", value),
        }
    }
}

fn serialize_map(input: &str) -> (Vec<Vec<Cell>>, Bot) {
    let width = input.find('\n').unwrap();

    let mut map = vec![vec![Cell::Empty; width - 2]; width - 2];
    let mut bot = Bot::new(0, 0, width - 2, width - 2);

    let input: Vec<char> = input.chars().collect();
    for i in width + 1..input.len() - width {
        let col = i % (width + 1);
        if col == 0 || col == width - 1 {
            continue;
        }
        let row = i / (width + 1);
        match input[i] {
            'O' => map[row - 1][col - 1] = Cell::Box,
            '#' => map[row - 1][col - 1] = Cell::Wall,
            '@' => {
                bot.row = row - 1;
                bot.col = col - 1;
            }
            _ => (),
        }
    }

    (map, bot)
}

// fn print_map(map: &Vec<Vec<Cell>>, bot: &Bot, dir: Dir) {
//     for _ in 0..map[0].len() {
//         print!("#");
//     }
//     println!();
//     for (ri, row) in map.iter().enumerate() {
//         for (ci, cell) in row.iter().enumerate() {
//             if ri == bot.row && ci == bot.col {
//                 print!("{}", dir);
//             } else {
//                 match cell {
//                     Cell::Empty => print!("."),
//                     Cell::Box => print!("O"),
//                     Cell::BoxLeft => print!("["),
//                     Cell::BoxRight => print!("]"),
//                     Cell::Wall => print!("#"),
//                 }
//             }
//         }
//         println!();
//     }
//     // for _ in 0..map[0].len() {
//     //     print!("#");
//     // }
//     // println!();
// }

pub fn part_one(input: &str) -> Option<u32> {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let (mut map, mut bot) = serialize_map(map);
    // print_map(&map, &bot);
    // println!("{:?}", bot);
    // return None;
    'dir: for dir in moves.chars() {
        if dir == '\n' {
            continue;
        }
        let dir = Dir::from(dir);

        // print_map(&map, &bot);
        // println!("Moving bot {}", dir);

        if !bot.can_move(dir) {
            continue;
        }

        let (mut row, mut col) = bot.move_pos(dir);
        match map[row][col] {
            Cell::Empty => {
                bot.move_bot(dir);
                continue;
            }
            Cell::Wall => continue,
            _ => (),
        }

        while map[row][col] == Cell::Box {
            match dir {
                Dir::Left => {
                    if col == 0 {
                        continue 'dir;
                    }
                    col -= 1;
                }
                Dir::Right => {
                    if col == bot.col_lim - 1 {
                        continue 'dir;
                    }
                    col += 1;
                }
                Dir::Up => {
                    if row == 0 {
                        continue 'dir;
                    }
                    row -= 1;
                }
                Dir::Down => {
                    if row == bot.row_lim - 1 {
                        continue 'dir;
                    }
                    row += 1;
                }
            }
        }

        if map[row][col] == Cell::Empty {
            bot.move_bot(dir);
            map[row][col] = Cell::Box;
            map[bot.row][bot.col] = Cell::Empty;
        }
    }

    // print_map(&map, &bot);

    let mut total = 0;
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == Cell::Box {
                total += (r + 1) * 100 + c + 1;
            }
        }
    }
    Some(total.try_into().unwrap())
}

fn serialize_map_p2(input: &str) -> (Vec<Vec<Cell>>, Bot) {
    let width = input.find('\n').unwrap();

    let mut map = vec![vec![Cell::Empty; (width - 2) * 2]; width - 2];
    let mut bot = Bot::new(0, 0, width - 2, (width - 2) * 2);

    let input: Vec<char> = input.chars().collect();
    for i in width + 1..input.len() - width {
        let col = i % (width + 1);
        if col == 0 || col == width - 1 {
            continue;
        }
        let row = i / (width + 1);
        match input[i] {
            'O' => {
                map[row - 1][col * 2 - 2] = Cell::BoxLeft;
                map[row - 1][col * 2 - 1] = Cell::BoxRight;
            }
            '#' => {
                map[row - 1][col * 2 - 2] = Cell::Wall;
                map[row - 1][col * 2 - 1] = Cell::Wall;
            }
            '@' => {
                bot.row = row - 1;
                bot.col = col * 2 - 2;
            }
            _ => (),
        }
    }

    (map, bot)
}

// This function calls itself exponentially for stacked boxes
fn can_move(row: usize, col: usize, dir: Dir, bot: &Bot, map: &Vec<Vec<Cell>>) -> bool {
    // println!("Checking ({}, {})", row, col);
    if row == bot.row_lim || col == bot.col_lim {
        return false;
    }
    if map[row][col] == Cell::Empty {
        return true;
    }
    if map[row][col] == Cell::Wall {
        return false;
    }
    if dir == Dir::Up && row == 0 {
        return false;
    }
    if dir == Dir::Down && row == bot.row_lim - 1 {
        return false;
    }
    let nrow = match dir {
        Dir::Down => row + 1,
        Dir::Up => row - 1,
        _ => panic!("Invalid use of do_move"),
    };
    if map[row][col] == Cell::BoxLeft {
        can_move(nrow, col, dir, bot, map) && can_move(nrow, col + 1, dir, bot, map)
    } else {
        can_move(nrow, col, dir, bot, map) && can_move(nrow, col - 1, dir, bot, map)
    }
}

fn do_move(row: usize, col: usize, dir: Dir, bot: &Bot, map: &mut Vec<Vec<Cell>>) {
    // println!("We moving ({row}, {col}): {:?}", map[row][col]);
    let nrow = match dir {
        Dir::Down => row + 1,
        Dir::Up => row - 1,
        _ => panic!("Invalid use of do_move"),
    };
    if map[row][col] == Cell::BoxLeft {
        if map[nrow][col] != Cell::Empty {
            // println!("Occupied! {:?} recursing!", map[nrow][col]);
            do_move(nrow, col, dir, bot, map);
        }
        if map[nrow][col + 1] != Cell::Empty {
            // println!("Occupied! {:?} recursing!", map[nrow][col + 1]);
            do_move(nrow, col + 1, dir, bot, map);
        }
        map[nrow][col] = Cell::BoxLeft;
        map[nrow][col + 1] = Cell::BoxRight;
        map[row][col] = Cell::Empty;
        map[row][col + 1] = Cell::Empty;
    } else {
        if map[nrow][col] != Cell::Empty {
            // println!("Occupied! {:?} recursing!", map[nrow][col]);
            do_move(nrow, col, dir, bot, map);
        }
        if map[nrow][col - 1] != Cell::Empty {
            // println!("Occupied! {:?} recursing!", map[nrow][col - 1]);
            do_move(nrow, col - 1, dir, bot, map);
        }
        map[nrow][col] = Cell::BoxRight;
        map[nrow][col - 1] = Cell::BoxLeft;
        map[row][col] = Cell::Empty;
        map[row][col - 1] = Cell::Empty;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let (mut map, mut bot) = serialize_map_p2(map);
    // println!("{:?}", bot);
    'dir: for dir in moves.chars() {
        if dir == '\n' {
            continue;
        }
        let dir = Dir::from(dir);

        // print_map(&map, &bot, dir);

        if !bot.can_move(dir) {
            continue;
        }

        let (row, mut col) = bot.move_pos(dir);
        match map[row][col] {
            Cell::Empty => {
                bot.move_bot(dir);
                continue;
            }
            Cell::Wall => continue,
            _ => (),
        }

        // println!("Trying ({row}, {col}): {:?}", map[row][col]);

        if dir == Dir::Left {
            while map[row][col].is_box() {
                col -= 1;
                if col == 0 && map[row][col].is_box() {
                    continue 'dir;
                }
            }
            if map[row][col] == Cell::Empty {
                // println!("Moving bot");
                bot.move_bot(dir);
                map[bot.row][bot.col] = Cell::Empty;
                while col < bot.col {
                    // println!("Shaboing! {}, {}", col, bot.col);
                    map[row][col] = Cell::BoxLeft;
                    map[row][col + 1] = Cell::BoxRight;
                    col += 2;
                }
            }
        } else if dir == Dir::Right {
            while map[row][col].is_box() {
                col += 1;
                if col >= bot.col_lim {
                    // println!("Exceeded col lim");
                    continue 'dir;
                }
            }
            if map[row][col] == Cell::Empty {
                // println!("Moving bot");
                bot.move_bot(dir);
                map[bot.row][bot.col] = Cell::Empty;
                while col > bot.col {
                    map[row][col] = Cell::BoxRight;
                    map[row][col - 1] = Cell::BoxLeft;
                    col -= 2;
                }
            }
        } else if can_move(row, col, dir, &bot, &map) {
            // println!("Ni!");
            do_move(row, col, dir, &bot, &mut map);
            // println!("Moving bot");
            bot.move_bot(dir);
        }
    }

    // print_map(&map, &bot, Dir::Up);

    let mut total = 0;
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == Cell::BoxLeft {
                total += (r + 1) * 100 + c + 2;
            }
        }
    }
    Some(total.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
