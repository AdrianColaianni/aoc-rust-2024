advent_of_code::solution!(4);

fn get_diagonals(l: Vec<Vec<char>>, ltor: bool) -> Vec<Vec<char>> {
    let mut diagonals = vec![vec![]; 1 + l.len() + l[0].len() - 2];
    for (row, mut l) in l.into_iter().enumerate() {
        if !ltor {
            l = l.into_iter().rev().collect()
        }
        for (col, c) in l.iter().enumerate() {
            match row.cmp(&col) {
                std::cmp::Ordering::Less => diagonals[col - row].push(c.clone()),
                std::cmp::Ordering::Equal => diagonals[0].push(c.clone()),
                std::cmp::Ordering::Greater => diagonals[l.len() - 1 + row - col].push(c.clone()),
            }
        }
    }

    diagonals
}

pub fn part_one(input: &str) -> Option<u32> {
    // Find - matches
    let mut c: u32 = input.matches("XMAS").count() as u32;
    c += input.matches("SAMX").count() as u32;

    // Find | matches
    let l: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut f = vec![vec![]; l.len()];
    for line in &l {
        for (i, c) in line.iter().enumerate() {
            f[i].push(c.clone());
        }
    }
    c += f
        .iter()
        .map(|l| {
            let l = l.iter().collect::<String>();
            l.matches("XMAS").count() + l.matches("SAMX").count()
        })
        .sum::<usize>() as u32;

    // Find \ matches
    let diagonals = get_diagonals(l.clone(), true);
    c += diagonals
        .iter()
        .map(|c| {
            let c: String = c.iter().collect();
            c.matches("XMAS").count() + c.matches("SAMX").count()
        })
        .sum::<usize>() as u32;

    // Find / matches
    let diagonals = get_diagonals(l, false);
    c += diagonals
        .iter()
        .map(|c| {
            let c: String = c.iter().collect();
            c.matches("XMAS").count() + c.matches("SAMX").count()
        })
        .sum::<usize>() as u32;

    Some(c)
}

pub fn part_two(input: &str) -> Option<u32> {
    let l: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut crosses: Vec<Vec<usize>> = vec![vec![0; l[0].len()]; l.len()];
    let width = crosses[0].len();

    let diagonals: Vec<String> = get_diagonals(l.clone(), true)
        .iter()
        .map(|e| e.iter().collect())
        .collect();

    for (di, l) in diagonals.iter().enumerate() {
        for (mut i, _) in l.match_indices("MAS") {
            i += 1; // Get center
            let (row, col) = if di == 0 {
                (i, i)
            } else if di < width {
                (i, i + di)
            } else {
                (i + di - width + 1, i)
            };
            crosses[row][col] += 1;
        }
        for (mut i, _) in l.match_indices("SAM") {
            i += 1;
            let (row, col) = if di == 0 {
                (i, i)
            } else if di < width {
                (i, i + di)
            } else {
                (i + di - width + 1, i)
            };
            crosses[row][col] += 1;
        }
    }

    let diagonals: Vec<String> = get_diagonals(l.clone(), false)
        .iter()
        .map(|e| e.iter().collect())
        .collect();

    for (di, l) in diagonals.iter().enumerate() {
        for (mut i, _) in l.match_indices("MAS") {
            i += 1; // Get center
            let (row, col) = if di == 0 {
                (i, width - i - 1)
            } else if di < width {
                (i, width - i - di - 1)
            } else {
                (di - width + 1 + i, width - i - 1)
            };
            crosses[row][col] += 1;
        }
        for (mut i, _) in l.match_indices("SAM") {
            i += 1;
            let (row, col) = if di == 0 {
                (i, width - i - 1)
            } else if di < width {
                (i, width - i - di - 1)
            } else {
                (di - width + 1 + i, width - i - 1)
            };
            crosses[row][col] += 1;
        }
    }

    Some(
        crosses
            .into_iter()
            .map(|l| l.iter().filter(|v| **v == 2).count())
            .sum::<usize>() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
