advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let l: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let row_lim = l.len();
    let col_lim = l[0].len();
    let mut c = 0;

    // Find - matches
    for row in 0..row_lim {
        for col in 0..col_lim {
            let look_right = col < col_lim - 3;
            let look_down = row < row_lim - 3;
            // Check -
            if look_right
                && ((l[row][col] == 'X'
                    && l[row][col + 1] == 'M'
                    && l[row][col + 2] == 'A'
                    && l[row][col + 3] == 'S')
                    || (l[row][col] == 'S'
                        && l[row][col + 1] == 'A'
                        && l[row][col + 2] == 'M'
                        && l[row][col + 3] == 'X'))
            {
                c += 1;
            }
            // Check |
            if look_down
                && ((l[row][col] == 'X'
                    && l[row + 1][col] == 'M'
                    && l[row + 2][col] == 'A'
                    && l[row + 3][col] == 'S')
                    || (l[row][col] == 'S'
                        && l[row + 1][col] == 'A'
                        && l[row + 2][col] == 'M'
                        && l[row + 3][col] == 'X'))
            {
                c += 1;
            }
            // Check \
            if look_down
                && look_right
                && ((l[row][col] == 'X'
                    && l[row + 1][col + 1] == 'M'
                    && l[row + 2][col + 2] == 'A'
                    && l[row + 3][col + 3] == 'S')
                    || (l[row][col] == 'S'
                        && l[row + 1][col + 1] == 'A'
                        && l[row + 2][col + 2] == 'M'
                        && l[row + 3][col + 3] == 'X'))
            {
                c += 1;
            }
            // Check /
            if look_down
                && col >= 3 // Can look left
                && ((l[row][col] == 'X'
                    && l[row + 1][col - 1] == 'M'
                    && l[row + 2][col - 2] == 'A'
                    && l[row + 3][col - 3] == 'S')
                    || (l[row][col] == 'S'
                        && l[row + 1][col - 1] == 'A'
                        && l[row + 2][col - 2] == 'M'
                        && l[row + 3][col - 3] == 'X'))
            {
                c += 1;
            }
        }
    }

    Some(c)
}

pub fn part_two(input: &str) -> Option<u32> {
    let l: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut c = 0;

    for row in 1..l.len() - 1 {
        for col in 1..l[0].len() - 1 {
            if l[row][col] != 'A' {
                continue;
            }
            // Check \
            let ltor = (l[row - 1][col - 1] == 'M' && l[row + 1][col + 1] == 'S')
                || (l[row - 1][col - 1] == 'S' && l[row + 1][col + 1] == 'M');
            // Check /
            let rtol = (l[row + 1][col - 1] == 'M' && l[row - 1][col + 1] == 'S')
                || (l[row + 1][col - 1] == 'S' && l[row - 1][col + 1] == 'M');

            if ltor && rtol {
                c += 1;
            }
        }
    }

    Some(c)
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
