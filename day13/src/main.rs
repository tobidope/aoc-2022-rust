use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Int(usize),
    List(Vec<Packet>),
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    fn from(expression: &[u8]) -> Self {
        let mut stack = vec![];
        let mut index = 1;

        while index < expression.len() {
            match expression[index] {
                b'0'..=b'9' => {
                    let digits = expression
                        .iter()
                        .take_while(|&d| d.is_ascii_digit())
                        .cloned()
                        .collect::<Vec<_>>();
                    index += digits.len();
                    let digits = String::from_utf8(digits).unwrap();
                    stack.push(Packet::Int(digits.parse().unwrap()))
                }
                b'[' => stack.push(Self::from(&expression[index..])),
                b']' => (),
                b',' | b' ' => (),
                _ => todo!(),
            }
        }
        Packet::List(stack)
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
        assert!(Int(1) <= Int(2));
        assert!(Int(1) < Int(2));
        assert!(Int(1) <= Int(2));
        assert!(Int(2) > Int(1));

        assert!(Int(1) <= List(vec![Int(2)]));
        assert!(List(vec![Int(2), Int(3), Int(4)]) < Int(4));

        assert!(List(vec![Int(7), Int(7), Int(7), Int(7)]) > List(vec![Int(7), Int(7), Int(7)]));
    }

    #[test]
    fn test_packet_from() {
        assert_eq!(List(vec![]), Packet::from("[]".as_bytes()));
    }
}
