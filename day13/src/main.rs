use std::cmp::Ordering;

const INPUT: &str = include_str!("../input.txt");
fn main() {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Packet::from)
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .filter_map(|(index, pair)| {
            if pair[0] < pair[1] {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Packet::from)
        .collect::<Vec<_>>();
    let p1 = Packet::from("[[2]]");
    let p2 = Packet::from("[[6]]");
    packets.push(p1.clone());
    packets.push(p2.clone());
    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if *p == p1 || *p == p2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Int(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn from(expression: &str) -> Self {
        fn parse(expression: &[u8]) -> (usize, Packet) {
            let mut packets = vec![];
            let mut index = 1;

            while index < expression.len() {
                match expression[index] {
                    b'0'..=b'9' => {
                        let digits = expression[index..]
                            .iter()
                            .take_while(|&d| d.is_ascii_digit())
                            .cloned()
                            .collect::<Vec<_>>();
                        index += digits.len();
                        let digits = String::from_utf8(digits).unwrap();
                        packets.push(Packet::Int(digits.parse().unwrap()))
                    }
                    b'[' => {
                        let (consumed, packet) = parse(&expression[index..]);
                        packets.push(packet);
                        index += consumed;
                    }
                    b']' => {
                        index += 1;
                        break;
                    }
                    b',' => index += 1,
                    _ => unreachable!(),
                }
            }
            (index, Packet::List(packets))
        }

        let (_, p) = parse(expression.as_bytes());
        p
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Packet::*;
        match (self, other) {
            (Int(left), Int(right)) => left.cmp(right),
            (Int(_), List(_)) => List(vec![self.clone()]).cmp(other),
            (List(_), Int(_)) => self.cmp(&List(vec![other.clone()])),
            (List(left), List(right)) => {
                for (l, r) in left.iter().zip(right.iter()) {
                    let ordering = l.cmp(r);
                    if ordering != Ordering::Equal {
                        return ordering;
                    }
                }
                left.len().cmp(&right.len())
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use Packet::*;

    #[test]
    fn test_ordering() {
        assert!(Int(1) < Int(2));
        assert!(Int(1) < Int(2));
        assert!(Int(1) < Int(2));
        assert!(Int(2) > Int(1));

        assert!(Int(1) < List(vec![Int(2)]));
        assert!(List(vec![Int(2), Int(3), Int(4)]) < Int(4));

        assert!(List(vec![Int(7), Int(7), Int(7), Int(7)]) > List(vec![Int(7), Int(7), Int(7)]));
    }

    #[test]
    fn test_packet_from() {
        assert_eq!(List(vec![]), Packet::from("[]"));
        assert_eq!(List(vec![Int(1)]), Packet::from("[1]"));
        assert_eq!(
            List(vec![Int(1), List(vec![Int(1), Int(3)])]),
            Packet::from("[1,[1,3]]")
        );
    }

    #[test]

    fn test_example() {
        assert!(Packet::from("[1,1,3,1,1]") < Packet::from("[1,1,5,1,1]"));
        assert!(Packet::from("[[1],[2,3,4]]") < Packet::from("[[1],4]"));
        assert!(Packet::from("[9]") > Packet::from("[[8,7,6]]"));
        assert!(Packet::from("[[4,4],4,4]") < Packet::from("[[4,4],4,4,4]"));
        assert!(Packet::from("[7,7,7,7]") > Packet::from("[7,7,7]"));
        assert!(Packet::from("[]") < Packet::from("[3]"));
        assert!(Packet::from("[[[]]]") > Packet::from("[[]]"));
        assert!(
            Packet::from("[1,[2,[3,[4,[5,6,7]]]],8,9]")
                > Packet::from("[1,[2,[3,[4,[5,6,0]]]],8,9]")
        );
    }
}
