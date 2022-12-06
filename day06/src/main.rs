use std::collections::HashSet;
const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

fn part2(input: &str) -> usize {
    find_marker(input, 14)
}

fn find_marker(input: &str, window_size: usize) -> usize {
    input
        .as_bytes()
        .windows(window_size)
        .map(|x| HashSet::from_iter(x.iter()))
        .position(|set: HashSet<&u8>| set.len() == window_size)
        .unwrap()
        + window_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(19, part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    }
}
