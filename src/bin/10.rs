advent_of_code::solution!(10);

struct Map<'a> {
    data: &'a [u8],
    row: usize,
    col: usize
}

impl<'a> Map<'a> {
    fn new(data: &'a str) -> Self {
        let data = data.trim();
        let row = data.find('\n').unwrap() + 1;
        let col = data.matches('\n').count() + 1;
        let data = data.as_bytes();
        Self { data, row, col }
    }

    fn get(&self, row: usize, col: usize) -> char {
        self.data[row * self.row + col] as char
    }

    fn rows(&self) -> usize {
        self.row - 1
    }

    fn cols(&self) -> usize {
        self.col
    }
}

fn trail_score(row: usize, col: usize, map: &Map, nines: &mut Vec<(usize, usize)>) {
    if map.get(row, col) == '9' && !nines.contains(&(row, col)) {
        nines.push((row, col));
        return;
    }

    let next = (map.get(row, col) as u8 + 1) as char;
    if row > 0 && map.get(row - 1, col) == next {
        trail_score(row - 1, col, map, nines);
    }
    if row + 1 < map.rows() && map.get(row + 1, col) == next {
        trail_score(row + 1, col, map, nines);
    }
    if col > 0 && map.get(row, col - 1) == next {
        trail_score(row, col - 1, map, nines);
    }
    if col + 1 < map.cols() && map.get(row, col + 1) == next {
        trail_score(row, col + 1, map, nines);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::new(input);

    let mut ans = 0;

    for row in 0..map.rows() {
        for col in 0..map.cols() {
            if map.get(row, col) != '0' {
                continue;
            }
            let mut nines = vec![];
            trail_score(row, col, &map, &mut nines);
            ans += nines.len() as u32;
        }
    }

    Some(ans)
}

fn trail_score_p2(row: usize, col: usize, map: &Map) -> u32 {
    if map.get(row, col) == '9' {
        return 1;
    }

    let mut v = 0;
    let next = (map.get(row, col) as u8 + 1) as char;
    if row > 0 && map.get(row - 1, col) == next {
        v += trail_score_p2(row - 1, col, map);
    }
    if row + 1 < map.rows() && map.get(row + 1, col) == next {
        v += trail_score_p2(row + 1, col, map);
    }
    if col > 0 && map.get(row, col - 1) == next {
        v += trail_score_p2(row, col - 1, map);
    }
    if col + 1 < map.cols() && map.get(row, col + 1) == next {
        v += trail_score_p2(row, col + 1, map);
    }

    v
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::new(input);

    let mut ans = 0;

    for row in 0..map.rows() {
        for col in 0..map.cols() {
            if map.get(row, col) == '0' {
                ans += trail_score_p2(row, col, &map);
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
