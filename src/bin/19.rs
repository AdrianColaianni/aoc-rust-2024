use std::collections::HashMap;

advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let (towels, designs) = input.trim().split_once("\n\n").unwrap();
    let towels = towels.split(", ").map(str::to_owned).collect();
    let designs = designs.split("\n").map(str::to_owned).collect();
    (towels, designs)
}

struct Node {
    next: [Option<Box<Node>>; 5],
    end: bool,
}

impl Node {
    pub fn new(end: bool) -> Self {
        Self {
            next: [const { None }; 5],
            end,
        }
    }

    pub fn add(&mut self, input: &str) {
        if input.len() == 0 {
            self.end = true;
            return;
        }
        let mut c = input.chars().next().unwrap() as usize;
        c = c % 10 - 3;
        if c > 2 {
            c -= 2
        }
        let n = self.next[c].get_or_insert(Box::new(Node::new(false)));
        n.add(&input[1..]);
    }

    pub fn check(&self, root: &Node, input: &str) -> bool {
        if input.len() == 0 {
            return self.end;
        }
        let mut c = input.chars().next().unwrap() as usize;
        c = c % 10 - 3;
        if c > 2 {
            c -= 2
        }
        if self.next[c]
            .as_ref()
            .is_some_and(|n| n.check(root, &input[1..]))
        {
            return true;
        }
        self.end && root.check(root, input)
    }

    pub fn check_count<'i>(&self, root: &Node, cache: &mut HashMap<&'i str, usize>, input: &'i str) -> usize {
        if input.len() == 0 {
            return self.end as usize;
        }
        let s = std::ptr::eq(self, root);
        if s {
            if let Some(&count) = cache.get(input) {
                return count;
            }
        }

        let mut c = input.chars().next().unwrap() as usize;
        c = c % 10 - 3;
        if c > 2 {
            c -= 2
        }
        let mut t = 0;
        if let Some(n) = &self.next[c] {
            t += n.check_count(root, cache, &input[1..]);
        }
        if self.end {
            t += root.check_count(root, cache, input);
        }
        if s {
            cache.insert(input, t);
        }

        t
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
    let (t, d) = parse_input(input);
    let mut root = Node::new(false);

    for t in &t {
        root.add(t);
    }

    let mut cache = HashMap::new();

    let mut c = 0;
    for d in &d {
        c += root.check_count(&root, &mut cache, d);
    }

    Some(c)
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
        assert_eq!(result, Some(16));
    }
}
