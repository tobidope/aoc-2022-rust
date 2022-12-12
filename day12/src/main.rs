use indoc::indoc;
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", part1(INPUT));
}

fn part1(input: &str) -> usize {
    let map: Map = input.parse().unwrap();
    let mut queue = VecDeque::from([map.start]);
    let mut visited = HashMap::new();
    visited.insert((0, 0), (0, 0));

    while !queue.is_empty() {
        let node = queue.pop_back().unwrap();
        let height = map.map[node.0 as usize][node.1 as usize];
        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
                if (i, j) == (0, 0) {
                    continue;
                }
                let neighbor = (node.0 + i, node.1 + j);
                if let Some(Some(&value)) = map
                    .map
                    .get(neighbor.0 as usize)
                    .map(|row| row.get(neighbor.1 as usize))
                {
                    if value <= height + 1 {
                        
                    }
                }
            }
        }
    }

    todo!()
}

type Point = (i32, i32);
struct Map {
    start: Point,
    end: Point,
    map: Vec<Vec<u8>>,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut map = vec![];
        for (row, line) in s.lines().enumerate() {
            if let Some(column) = line.find('S') {
                start = (row, column);
            }
            if let Some(column) = line.find('E') {
                end = (row, column);
            }
            map.push(
                line.bytes()
                    .map(|c| match c {
                        b'S' => b'a',
                        b'E' => b'z',
                        _ => c,
                    })
                    .collect(),
            )
        }
        Ok(Map { start, end, map })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map() {
        let example = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi"};
        let map: Map = example.parse().unwrap();
        assert_eq!((0, 0), map.start);
        assert_eq!((2, 5), map.end);
        assert_eq!(b'm', map.map[0][7])
    }
}
