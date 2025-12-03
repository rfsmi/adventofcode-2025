use nom::{
    character::complete::{anychar, multispace0},
    combinator::map_opt,
    multi::{many0, many1},
    sequence::preceded,
    IResult,
};

fn parse(s: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let digit = map_opt(anychar, |d| d.to_digit(10));
    many0(preceded(multispace0, many1(digit)))(s)
}

fn joltage<const N: usize>(bank: Vec<u32>) -> u64 {
    let mut sum = 0;
    let mut i = 0;
    for j in (0..N).rev() {
        i = (i..bank.len() - j).rev().max_by_key(|&i| bank[i]).unwrap();
        sum += bank[i] as u64 * 10u64.pow(j as u32);
        i += 1;
    }
    sum
}

pub fn solve(s: &str) -> u64 {
    parse(s).unwrap().1.into_iter().map(joltage::<2>).sum()
}

pub fn solve_2(s: &str) -> u64 {
    parse(s).unwrap().1.into_iter().map(joltage::<12>).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        987654321111111
        811111111111119
        234234234234278
        818181911112111";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 357);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 3121910778619);
    }
}
