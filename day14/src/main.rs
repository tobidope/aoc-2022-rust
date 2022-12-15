use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    separated_list1(multispace1, parse_path)(input)
}

fn parse_path(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), parse_point)(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let parser = separated_pair(parse_number, tag(","), parse_number);
    map(parser, |(x, y)| Point { x, y })(input)
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |n: &str| n.parse())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert_eq!(Ok((",1", 2)), parse_number("2,1"));
    }

    #[test]
    fn test_parse_point() {
        let result = parse_point("2,1 ->");
        assert_eq!(Ok((" ->", Point { x: 2, y: 1 })), result);
    }

    #[test]
    fn test_parse_path() {
        let path = "503,4 -> 502,4 -> 502,9 -> 494,9";
        let path = parse_path(path);
        assert_eq!(
            Ok((
                "",
                vec![
                    Point { x: 503, y: 4 },
                    Point { x: 502, y: 4 },
                    Point { x: 502, y: 9 },
                    Point { x: 494, y: 9 }
                ]
            )),
            path
        );
    }

    #[test]
    fn test_parse() {}
}
