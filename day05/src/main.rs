use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let (ship, instructions) = INPUT.split_once("\n\n").unwrap();
    let mut ship1: Ship = ship.parse().unwrap();
    let mut ship2 = ship1.clone();
    let instructions: Vec<Instruction> = instructions.lines().map(|l| l.parse().unwrap()).collect();
    for i in instructions {
        ship1.arrange(&i);
        ship2.arrange_multiple(&i);
    }
    println!("{}", ship1.top_crates());
    println!("{}", ship2.top_crates());
}
#[derive(Debug, Clone)]
struct Ship {
    stacks: Vec<Vec<u8>>,
}

impl FromStr for Ship {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(|l| l.as_bytes().to_vec()).collect::<Vec<_>>();
        let mut stacks: Vec<Vec<u8>> = Vec::new();
        let len = lines.last().unwrap().len();
        for i in (1..len).step_by(4) {
            let mut stack: Vec<u8> = Vec::new();
            for j in (0..lines.len() - 1).rev() {
                let line = &lines[j];
                let krate = line[i];
                if krate.is_ascii_whitespace() {
                    break;
                }
                stack.push(krate);
            }
            stacks.push(stack);
        }
        Ok(Ship { stacks })
    }
}

impl Ship {
    fn arrange(&mut self, instruction: &Instruction) {
        self.arrange_intern(instruction, true);
    }

    fn arrange_multiple(&mut self, instruction: &Instruction) {
        self.arrange_intern(instruction, false);
    }

    fn arrange_intern(&mut self, instruction: &Instruction, reversed: bool) {
        let from_stack = &mut self.stacks[instruction.from - 1];
        let to_remove = from_stack.len() - instruction.count..;
        let mut removed = from_stack.drain(to_remove).collect::<Vec<u8>>();
        if reversed {
            removed.reverse();
        }
        self.stacks[instruction.to - 1].extend(removed);
    }

    fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s[s.len() - 1] as char)
            .collect::<String>()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        let (from, to, count) = (
            parts[3].parse::<usize>().unwrap(),
            parts[5].parse::<usize>().unwrap(),
            parts[1].parse::<usize>().unwrap(),
        );
        Ok(Instruction { from, to, count })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship_from_str() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3  "#;
        let ship: Ship = input.parse().unwrap();
        assert_eq!(3, ship.stacks.len());
        assert_eq!(b'Z', ship.stacks[0][0]);
        assert_eq!(b'N', ship.stacks[0][1]);
        assert_eq!(b'D', ship.stacks[1][2]);
    }

    #[test]
    fn test_instruction_from_str() {
        let input = "move 1 from 2 to 1";
        let i: Instruction = input.parse().unwrap();
        assert_eq!(
            Instruction {
                from: 2,
                to: 1,
                count: 1
            },
            i
        );
    }

    #[test]
    fn test_ship_arrange() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3  "#;
        let mut ship: Ship = input.parse().unwrap();
        let i: Instruction = "move 1 from 2 to 1".parse().unwrap();
        ship.arrange(&i);
        assert_eq!(b'D', ship.stacks[0][2]);
    }

    #[test]
    fn test_ship_top_crates() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3  "#;
        let ship: Ship = input.parse().unwrap();
        assert_eq!("NDP", ship.top_crates());
    }
}
