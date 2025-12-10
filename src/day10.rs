use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, space0},
    combinator::{map_res, value},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

fn parse(s: &str) -> IResult<&str, Vec<(Vec<usize>, Vec<Vec<usize>>, Vec<usize>)>> {
    let nums = || separated_list1(tag(","), map_res(digit1, str::parse));
    let diagram = many1(alt((value(0, char('.')), value(1, char('#')))));
    let line = tuple((
        preceded(multispace0, delimited(tag("["), diagram, tag("]"))),
        many1(preceded(space0, delimited(tag("("), nums(), tag(")")))),
        preceded(space0, delimited(tag("{"), nums(), tag("}"))),
    ));
    many0(line)(s)
}

pub fn solve(s: &str) -> usize {
    todo!()
}
