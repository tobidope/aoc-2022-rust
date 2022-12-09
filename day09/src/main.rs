use std::{collections::HashSet, ops::Add, str::FromStr};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let head = Position::default();
    let tail = Position::default();
    let mut visited = HashSet::new();
    visited.insert(tail);

    let steps: Vec<Position> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(&mut self, rhs: &Position) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s.split_once(' ').unwrap();
        let amount = amount.parse::<i32>().unwrap();
        match (direction, amount) {
            ("R", n) => Ok(Position { x: n, y: 0 }),
            ("L", n) => Ok(Position { x: -n, y: 0 }),
            ("U", n) => Ok(Position { x: 0, y: n }),
            ("D", n) => Ok(Position { x: 0, y: -n }),
            _ => Err(format!("Couldn't parse {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Ok(Position { x: 1, y: 0 }), "R 1".parse());
        assert_eq!(Ok(Position { x: -1, y: 0 }), "L 1".parse());
        assert_eq!(Ok(Position { x: 0, y: 1 }), "U 1".parse());
        assert_eq!(Ok(Position { x: 0, y: -1 }), "D 1".parse());
    }
}
