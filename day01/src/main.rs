use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../input.txt");
fn main() {
    println!("{}", elve_calories_max(INPUT, 1));
    println!("{}", elve_calories_max(INPUT, 3));
}

fn elve_calories_max(input: &str, elves: usize) -> usize {
    input
        .split("\n\n")
        .map(|elve| {
            elve.lines()
                .map(|calorie| calorie.parse::<usize>().unwrap())
                .sum()
        })
        .collect::<BinaryHeap<_>>()
        .into_sorted_vec()
        .iter()
        .rev()
        .take(elves)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(74711, elve_calories_max(INPUT, 1));
    }

    #[test]
    fn test_part2() {
        assert_eq!(209481, elve_calories_max(INPUT, 3));
    }
}
