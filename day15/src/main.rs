use std::collections::BinaryHeap;

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
fn part1(input: &str, row: i32) -> i32 {
    let ranges = input
        .lines()
        .map(parse_sensor)
        .flat_map(|s| s.no_beacons(row))
        .collect::<BinaryHeap<_>>()
        .into_sorted_vec();
    ranges[1..]
        .iter()
        .fold(vec![ranges[0]], |mut acc, &current| {
            let previous = acc.last_mut().unwrap();
            if previous.1 < current.0 {
                acc.push(current);
            } else {
                *previous = (previous.0.min(current.0), previous.1.max(current.1));
            }
            acc
        })
        .iter()
        .map(|&(l, r)| r - l + 1)
        .sum::<i32>()
        - 1
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Sensor {
    position: (i32, i32),
    beacon: (i32, i32),
    distance: i32,
}

impl Sensor {
    fn new(position: (i32, i32), beacon: (i32, i32)) -> Self {
        Self {
            position,
            beacon,
            distance: (position.0 - beacon.0).abs() + (position.1 - beacon.1).abs(),
        }
    }

    fn no_beacons(&self, row: i32) -> Option<(i32, i32)> {
        let distance = (self.position.1 - row).abs();
        if distance > self.distance {
            None
        } else {
            let left = self.position.0 - (self.distance - distance);
            let right = self.position.0 + (self.distance - distance);
            Some((left, right))
        }
    }
}

fn parse_sensor(input: &str) -> Sensor {
    let Ok((_, result)) = map(
        separated_pair(
            preceded(tag("Sensor at "), parse_coordinate),
            tag(": closest beacon is at "),
            parse_coordinate,
        ),
        |(position, beacon)| Sensor::new(position, beacon),
    )(input) else {
        panic!("Couldn't parse sensor: {input}")
    };
    result
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
        let sensor = parse_sensor("Sensor at x=2, y=18: closest beacon is at x=-2, y=15");
        assert_eq!(Sensor::new((2, 18), (-2, 15)), sensor)
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
