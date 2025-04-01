advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let (towels, designs) = input.trim().split_once("\n\n").unwrap();
    let towels = towels.split(", ").map(str::to_owned).collect();
    let designs = designs.split("\n").map(str::to_owned).collect();
    (towels, designs)
}

struct Node {
    next: [Option<Box<Node>>; 26],
    end: bool,
}

impl Node {
    pub fn new(end: bool) -> Self {
        Self {
            next: [const { None }; 26],
            end,
        }
    }

    pub fn add(&mut self, input: &str) {
        if input.len() == 0 {
            self.end = true;
            return;
        }
        let c = input.chars().next().unwrap();
        let c = c as usize - 'a' as usize;
        let n = self.next[c].get_or_insert(Box::new(Node::new(false)));
        n.add(&input[1..]);
    }

    pub fn check(&self, root: &Node, input: &str) -> bool {
        if input.len() == 0 {
            return self.end;
        }
        let c = input.chars().next().unwrap();
        let c = c as usize - 'a' as usize;
        if self.next[c]
            .as_ref()
            .is_some_and(|n| n.check(root, &input[1..]))
        {
            return true;
        }
        self.end && root.check(root, input)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (t, d) = parse_input(input);
    let mut root = Node::new(false);

    for t in &t {
        root.add(t);
    }

    let mut c = 0;
    for d in &d {
        if root.check(&root, d) {
            c += 1;
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
