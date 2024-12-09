advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let map: Vec<char> = input.trim().chars().collect();
    // println!("{:?}", map);
    let mut blocks: Vec<Option<usize>> = vec![];

    for i in 0..map.len() {
        if i % 2 == 0 {
            let file: usize = map[i].to_digit(10).unwrap() as usize;
            let id = i / 2;
            for _ in 0..file {
                blocks.push(Some(id));
                // print!("{}", id);
            }
        } else {
            let space: usize = map[i].to_digit(10).unwrap() as usize;
            for _ in 0..space {
                blocks.push(None);
            }
            // print!("{}", ".".repeat(space));
        }
    }
    // println!();

    // println!("{:?}", blocks);

    let mut i = 0;
    loop {
        if i >= blocks.len() {
            break;
        }
        if blocks[i].is_some() {
            i += 1;
            continue;
        }
        while blocks[i].is_none() {
            blocks[i] = blocks.pop().unwrap();
        }
        // println!("Index {} empty, now has {:?}", i, blocks[i]);
        i += 1;
    }

    // println!("{:?}", blocks);

    let mut ans = 0;

    for (i, v) in blocks.into_iter().enumerate() {
        ans += i * v.unwrap();
    }

    Some(ans)
}

#[derive(Debug, Clone, Copy)]
enum Node {
    // id, size
    File(usize, usize),
    // size
    Space(usize),
}

impl Node {
    fn is_free(&self) -> bool {
        match self {
            Node::File(_, _) => false,
            Node::Space(_) => true,
        }
    }

    fn size(&self) -> usize {
        match *self {
            Node::File(_, s) => s,
            Node::Space(s) => s,
        }
    }

    fn add(&mut self, v: usize) {
        match self {
            Node::File(_, s) => *s += v,
            Node::Space(s) => *s += v,
        }
    }

    fn sub(&mut self, v: usize) {
        match self {
            Node::File(_, s) => *s -= v,
            Node::Space(s) => *s -= v,
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Vec<char> = input.trim().chars().collect();
    let mut blocks: Vec<Node> = vec![];

    for i in 0..map.len() {
        if i % 2 == 0 {
            let size: usize = map[i].to_digit(10).unwrap() as usize;
            let id = i / 2;
            blocks.push(Node::File(id, size));
        } else {
            let size: usize = map[i].to_digit(10).unwrap() as usize;
            blocks.push(Node::Space(size));
        }
    }

    let mut i = blocks.len() - 1;
    loop {
        if i == 0 {
            break;
        }
        match blocks[i] {
            Node::File(id, size) => {
                if let Some((free_idx, free)) = blocks
                    .iter()
                    .enumerate()
                    .filter(|(bi, b)| *bi < i && b.is_free() && b.size() >= size)
                    .next()
                {
                    // println!("Can fit block {} in at {}", id, free_idx);
                    if size == free.size() {
                        blocks[free_idx] = blocks[i];
                        blocks[i] = Node::Space(size);
                    } else {
                        blocks[free_idx].sub(size);
                        blocks.insert(free_idx, blocks[i]);
                        blocks[i + 1] = Node::Space(size);
                    }
                    // Combine nearby spaces
                    let mut i = 0;
                    loop {
                        if i == blocks.len() - 1 {
                            break;
                        }
                        if blocks[i].is_free() && blocks[i + 1].is_free() {
                            let size = blocks.remove(i + 1).size();
                            blocks[i].add(size);
                        } else {
                            i += 1;
                        }
                    }
                    // println!("{:?}", blocks);
                }
            }
            Node::Space(_) => (),
        }
        i -= 1;
    }

    // println!("{:?}", blocks);

    let mut ans = 0;
    let mut i = 0;
    for block in blocks {
        match block {
            Node::File(id, size) => {
                for _ in 0..size {
                    ans += id * i;
                    i += 1;
                }
            }
            Node::Space(size) => i += size,
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
