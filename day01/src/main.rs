const INPUT: &str = include_str!("../input.txt");
fn main() {
    println!("{}", elve_calories_max(INPUT, 1));
    println!("{}", elve_calories_max(INPUT, 3));
}

fn elve_calories_max(input: &str, elves: usize) -> usize {
    let mut calories = input
        .split("\n\n")
        .map(|elve| {
            elve.lines()
                .map(|calorie| calorie.parse::<usize>().unwrap())
                .sum()
        })
        .collect::<Vec<_>>();
    calories.sort();
    calories.iter().rev().take(elves).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elf_with_highest_calories() {
        assert_eq!(74711, elve_calories_max(INPUT, 1));
    }

    #[test]
    fn sum_of_three_highest_calories() {
        assert_eq!(209481, elve_calories_max(INPUT, 3));
    }
}
