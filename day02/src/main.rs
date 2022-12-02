const INPUT: &str = include_str!("../input.txt");

fn main() {
    let result: usize = INPUT.lines().map(|l| score(l.as_bytes())).sum();
    println!("{}", result);
    let result: usize = INPUT.lines().map(|l| score2(l.as_bytes())).sum();
    println!("{}", result);
}

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    fn wins_against(&self, other: &Move) -> GameResult {
        match (self, other) {
            (Move::Rock, Move::Scissor)
            | (Move::Paper, Move::Rock)
            | (Move::Scissor, Move::Paper) => GameResult::Win,
            _ => {
                if self == other {
                    GameResult::Draw
                } else {
                    GameResult::Loss
                }
            }
        }
    }

    fn score(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }

    fn move_for_outcome(&self, expected: &GameResult) -> Self {
        if Move::Rock.wins_against(self) == *expected {
            Move::Rock
        } else if Move::Paper.wins_against(self) == *expected {
            Move::Paper
        } else {
            Move::Scissor
        }
    }
}

impl From<u8> for Move {
    fn from(c: u8) -> Self {
        match c {
            b'A' | b'X' => Move::Rock,
            b'B' | b'Y' => Move::Paper,
            b'C' | b'Z' => Move::Scissor,
            _ => panic!("Unknown Move"),
        }
    }
}
#[derive(Debug, PartialEq)]
enum GameResult {
    Draw,
    Win,
    Loss,
}

impl GameResult {
    fn score(&self) -> usize {
        match self {
            GameResult::Draw => 3,
            GameResult::Win => 6,
            GameResult::Loss => 0,
        }
    }
}
impl From<u8> for GameResult {
    fn from(c: u8) -> Self {
        match c {
            b'X' => GameResult::Loss,
            b'Y' => GameResult::Draw,
            b'Z' => GameResult::Win,
            _ => panic!("Unknown GameResult"),
        }
    }
}

fn score(score_line: &[u8]) -> usize {
    let (opponent, player) = (Move::from(score_line[0]), Move::from(score_line[2]));
    let outcome = player.wins_against(&opponent);
    player.score() + outcome.score()
}

fn score2(score_line: &[u8]) -> usize {
    let (opponent, outcome) = (Move::from(score_line[0]), GameResult::from(score_line[2]));
    let player = opponent.move_for_outcome(&outcome);
    player.score() + outcome.score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(8, score("A Y".as_bytes()));
    }

    #[test]
    fn test_score2() {
        assert_eq!(4, score2("A Y".as_bytes()));
        assert_eq!(1, score2("B X".as_bytes()));
        assert_eq!(7, score2("C Z".as_bytes()));
    }

    #[test]
    fn test_move_wins() {
        assert_eq!(GameResult::Draw, Move::Paper.wins_against(&Move::Paper));
        assert_eq!(GameResult::Draw, Move::Rock.wins_against(&Move::Rock));
        assert_eq!(GameResult::Draw, Move::Scissor.wins_against(&Move::Scissor));
        assert_eq!(GameResult::Win, Move::Scissor.wins_against(&Move::Paper));
        assert_eq!(GameResult::Loss, Move::Scissor.wins_against(&Move::Rock));
    }
}
