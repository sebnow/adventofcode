use anyhow::{Context, Result};
use std::collections::VecDeque;

#[derive(Debug)]
enum Inode {
    File(File),
    Dir(Dir),
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

#[derive(Debug)]
struct Dir {
    name: String,
    inums: Vec<usize>,
}

impl Inode {
    fn total_size(&self, ilist: &[Inode]) -> u64 {
        match self {
            Inode::File(f) => f.size,
            Inode::Dir(d) => d
                .inums
                .iter()
                .map(|&inum| ilist[inum].total_size(ilist))
                .sum(),
        }
    }

    fn name(&self) -> &str {
        match self {
            Inode::File(f) => &f.name,
            Inode::Dir(d) => &d.name,
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Inode>> {
    let mut ilist = Vec::new();
    let mut path = VecDeque::new();
    let mut in_ls = false;

    ilist.push(Inode::Dir(Dir {
        name: "".to_string(),
        inums: vec![],
    }));

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
                let dir_inum = path.front().unwrap();

                match &ilist[*dir_inum] {
                    Inode::File(_) => panic!("expected directory but found file"),
                    Inode::Dir(d) => {
                        let inum = d
                            .inums
                            .iter()
                            .find(|&inum| ilist[*inum].name() == dirname)
                            .with_context(|| "finding child directory")
                            .unwrap();

                        path.push_front(*inum);
                    }
                };
            }
            (_, "$", "ls") => {
                in_ls = true;
            }
            (true, size, name) if size.as_bytes()[0] < b'a' => {
                let parent_inum = path.front().unwrap();
                ilist.push(Inode::File(File {
                    name: name.to_string(),
                    size: size.parse().unwrap(),
                }));
                let file_inum = ilist.len() - 1;
                let parent = &mut ilist[*parent_inum];

                match parent {
                    Inode::Dir(dir) => dir.inums.push(file_inum),
                    Inode::File(_) => panic!("expected directory but found file"),
                }
            }
            (true, "dir", name) => {
                let parent_inum = path.front().unwrap();
                ilist.push(Inode::Dir(Dir {
                    name: name.to_string(),
                    inums: vec![],
                }));
                let dir_inum = ilist.len() - 1;
                let parent = &mut ilist[*parent_inum];

                match parent {
                    Inode::Dir(dir) => dir.inums.push(dir_inum),
                    Inode::File(_) => panic!("expected directory but found file"),
                }
            }
            _ => panic!("unexpected case: {}", line),
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
            Inode::File(_) => None,
            _ => {
                let total_size = inode.total_size(&ilist);
                if total_size <= 100_000 {
                    Some(total_size)
                } else {
                    None
                }
            }
        })
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
            Inode::File(_) => false,
            _ => size >= space_to_delete,
        })
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(_, size)| size)
        .unwrap()
        .to_string()
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day07.txt");
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
