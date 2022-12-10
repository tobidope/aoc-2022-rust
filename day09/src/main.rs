use std::{
    collections::{HashSet, VecDeque},
    iter::repeat,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(tail);

    for (x, y) in parse_steps(input) {
        head = (head.0 + x, head.1 + y);

        if let Some(new_tail) = needs_to_move(&head, &tail) {
            tail = new_tail;
            visited.insert(tail);
        }
    }

    visited.len()
}

fn part2(input: &str) -> usize {
    let mut head = (0, 0);
    let mut deque = VecDeque::from([(0, 0); 10]);
    let mut visited = HashSet::new();
    visited.insert(head);

    for step in parse_steps(input) {
        head = deque.pop_front().unwrap();
        head = (head.0 + step.0, head.1 + step.1);
        let mut current = head;
        for previous in deque.iter_mut() {
            if let Some(new) = needs_to_move(&current, previous) {
                *previous = new;
            }
            current = *previous;
        }
        deque.push_front(head);
        visited.insert(*deque.back().unwrap());
    }

    visited.len()
}

fn parse_steps(input: &str) -> impl Iterator<Item = (i32, i32)> + '_ {
    input
        .lines()
        .map(|line| {
            let (dir, steps) = line.split_once(' ').unwrap();
            let steps = steps.parse::<usize>().unwrap();
            (dir, steps)
        })
        .flat_map(|(dir, steps)| match (dir, steps) {
            ("R", n) => repeat(1).take(n).zip(repeat(0)).collect::<Vec<_>>(),
            ("L", n) => repeat(-1).take(n).zip(repeat(0)).collect::<Vec<_>>(),
            ("U", n) => repeat(0).zip(repeat(1).take(n)).collect::<Vec<_>>(),
            ("D", n) => repeat(0).zip(repeat(-1).take(n)).collect::<Vec<_>>(),
            _ => panic!("Unknown step {} {}", dir, steps),
        })
}

fn needs_to_move(head: &(i32, i32), tail: &(i32, i32)) -> Option<(i32, i32)> {
    let (diff_x, diff_y): (i32, i32) = (head.0 - tail.0, head.1 - tail.1);
    if diff_x.abs() > 1 || diff_y.abs() > 1 {
        Some((tail.0 + diff_x.signum(), tail.1 + diff_y.signum()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

        assert_eq!(13, part1(input));
        assert_eq!(6339, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

        assert_eq!(1, part2(input));
    }
}
