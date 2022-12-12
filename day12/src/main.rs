use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

fn part2(input: &str) -> usize {
    let map: Map = input.parse().unwrap();
    map.starting_points()
        .iter()
        .map(|p| (bfs(&map.map, p, &map.end)))
        .min()
        .unwrap()
}

fn part1(input: &str) -> usize {
    let map: Map = input.parse().unwrap();
    bfs(&map.map, &map.start, &map.end)
}

fn bfs(map: &[Vec<u8>], start: &Point, end: &Point) -> usize {
    let mut queue = VecDeque::from([(*start, 0)]);
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    while let Some((node, distance)) = queue.pop_back() {
        let height = map[node.0][node.1];
        if node == *end {
            return distance;
        }

        for neighbor in [
            (node.0 + 1, node.1),
            (node.0 - 1, node.1),
            (node.0, node.1 + 1),
            (node.0, node.1 - 1),
        ] {
            if let Some(&neighbor_height) = map.get(neighbor.0).and_then(|row| row.get(neighbor.1))
            {
                if neighbor_height <= height + 1 && !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_front((neighbor, distance + 1))
                }
            }
        }
    }
    usize::MAX
}

type Point = (usize, usize);
struct Map {
    start: Point,
    end: Point,
    map: Vec<Vec<u8>>,
}

impl Map {
    fn starting_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        for (i, row) in self.map.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                if value == b'a' {
                    points.push((i, j));
                }
            }
        }
        points
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Point = (0, 0);
        let mut end: Point = (0, 0);
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
    use indoc::indoc;

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

    #[test]
    fn test_part1() {
        let example = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi"};
        let result = part1(example);
        assert_eq!(31, result);
    }
}
