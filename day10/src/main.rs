use std::{fmt::Display, str::FromStr};

const INPUT: &str = include_str!("../input.txt");
fn main() {
    println!("{}", part1(INPUT));
}

fn part1(input: &str) -> i32 {
    let instructions = parse_instructions(input);
    let mut circuit = Circuit::new(&instructions);
    circuit.run()
}

const WIDTH: usize = 40;
const HEIGHT: usize = 6;
struct Circuit {
    instructions: Vec<Instruction>,
    cycle: i32,
    register: i32,
    crt: Crt,
    pixel_postion: i32,
}

impl Circuit {
    fn new(instructions: &[Instruction]) -> Self {
        Self {
            instructions: instructions.to_vec(),
            cycle: 0,
            register: 1,
            crt: Crt::new(),
            pixel_postion: 0,
        }
    }

    fn run(&mut self) -> i32 {
        let mut signal_strength = 0;
        for instruction in self.instructions.iter() {
            for _ in 0..instruction.cycles() {
                self.cycle += 1;
                self.crt.draw(self.cycle, self.register);
                self.pixel_postion += 1;
                if self.cycle == 20 || (self.cycle - 20) % 40 == 0 {
                    signal_strength += self.register * self.cycle as i32
                }
            }
            self.register = instruction.change_register(&self.register)
        }
        println!("{}", self.crt);
        signal_strength
    }
}

struct Crt {
    display: [char; WIDTH * HEIGHT],
}
impl Crt {
    fn new() -> Self {
        Self {
            display: [' '; WIDTH * HEIGHT],
        }
    }

    fn draw(&mut self, cycle: i32, register: i32) -> char {
        let pixel: usize = (cycle - 1).try_into().unwrap();
        let sprite = register - 1..register + 2;
        let position: i32 = (pixel % WIDTH) as i32;
        let visible = sprite.contains(&position);
        self.display[pixel] = if visible { '#' } else { '.' };
        self.display[pixel]
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = self
            .display
            .chunks(WIDTH)
            .map(|line| String::from_iter(line.iter()))
            .collect();
        write!(f, "{}", lines.join("\n"))
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    AddX(i32),
    Noop,
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::Noop => 1,
        }
    }

    fn change_register(&self, register: &i32) -> i32 {
        match self {
            Instruction::AddX(n) => register + n,
            Instruction::Noop => *register,
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>()
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        match parts[..] {
            ["noop"] => Ok(Instruction::Noop),
            ["addx", n] => Ok(Instruction::AddX(n.parse().unwrap())),
            _ => Err(format!("Can't parse instruction {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register() {
        assert_eq!(Instruction::Noop, "noop".parse().unwrap());
        assert_eq!(Instruction::AddX(3), "addx 3".parse().unwrap());
    }

    #[test]
    fn test_circuit_run() {
        let instructions = r#"noop
addx 3
addx -5"#;
        let instructions = parse_instructions(instructions);
        let mut circuit = Circuit::new(&instructions);
        circuit.run();
        assert_eq!(-1, circuit.register);

        let instructions = parse_instructions(include_str!("../example.txt"));
        let mut circuit = Circuit::new(&instructions);
        let s = circuit.run();
        assert_eq!(13140, s);
    }

    #[test]
    fn test_crt_print() {
        let mut crt = Crt::new();
        crt.display[0] = '#';
        crt.display[40] = '#';
        println!("{crt}")
    }

    #[test]
    fn test_crt_draw() {
        let mut crt = Crt::new();
        let mut cycle = 1;
        let mut register = 1;
        assert_eq!('#', crt.draw(cycle, register));
        assert_eq!('#', crt.display[0]);
        cycle = 41;
        assert_eq!('#', crt.draw(cycle, register));
        assert_eq!('#', crt.display[40]);
        register = 5;
        cycle = 41;
        assert_eq!('.', crt.draw(cycle, register));
        assert_eq!('.', crt.display[40]);
    }
}
