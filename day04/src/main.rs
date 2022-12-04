use std::{
    cmp::{max, min},
    ops::RangeInclusive,
    str::FromStr,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let input: Vec<Assignment> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
    let result = part1(&input);
    println!("{result}");
    let result = part2(&input);
    println!("{result}");
}

fn part1(input: &[Assignment]) -> usize {
    input.iter().filter(|a| a.fully_contains()).count()
}

fn part2(input: &[Assignment]) -> usize {
    input.iter().filter(|a| a.is_overlapping()).count()
}

#[derive(Debug, PartialEq, Eq)]
struct Assignment {
    first: RangeInclusive<usize>,
    second: RangeInclusive<usize>,
}

impl Assignment {
    fn fully_contains(&self) -> bool {
        (self.first.contains(self.second.start()) && self.first.contains(self.second.end()))
            || (self.second.contains(self.first.start()) && self.second.contains(self.first.end()))
    }

    fn is_overlapping(&self) -> bool {
        let left = max(self.first.start(), self.second.start());
        let right = min(self.first.end(), self.second.end());
        left <= right
    }
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split(&[',', '-'])
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if numbers.len() != 4 {
            Err(format!("Couldn't parse {}", s))
        } else {
            Ok(Self {
                first: numbers[0]..=numbers[1],
                second: numbers[2]..=numbers[3],
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlapping() {
        let a = Assignment {
            first: (2..=8),
            second: (3..=7),
        };
        assert!(a.fully_contains());

        let a = Assignment {
            first: 2..=4,
            second: 6..=8,
        };
        assert!(!a.fully_contains())
    }

    #[test]
    fn test_from_str() {
        let a: Assignment = "2-8,3-7".parse().unwrap();
        assert_eq!(
            Assignment {
                first: 2..=8,
                second: 3..=7
            },
            a
        );
    }

    #[test]
    fn test_is_overlapping() {
        let a: Assignment = "5-7,7-9".parse().unwrap();
        assert!(a.is_overlapping());
        let a: Assignment = "2-4,6-8".parse().unwrap();
        assert!(!a.is_overlapping());
    }
}
