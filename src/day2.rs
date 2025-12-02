use nom::{
    character::complete::{char, digit1, multispace0},
    combinator::{map_res, recognize},
    multi::separated_list0,
    sequence::{pair, preceded},
    IResult,
};

fn parse(s: &str) -> IResult<&str, Vec<(u64, u64)>> {
    fn int(s: &str) -> IResult<&str, u64> {
        map_res(recognize(digit1), str::parse)(s)
    }
    let range = pair(preceded(multispace0, int), preceded(char('-'), int));
    separated_list0(char(','), range)(s)
}

fn repeat(n: u64, count: u32) -> u64 {
    let scale = 10u64.pow(n.ilog10() + 1);
    n * (0..count).map(|i| scale.pow(i)).sum::<u64>()
}

fn next_invalid_id(id: u64, groups: u32) -> u64 {
    let length = id.checked_ilog10().unwrap_or(0) + 1;
    let part = if length % groups == 0 {
        id / 10u64.pow(length - length / groups)
    } else {
        1 * 10u64.pow(length / groups)
    };
    match repeat(part, groups) {
        next_id if next_id > id => next_id,
        _ => repeat(part + 1, groups),
    }
}

fn next_invalid_id_2(id: u64) -> u64 {
    let length = id.checked_ilog10().unwrap_or(0) + 1;
    (2..=length + 1)
        .map(|i| next_invalid_id(id, i))
        .min()
        .unwrap()
}

pub fn solve(s: &str) -> u64 {
    let mut sum = 0;
    for (l, h) in parse(s).unwrap().1 {
        let mut id = next_invalid_id(l - 1, 2);
        while id <= h {
            sum += id;
            id = next_invalid_id(id, 2);
        }
    }
    sum
}

pub fn solve_2(s: &str) -> u64 {
    let mut sum = 0;
    for (l, h) in parse(s).unwrap().1 {
        let mut id = next_invalid_id_2(l - 1);
        while id <= h {
            sum += id;
            id = next_invalid_id_2(id);
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 1227775554);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 4174379265);
    }

    #[test]
    fn test_next_invalid_id_2() {
        assert_eq!(next_invalid_id_2(99), 111);
    }
}
