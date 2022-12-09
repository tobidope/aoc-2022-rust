use std::{
    collections::HashSet,
    ops::{Add, Sub},
    str::FromStr,
};

const INPUT: &str = include_str!("../example.txt");

fn main() {
    let mut head = Position::default();
    let mut tail = Position::default();
    let mut visited = HashSet::new();
    visited.insert(tail);

    let steps: Vec<Direction> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
    for step in steps {
        for p in head.step_positions(&step) {
            head = head + p;
            match head {
                Position { x: 0, y: 0 } => (),
                Position { x, y: 0 } if x.abs() > 1 => {
                    tail.x += x;
                }
                Position { x: 0, y } if y.abs() > 1 => {
                    tail.y += y;
                }
                Position { x, y } if x.abs() == 2 || y.abs() == 2 => {
                    tail.x += x.signum();
                    tail.y += y.signum();
                }
                _ => (),
            }
        }
        visited.insert(tail);
    }
    dbg!(visited);
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step_positions(&self, d: &Direction) -> Vec<Position> {
        let (step, size) = match d {
            Direction::Right(n) => (Position { x: 1, y: 0 }, *n),
            Direction::Left(n) => (Position { x: -1, y: 0 }, *n),
            Direction::Up(n) => (Position { x: 0, y: 1 }, *n),
            Direction::Down(n) => (Position { x: 0, y: -11 }, *n),
        };
        let mut position = vec![];
        let mut current = *self;
        for _ in 0..size {
            current = current + step;
            position.push(current);
        }

        position
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s.split_once(' ').unwrap();
        let amount = amount.parse::<u32>().unwrap();
        match (direction, amount) {
            ("R", n) => Ok(Direction::Right(n)),
            ("L", n) => Ok(Direction::Left(n)),
            ("U", n) => Ok(Direction::Up(n)),
            ("D", n) => Ok(Direction::Down(n)),
            _ => Err(format!("Couldn't parse {}", s)),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Right(u32),
    Left(u32),
    Up(u32),
    Down(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(Ok(Direction::Right(1)), "R 1".parse());
        assert_eq!(Ok(Direction::Left(1)), "L 1".parse());
        assert_eq!(Ok(Direction::Up(1)), "U 1".parse());
        assert_eq!(Ok(Direction::Down(1)), "D 1".parse());
    }
}
