use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");
fn main() {
    println!("{}", part1(INPUT, true, 20));
    println!("{}", part1(INPUT, false, 10_000));
}

fn part1(input: &str, divide_by_three: bool, rounds: usize) -> usize {
    let mut monkeys = parse_monkey(input);
    let mut inspected_items = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].clone();
            monkeys[i].items.clear();
            let items = monkey.inspect_items(divide_by_three);
            inspected_items[i] += items.len();
            for (level, index) in items {
                monkeys[index].items.push(level);
            }
        }
    }

    inspected_items.sort();
    inspected_items.iter().rev().take(2).product()
}

fn parse_monkey(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|m| m.parse::<Monkey>().unwrap())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    dividend: usize,
    throw_to: (usize, usize),
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        operation: Operation,
        dividend: usize,
        throw_to: (usize, usize),
    ) -> Self {
        Self {
            items,
            operation,
            dividend,
            throw_to,
        }
    }
}

impl Monkey {
    fn inspect_items(&mut self, divide_by_three: bool) -> Vec<(usize, usize)> {
        self.items
            .iter()
            .map(|&worry_level| self.operation.evaluate(worry_level))
            .map(|worry_level| {
                if divide_by_three {
                    worry_level / 3
                } else {
                    worry_level
                }
            })
            .map(|level| {
                if level % self.dividend == 0 {
                    (level, self.throw_to.0)
                } else {
                    (level, self.throw_to.1)
                }
            })
            .collect()
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items: Option<Vec<usize>> = None;
        let mut operation: Option<Operation> = None;
        let mut dividend: Option<usize> = None;
        let mut throw_true: Option<usize> = None;
        let mut throw_false: Option<usize> = None;

        for line in s.lines().map(str::trim) {
            let parts = line.split_ascii_whitespace().collect::<Vec<&str>>();
            match parts[..] {
                ["Monkey", _] => (),
                ["Starting", "items:", ..] => {
                    items = Some(
                        parts[2..]
                            .iter()
                            .map(|n| n.strip_suffix(',').unwrap_or(n).parse().unwrap())
                            .collect(),
                    );
                }
                ["Operation:", ..] => operation = line.parse().ok(),
                ["Test:", .., n] => dividend = n.parse().ok(),
                ["If", "true:", .., n] => throw_true = n.parse().ok(),
                ["If", "false:", .., n] => throw_false = n.parse().ok(),
                _ => return Err(format!("Can't parse line {line}")),
            }
        }
        Ok(Monkey::new(
            items.unwrap(),
            operation.unwrap(),
            dividend.unwrap(),
            (throw_true.unwrap(), throw_false.unwrap()),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operation {
    Multiply(usize),
    Add(usize),
    Square,
}

impl Operation {
    fn evaluate(&self, worry_level: usize) -> usize {
        match self {
            Operation::Multiply(n) => worry_level * n,
            Operation::Add(n) => worry_level + n,
            Operation::Square => worry_level * worry_level,
        }
    }
}
impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.trim().split_ascii_whitespace().collect::<Vec<&str>>();
        match items[..] {
            ["Operation:", .., "*", "old"] => Ok(Operation::Square),
            ["Operation:", .., "*", n] => Ok(Operation::Multiply(n.parse().unwrap())),
            ["Operation:", .., "+", n] => Ok(Operation::Add(n.parse().unwrap())),
            _ => Err(format!("Couldn't parse line {s}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operation() {
        let operation = "  Operation: new = old * 19".parse().unwrap();
        assert!(matches!(operation, Operation::Multiply(19)));
        let operation = "  Operation: new = old + 6".parse().unwrap();
        assert!(matches!(operation, Operation::Add(6)));
    }

    #[test]
    fn test_parse_monkey() {
        let monkey: Monkey = r#"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"#
            .parse()
            .unwrap();

        assert_eq!(
            Monkey {
                items: vec![79, 98],
                operation: Operation::Multiply(19),
                dividend: 23,
                throw_to: (2, 3),
            },
            monkey
        );
    }

    #[test]
    fn test_monkey_inspect_items() {
        let mut monkey = Monkey {
            items: vec![79, 98],
            operation: Operation::Multiply(19),
            dividend: 23,
            throw_to: (2, 3),
        };

        let result = monkey.inspect_items(true);
        assert_eq!(vec![(500, 3), (620, 3)], result);
    }
}
