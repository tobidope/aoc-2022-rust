const INPUT: &str = include_str!("../input.txt");
fn main() {
    println!("{}", max_calories(INPUT));
    println!("{}", top_three_calories(INPUT));
}

fn max_calories(input: &str) -> usize {
    let mut max: usize = 0;
    let mut current: usize = 0;
    for line in input.lines() {
        if line.is_empty() {
            max = if current > max { current } else { max };
            current = 0;
        } else {
            let weight: usize = line.parse().unwrap();
            current += weight;
        }
    }
    max
}

fn top_three_calories(input: &str) -> usize {
    let mut calories = Vec::new();

    let mut current = 0;
    for line in input.lines() {
        if line.is_empty() {
            calories.push(current);
            current = 0;
        } else {
            let weight: usize = line.parse().unwrap();
            current += weight;
        }
    }
    calories.sort();
    let len = calories.len();
    calories[len - 3] + calories[len - 2] + calories[len - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elf_with_highest_calories() {
        assert_eq!(74711, max_calories(INPUT));
    }

    #[test]
    fn sum_of_three_highest_calories() {
        assert_eq!(209481, top_three_calories(INPUT));
    }
}
