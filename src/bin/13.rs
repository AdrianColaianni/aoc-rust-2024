use std::isize;

advent_of_code::solution!(13);

fn new_button(input: &str) -> (isize, isize) {
    let x = input[12..14].parse().unwrap();
    let y = input[18..20].parse().unwrap();

    (x, y)
}

fn new_prize(input: &str) -> (isize, isize) {
    let x = input.find(',').unwrap();
    let x = input[9..x].parse().unwrap();

    let y = input.find('Y').unwrap() + 2;
    let y = input[y..].parse().unwrap();

    (x, y)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for machine in input.split("\n\n") {
        let mut lines = machine.split('\n');
        let (a_x, a_y) = new_button(lines.next().unwrap());
        let (b_x, b_y) = new_button(lines.next().unwrap());
        let (p_x, p_y) = new_prize(lines.next().unwrap());

        let p_a = (b_y * p_x - b_x * p_y) / (b_y * a_x - b_x * a_y);
        let p_b = (p_x - a_x * p_a) / b_x;
        if p_x == a_x * p_a + b_x * p_b && p_y == a_y * p_a + b_y * p_b {
            total += p_a * 3 + p_b;
        }
    }

    Some(total.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut total = 0;
    for machine in input.split("\n\n") {
        let mut lines = machine.split('\n');
        let (a_x, a_y) = new_button(lines.next().unwrap());
        let (b_x, b_y) = new_button(lines.next().unwrap());
        let (mut p_x, mut p_y) = new_prize(lines.next().unwrap());
        p_x += 10000000000000;
        p_y += 10000000000000;

        let p_a = (b_y * p_x - b_x * p_y) / (b_y * a_x - b_x * a_y);
        let p_b = (p_x - a_x * p_a) / b_x;
        if p_x == a_x * p_a + b_x * p_b && p_y == a_y * p_a + b_y * p_b {
            total += p_a * 3 + p_b;
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
