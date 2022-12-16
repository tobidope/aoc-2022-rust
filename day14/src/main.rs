use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    let point = separated_pair(complete::i32, tag(","), complete::i32);
    let point = map(point, |(x, y)| Point { x, y });
    let path = separated_list1(tag(" -> "), point);
    separated_list1(multispace1, path)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse() {
        let input = indoc! {"
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        "};

        let (_, result) = parse(input).unwrap();
        assert_eq!(2, result.len());
        assert_eq!(Point { x: 498, y: 4 }, result[0][0]);
    }
}
