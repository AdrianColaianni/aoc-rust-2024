use std::collections::HashMap;

advent_of_code::solution!(20);

type Map = Vec<Vec<char>>;

pub fn part_one(input: &str) -> Option<usize> {
    let mut start = (0, 0);
    let mut map: Map = vec![];
    for (r, line) in input.lines().enumerate() {
        map.push(vec![]);
        for (c, char) in line.chars().enumerate() {
            map[r].push(char);
            if char == 'S' {
                start = (r, c);
            }
        }
    }
    let size = map.len();

    // println!("{:?}", start);
    // println!("{:?}", end);
    // print!("  ");
    // for c in 0..size {
    //     print!("{}", c%10);
    // }
    // println!();
    // for r in 0..size {
    //     print!("{:2}", r);
    //     for c in 0..size {
    //         print!("{}", map[r][c]);
    //     }
    //     println!();
    // }

    let mut cost: Vec<Vec<usize>> = (0..size)
        .map(|_| (0..size).map(|_| usize::MAX).collect())
        .collect();

    let mut path = vec![];

    let (mut r, mut c) = start;
    let mut cs = 0;
    loop {
        path.push((r, c));
        cost[r][c] = cs;
        if map[r][c] == 'E' {
            break;
        }
        cs += 1;
        r -= 1;
        if map[r][c] != '#' && cost[r][c] == usize::MAX {
            continue;
        }
        r += 2;
        if map[r][c] != '#' && cost[r][c] == usize::MAX {
            continue;
        }
        r -= 1;
        c -= 1;
        if map[r][c] != '#' && cost[r][c] == usize::MAX {
            continue;
        }
        c += 2;
        if map[r][c] != '#' && cost[r][c] == usize::MAX {
            continue;
        }
    }

    let mut cheats: HashMap<usize, usize> = HashMap::new();

    for (r, c) in path {
        if r > 2 && map[r - 1][c] == '#' && map[r - 2][c] != '#' {
            if cost[r - 2][c] > cost[r][c] {
                let s = cost[r - 2][c] - cost[r][c] - 2;
                *cheats.entry(s).or_default() += 1;
            }
        }
        if r < size - 3 && map[r + 1][c] == '#' && map[r + 2][c] != '#' {
            if cost[r + 2][c] > cost[r][c] {
                let s = cost[r + 2][c] - cost[r][c] - 2;
                *cheats.entry(s).or_default() += 1;
            }
        }
        if c > 2 && map[r][c - 1] == '#' && map[r][c - 2] != '#' {
            if cost[r][c - 2] > cost[r][c] {
                let s = cost[r][c - 2] - cost[r][c] - 2;
                *cheats.entry(s).or_default() += 1;
            }
        }
        if c < size - 3 && map[r][c + 1] == '#' && map[r][c + 2] != '#' {
            if cost[r][c + 2] > cost[r][c] {
                let s = cost[r][c + 2] - cost[r][c] - 2;
                *cheats.entry(s).or_default() += 1;
            }
        }
    }

    let mut c = 0;
    for (cost, count) in cheats {
        if cost >= 100 {
            c += count;
        }
    }

    Some(c)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
