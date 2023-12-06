use anyhow::{anyhow, Context, Result};
use std::collections::VecDeque;

#[derive(Debug)]
enum Inode {
    File { name: String, size: u64 },
    Dir { name: String, inums: Vec<usize> },
}

impl Inode {
    fn total_size(&self, ilist: &[Inode]) -> u64 {
        match self {
            Inode::File { size, .. } => *size,
            Inode::Dir { inums, .. } => inums
                .iter()
                .map(|&inum| ilist[inum].total_size(ilist))
                .sum(),
        }
    }

    fn name(&self) -> &str {
        match self {
            Inode::File { name, .. } => name,
            Inode::Dir { name, .. } => name,
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Inode>> {
    let mut ilist = Vec::new();
    let mut path = VecDeque::new();
    let mut in_ls = false;

    ilist.push(Inode::Dir {
        name: "".to_string(),
        inums: vec![],
    });

    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        match (in_ls, words[0], words[1]) {
            (_, "$", "cd") if words[2] == ".." => {
                in_ls = false;
                path.pop_front();
            }
            (_, "$", "cd") if words[2] == "/" => {
                in_ls = false;
                path.clear();
                path.push_front(0);
            }
            (_, "$", "cd") => {
                in_ls = false;
                let dirname = words[2..].join(" ");
                let dir_inum = path
                    .front()
                    .with_context(|| "current working directory is not set")?;

                match &ilist[*dir_inum] {
                    Inode::File { .. } => return Err(anyhow!("expected directory but found file")),
                    Inode::Dir { inums, .. } => {
                        let inum = inums
                            .iter()
                            .find(|&inum| ilist[*inum].name() == dirname)
                            .with_context(|| "finding child directory")?;

                        path.push_front(*inum);
                    }
                };
            }
            (_, "$", "ls") => {
                in_ls = true;
            }
            (true, size, name) => {
                let parent_inum = path
                    .front()
                    .with_context(|| "listing while not in a directory")?;
                let inode = if size.as_bytes()[0] < b'a' {
                    Inode::File {
                        name: name.to_string(),
                        size: size.parse().with_context(|| "malformed file size")?,
                    }
                } else {
                    Inode::Dir {
                        name: name.to_string(),
                        inums: vec![],
                    }
                };

                ilist.push(inode);
                let inum = ilist.len() - 1;
                let parent = &mut ilist[*parent_inum];

                match parent {
                    Inode::Dir { inums, .. } => inums.push(inum),
                    Inode::File { .. } => return Err(anyhow!("expected directory but found file")),
                }
            }
            _ => return Err(anyhow!("unexpected case: {}", line)),
        }
    }

    Ok(ilist)
}

fn part_one(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let ilist = input;

    ilist
        .iter()
        .filter_map(|inode| match inode {
            Inode::File { .. } => None,
            _ => Some(inode.total_size(&ilist)),
        })
        .filter(|&size| size <= 100_000)
        .sum::<u64>()
        .to_string()
}

fn part_two(s: &str) -> String {
    let input = parse_input(s).unwrap();
    let ilist = input;
    let disk_capacity = 70_000_000;
    let required_unused_space = 30_000_000;

    let total_used_space = ilist[0].total_size(&ilist);
    let unused_space = disk_capacity - total_used_space;
    let space_to_delete = required_unused_space - unused_space;

    ilist
        .iter()
        .map(|inode| (inode, inode.total_size(&ilist)))
        .filter(|&(inode, size)| match inode {
            Inode::File { .. } => false,
            _ => size >= space_to_delete,
        })
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(_, size)| size)
        .unwrap()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../../../input/2022/day07.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use aocutil::test_example;

    test_example!(example_7_1, part_one, 7, 1, 1);
    test_example!(example_7_2, part_two, 7, 2, 1);
}
