use indextree::{Arena, NodeId};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let arena = &mut Arena::new();
    let (root, directories) = parse(INPUT, arena);
    let directory_sizes: Vec<usize> = directories.iter().map(|d| size(d, arena)).collect();
    let part1_size: usize = directory_sizes
        .iter()
        .filter(|&size| *size <= 100_000)
        .sum();
    println!("{part1_size}");

    let space_available = 70_000_000;
    let space_needed = 30_000_000;
    let used_space = size(&root, arena);
    let free_space = space_available - used_space;
    let free_up = space_needed - free_space;

    let part2_size = directory_sizes
        .iter()
        .filter(|&&size| size >= free_up)
        .min()
        .unwrap();
    println!("{part2_size}");
}

fn parse(input: &str, arena: &mut Arena<Entry>) -> (NodeId, Vec<NodeId>) {
    let root = arena.new_node(Entry::Directory {
        name: "/".to_string(),
    });
    let mut directories: Vec<NodeId> = vec![];

    let mut current_node = root;
    for line in input.lines() {
        let args: Vec<&str> = line.split_ascii_whitespace().collect();
        match args[..] {
            ["$", "cd", "/"] => current_node = root,
            ["$", "cd", ".."] => current_node = arena[current_node].parent().unwrap(),
            ["$", "cd", directory] => {
                current_node = current_node
                    .children(arena)
                    .find(|&child| matches!(arena[child].get(), Entry::Directory { name } if name == directory))
                    .unwrap();
            }
            ["$", "ls"] => (),
            ["dir", name] => {
                let directory = arena.new_node(Entry::Directory {
                    name: name.to_string(),
                });
                directories.push(directory);
                current_node.append(directory, arena);
            }
            [size, name] => {
                let size: usize = size.parse().unwrap();
                let file = arena.new_node(Entry::File {
                    name: name.to_string(),
                    size,
                });
                current_node.append(file, arena);
            }
            _ => panic!("Can't parse {}", line),
        }
    }
    (root, directories)
}

#[derive(Debug)]
enum Entry {
    #[allow(dead_code)]
    File {
        name: String,
        size: usize,
    },
    Directory {
        name: String,
    },
}

fn size(entry: &NodeId, arena: &Arena<Entry>) -> usize {
    match arena[*entry].get() {
        Entry::File { name: _, size } => *size,
        Entry::Directory { name: _ } => entry
            .children(arena)
            .map(|child| match arena[child].get() {
                Entry::File { name: _, size } => *size,
                Entry::Directory { name: _ } => size(&child, arena),
            })
            .sum(),
    }
}
