use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::many0,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

fn parse(s: &str) -> IResult<&str, (Vec<(u64, u64)>, Vec<u64>)> {
    let num = || map_res(digit1, str::parse);
    let range = separated_pair(num(), tag("-"), num());
    let fresh = many0(preceded(multispace0, range));
    let ids = many0(preceded(multispace0, num()));
    pair(fresh, ids)(s)
}

pub fn solve(s: &str) -> usize {
    let (fresh, ids) = parse(s).unwrap().1;
    ids.iter()
        .filter(|id| fresh.iter().any(|(l, h)| (l..=h).contains(id)))
        .count()
}

pub fn solve_2(s: &str) -> u64 {
    let (mut ranges, _) = parse(s).unwrap().1;
    ranges.sort();
    let mut i = 0;
    while i + 1 < ranges.len() {
        if ranges[i].1 < ranges[i + 1].0 {
            i += 1;
        } else {
            ranges[i].1 = u64::max(ranges[i].1, ranges[i + 1].1);
            ranges.remove(i + 1);
        }
    }
    ranges.into_iter().map(|(i, j)| j - i + 1).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 3);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 14);
    }
}
