use itertools::Itertools;
use std::collections::BTreeSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let (_, pathes) = parse(input).unwrap();
    let mut grid = create_grid(&pathes);
    let count_elements = grid.len();
    let max_y = grid.iter().max_by_key(|p| p.y).unwrap().y;
    let mut path = vec![Point { x: 500, y: 0 }];
    loop {
        let current_position = *path.last().unwrap();
        if current_position.y >= max_y {
            break;
        }
        let mut next_found = false;
        for next in [(0, 1), (-1, 1), (1, 1)] {
            let next_position = Point {
                x: current_position.x + next.0,
                y: current_position.y + next.1,
            };
            if !grid.contains(&next_position) {
                path.push(next_position);
                next_found = true;
                break;
            }
        }
        if !next_found {
            grid.insert(current_position);
            path.pop();
        }
    }
    grid.len() - count_elements
}

fn part2(input: &str) -> usize {
    let (_, pathes) = parse(input).unwrap();
    let mut grid = create_grid(&pathes);
    let count_elements = grid.len();
    let max_y = grid.iter().max_by_key(|p| p.y).unwrap().y + 2;
    let start_point = Point { x: 500, y: 0 };
    let mut path = vec![start_point];
    while !grid.contains(&start_point) {
        let current_position = *path.last().unwrap();
        let mut next_found = false;
        for next in [(0, 1), (-1, 1), (1, 1)] {
            let next_position = Point {
                x: current_position.x + next.0,
                y: current_position.y + next.1,
            };
            if !(grid.contains(&next_position) || next_position.y >= max_y) {
                path.push(next_position);
                next_found = true;
                break;
            }
        }
        if !next_found {
            grid.insert(current_position);
            path.pop();
        }
    }
    grid.len() - count_elements
}

fn create_grid(pathes: &Vec<Vec<Point>>) -> BTreeSet<Point> {
    let mut set = BTreeSet::new();
    for path in pathes {
        for (first, second) in path.iter().tuple_windows() {
            set.extend(first.path_between(second));
        }
    }
    set
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn path_between(&self, other: &Point) -> Vec<Point> {
        let (delta_x, delta_y) = (self.x - other.x, self.y - other.y);
        let counter = delta_x.abs().max(delta_y.abs());
        let mut start = *self;
        let mut result = vec![start];
        for _ in 0..counter {
            start.x += -delta_x.signum();
            start.y += -delta_y.signum();
            result.push(start);
        }

        result
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    let point = separated_pair(complete::i32, tag(","), complete::i32);
    let point = map(point, |(x, y)| Point { x, y });
    let path = separated_list1(tag(" -> "), point);
    separated_list1(multispace1, path)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse() {
        let input = indoc! {"
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        "};

        let (_, result) = parse(input).unwrap();
        assert_eq!(2, result.len());
        assert_eq!(Point { x: 498, y: 4 }, result[0][0]);
    }

    #[test]
    fn test_point_path_between() {
        let path = Point { x: 498, y: 4 }.path_between(&Point { x: 498, y: 6 });
        let expected = vec![
            Point { x: 498, y: 4 },
            Point { x: 498, y: 5 },
            Point { x: 498, y: 6 },
        ];
        assert_eq!(expected, path);

        let path = Point { x: 498, y: 6 }.path_between(&Point { x: 498, y: 4 });
        let expected = vec![
            Point { x: 498, y: 6 },
            Point { x: 498, y: 5 },
            Point { x: 498, y: 4 },
        ];
        assert_eq!(expected, path);

        let path = Point { x: 502, y: 4 }.path_between(&Point { x: 502, y: 9 });
        let expected = vec![
            Point { x: 502, y: 4 },
            Point { x: 502, y: 5 },
            Point { x: 502, y: 6 },
            Point { x: 502, y: 7 },
            Point { x: 502, y: 8 },
            Point { x: 502, y: 9 },
        ];
        assert_eq!(expected, path);

        let path = Point { x: 502, y: 9 }.path_between(&Point { x: 494, y: 9 });
        let expected = vec![
            Point { x: 502, y: 9 },
            Point { x: 501, y: 9 },
            Point { x: 500, y: 9 },
            Point { x: 499, y: 9 },
            Point { x: 498, y: 9 },
            Point { x: 497, y: 9 },
            Point { x: 496, y: 9 },
            Point { x: 495, y: 9 },
            Point { x: 494, y: 9 },
        ];
        assert_eq!(expected, path);
    }

    #[test]
    fn test_create_grid() {
        let input = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};

        let (_, pathes) = parse(input).unwrap();
        let grid = create_grid(&pathes);
        let expected_points = vec![
            Point { x: 494, y: 9 },
            Point { x: 495, y: 9 },
            Point { x: 496, y: 9 },
            Point { x: 498, y: 9 },
            Point { x: 499, y: 9 },
            Point { x: 500, y: 9 },
            Point { x: 501, y: 9 },
            Point { x: 502, y: 9 },
            Point { x: 502, y: 8 },
            Point { x: 502, y: 7 },
            Point { x: 502, y: 6 },
            Point { x: 502, y: 5 },
            Point { x: 502, y: 4 },
        ];
        for p in expected_points {
            assert!(grid.contains(&p));
        }
        assert_eq!(20, grid.len());
    }

    #[test]
    fn test_part1() {
        let input = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};
        let units = part1(input);
        assert_eq!(24, units)
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};
        let units = part2(input);
        assert_eq!(93, units)
    }
}
