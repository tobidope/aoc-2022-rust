use std::{collections::HashSet, iter::repeat};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", part1(INPUT));
}

fn part1(input: &str) -> usize {
    let mut head = [0, 0];
    let mut tail = [0, 0];
    let mut visited = HashSet::new();
    visited.insert(tail);

    for line in input.lines() {
        let (dir, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse::<usize>().unwrap();
        let head_steps = match (dir, steps) {
            ("R", n) => repeat(1).take(n).zip(repeat(0)).collect::<Vec<_>>(),
            ("L", n) => repeat(-1).take(n).zip(repeat(0)).collect::<Vec<_>>(),
            ("U", n) => repeat(0).zip(repeat(1).take(n)).collect::<Vec<_>>(),
            ("D", n) => repeat(0).zip(repeat(-1).take(n)).collect::<Vec<_>>(),
            _ => panic!("Unknown step {}", line),
        };

        for (x, y) in head_steps {
            head[0] += x;
            head[1] += y;

            let (diff_x, diff_y): (i32, i32) = (head[0] - tail[0], head[1] - tail[1]);

            if diff_x.abs() > 1 || diff_y.abs() > 1 {
                tail[0] += diff_x.signum();
                tail[1] += diff_y.signum();
                visited.insert(tail);
            }
        }
    }

    visited.len()
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
    }
}
