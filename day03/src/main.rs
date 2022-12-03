use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");
fn main() {
    let result: usize = INPUT.lines().map(part1).sum();
    println!("{}", result);
    let result = part2(INPUT);
    println!("{}", result);
}

fn part1(input: &str) -> usize {
    let mid = input.len() / 2;
    let rucksack1 = &input[..mid];
    let rucksack2 = &input[mid..];

    rucksack1
        .bytes()
        .find(|&c| rucksack2.contains(c as char))
        .map(|c| priority(&c))
        .unwrap() as usize
}

fn part2(input: &str) -> usize {
    let mut result = 0;
    let mut set: HashSet<u8> = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        if i % 3 == 0 && !set.is_empty() {
            let char = &set.drain().next().unwrap();
            result += priority(char);
        }
        let new_set = line.bytes().collect::<HashSet<u8>>();
        set = if !set.is_empty() {
            set.intersection(&new_set).copied().collect()
        } else {
            new_set
        };
    }
    let char = &set.drain().next().unwrap();
    result += priority(char);
    result
}

fn priority(c: &u8) -> usize {
    if c.is_ascii_lowercase() {
        c - b'a' + 1
    } else {
        c - b'A' + 27
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(16, part1("vJrwpWtwJgWrhcsFMMfFFhFp"));
        assert_eq!(38, part1("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
    }

    #[test]
    fn test_part2() {
        let example = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(70, part2(example));
    }
}
