use std::{
    collections::HashSet,
    iter::{empty, repeat},
};

use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

const INPUT: &str = include_str!("../input.txt");
fn main() {
    println!("{}", part1(INPUT, 2000000))
}
fn part1(input: &str, row: i32) -> usize {
    let sensors = input
        .lines()
        .map(|l| parse_sensor(l).unwrap().1)
        .collect::<Vec<_>>();
    let no_beacons: HashSet<(i32, i32)> = sensors.iter().flat_map(|s| s.no_beacons(row)).collect();
    no_beacons.len()
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

    fn no_beacons(&self, row: i32) -> Box<dyn Iterator<Item = (i32, i32)> + '_> {
        let beacon_distance = self.distance_to_beacon();
        let distance = (self.position.1 - row).abs();
        if distance > beacon_distance {
            Box::new(empty::<(i32, i32)>())
        } else {
            let left = self.position.0 - (beacon_distance - distance);
            let right = self.position.0 + (beacon_distance - distance);
            Box::new(
                (left..=right)
                    .zip(repeat(row))
                    .filter(|&p| p != self.beacon),
            )
        }
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
    use indoc::indoc;

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

    #[test]
    fn test_part1() {
        let input = indoc! {"
            Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            Sensor at x=9, y=16: closest beacon is at x=10, y=16
            Sensor at x=13, y=2: closest beacon is at x=15, y=3
            Sensor at x=12, y=14: closest beacon is at x=10, y=16
            Sensor at x=10, y=20: closest beacon is at x=10, y=16
            Sensor at x=14, y=17: closest beacon is at x=10, y=16
            Sensor at x=8, y=7: closest beacon is at x=2, y=10
            Sensor at x=2, y=0: closest beacon is at x=2, y=10
            Sensor at x=0, y=11: closest beacon is at x=2, y=10
            Sensor at x=20, y=14: closest beacon is at x=25, y=17
            Sensor at x=17, y=20: closest beacon is at x=21, y=22
            Sensor at x=16, y=7: closest beacon is at x=15, y=3
            Sensor at x=14, y=3: closest beacon is at x=15, y=3
            Sensor at x=20, y=1: closest beacon is at x=15, y=3"
        };

        assert_eq!(26, part1(input, 10));
    }
}
