use nom::{
    character::complete::{digit1, multispace0},
    combinator::{fail, map, map_res, recognize},
    multi::many0,
    sequence::{pair, preceded},
    IResult,
};

fn parse(s: &str) -> IResult<&str, Vec<i32>> {
    fn int(s: &str) -> IResult<&str, i32> {
        map_res(recognize(digit1), str::parse)(s)
    }
    fn dir(s: &str) -> IResult<&str, i32> {
        match s.split_at_checked(1) {
            Some(("L", rest)) => Ok((rest, -1)),
            Some(("R", rest)) => Ok((rest, 1)),
            _ => fail(s),
        }
    }
    many0(preceded(multispace0, map(pair(dir, int), |(a, b)| a * b)))(s)
}

pub fn solve(s: &str) -> u32 {
    let mut dial = 50;
    let mut zeros = 0;
    for turn in parse(s).unwrap().1 {
        dial += turn;
        dial = dial.rem_euclid(100);
        if dial == 0 {
            zeros += 1;
        }
    }
    zeros
}

pub fn solve_2(s: &str) -> i32 {
    let mut dial = 50;
    let mut zeros = 0;
    for turn in parse(s).unwrap().1 {
        if turn.is_negative() && dial == 0 {
            zeros -= 1;
        }
        dial += turn;
        zeros += match turn.is_negative() {
            true => (dial - 1).div_euclid(100).abs(),
            false => dial.div_euclid(100).abs(),
        };
        dial = dial.rem_euclid(100);
    }
    zeros
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 3);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 6);
    }
}
