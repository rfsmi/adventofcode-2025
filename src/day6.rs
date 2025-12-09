use std::iter::zip;

use nom::{
    character::complete::{digit1, multispace0, satisfy, space0},
    combinator::map_res,
    multi::{many0, many1},
    sequence::{pair, preceded},
    IResult,
};

fn parse(s: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<char>)> {
    let num = || preceded(space0, map_res(digit1, str::parse));
    let nums = many0(preceded(multispace0, many1(num())));
    let op = preceded(space0, satisfy(|c| c == '*' || c == '+'));
    let ops = preceded(multispace0, many1(op));
    pair(nums, ops)(s)
}

fn calculate(nums: Vec<Vec<u64>>, ops: Vec<char>) -> u64 {
    zip(ops, nums)
        .map(|(op, nums)| match op {
            '*' => nums.iter().fold(1, |acc, n| acc * n),
            '+' => nums.iter().fold(0, |acc, n| acc + n),
            _ => panic!(),
        })
        .sum()
}

pub fn solve(s: &str) -> u64 {
    let (nums, ops) = parse(s).unwrap().1;
    let mut nums_t = Vec::new();
    for x in 0..nums[0].len() {
        nums_t.push(vec![]);
        for y in 0..nums.len() {
            nums_t.last_mut().unwrap().push(nums[y][x]);
        }
    }
    calculate(nums_t, ops)
}

pub fn solve_2(s: &str) -> u64 {
    let (_, ops) = parse(s).unwrap().1;
    let s: Vec<Vec<_>> = s.lines().map(|l| l.chars().collect()).collect();
    let mut nums: Vec<Vec<u64>> = vec![Vec::new()];
    for x in 0..s[0].len() {
        let Some(num) = (0..s.len() - 1)
            .filter_map(|y| s[y][x].to_digit(10).map(|d| d as u64))
            .reduce(|acc, d| acc * 10 + d)
        else {
            nums.push(Vec::new());
            continue;
        };
        nums.last_mut().unwrap().push(num);
    }
    calculate(nums, ops)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 4277556);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 3263827);
    }
}
