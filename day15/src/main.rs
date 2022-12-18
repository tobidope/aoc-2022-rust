use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

const INPUT: &str = include_str!("../input.txt");
fn main() {
    println!("Hello, world!");
}
fn part1(input: &str, row: i32) -> usize {
    let sensors = input
        .lines()
        .map(|l| parse_sensor(l).unwrap().1)
        .collect::<Vec<_>>();
    sensors.iter();
    0
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Sensor {
    position: (i32, i32),
    beacon: (i32, i32),
}

impl Sensor {
    fn distance_to_beacon(&self) -> i32 {
        (self.position.0 - self.beacon.0).abs() + (self.position.1 - self.beacon.1).abs()
    }
}

fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    map(
        separated_pair(
            preceded(tag("Sensor at "), parse_coordinate),
            tag(": closest beacon is at "),
            parse_coordinate,
        ),
        |(position, beacon)| Sensor { position, beacon },
    )(input)
}

fn parse_coordinate(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        preceded(tag("x="), complete::i32),
        tag(", "),
        preceded(tag("y="), complete::i32),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sensor() {
        let (_, sensor) =
            parse_sensor("Sensor at x=2, y=18: closest beacon is at x=-2, y=15").unwrap();
        assert_eq!(
            Sensor {
                position: (2, 18),
                beacon: (-2, 15)
            },
            sensor
        )
    }
}
