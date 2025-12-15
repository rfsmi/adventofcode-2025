use std::{collections::HashMap, iter::zip};

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

fn parity_presses(
    n: usize,
    buttons: &[Vec<usize>],
) -> HashMap<Vec<usize>, Vec<(usize, Vec<usize>)>> {
    let mut result: HashMap<Vec<usize>, Vec<(usize, Vec<usize>)>> = HashMap::new();
    for mask in 0u16..(1 << buttons.len()) {
        let mut value = vec![0; n];
        for (i, button) in buttons.iter().enumerate() {
            if mask & (1 << i) == 0 {
                continue;
            }
            for &j in button {
                value[j] += 1;
            }
        }
        let parity = value.iter().map(|v| v % 2).collect();
        let entry = result.entry(parity).or_default();
        entry.push((mask.count_ones() as usize, value));
    }
    result
}

fn bifurcate(target: Vec<usize>, buttons: Vec<Vec<usize>>) -> usize {
    enum DFS {
        Recurse { state: Vec<usize> },
        DoubleOffset { offset: usize },
        Min { state: Vec<usize>, n_nodes: usize },
    }
    let parity_cache = parity_presses(target.len(), &buttons);
    let mut memo: HashMap<Vec<usize>, Option<usize>> = HashMap::new();
    let mut retval: Vec<Option<usize>> = Vec::new();
    let mut stack: Vec<_> = vec![DFS::Recurse {
        state: target.clone(),
    }];
    while let Some(dfs) = stack.pop() {
        match dfs {
            DFS::Recurse { state } => {
                if let Some(&ret) = memo.get(&state) {
                    retval.push(ret);
                    continue;
                }
                if state.iter().all(|&v| v == 0) {
                    retval.push(Some(0));
                    continue;
                }
                let top = stack.len();
                let state_parity: Vec<_> = state.iter().map(|v| v % 2).collect();
                if let Some(presses) = parity_cache.get(&state_parity) {
                    for &(offset, ref adjust) in presses {
                        if zip(adjust, &state).any(|(a, b)| a > b) {
                            continue;
                        }
                        let mut state = state.clone();
                        zip(adjust, &mut state).for_each(|(a, b)| *b = (*b - *a) / 2);
                        stack.push(DFS::DoubleOffset { offset });
                        stack.push(DFS::Recurse { state });
                    }
                }
                let n_nodes = (stack.len() - top) / 2;
                stack.insert(top, DFS::Min { state, n_nodes });
            }
            DFS::DoubleOffset { offset } => {
                if let Some(ret) = retval.last_mut().unwrap() {
                    *ret = *ret * 2 + offset;
                }
            }
            DFS::Min { n_nodes, state } => {
                let nodes = retval.split_off(retval.len() - n_nodes);
                let min = nodes.into_iter().flatten().min();
                retval.push(min);
                memo.insert(state, min);
            }
        }
    }
    retval[0].unwrap()
}

pub fn solve(s: &str) -> usize {
    let machines = parse(s).unwrap().1;
    machines
        .into_iter()
        .map(|(target, buttons, _)| {
            let combos = parity_presses(target.len(), &buttons);
            combos
                .get(&target)
                .unwrap()
                .iter()
                .map(|&(n, _)| n)
                .min()
                .unwrap()
        })
        .sum()
}

pub fn solve_2(s: &str) -> usize {
    let machines = parse(s).unwrap().1;
    machines
        .into_iter()
        .enumerate()
        .map(|(i, (_, buttons, target))| {
            let result = bifurcate(target, buttons);
            println!("{i}: {result}");
            result
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        ";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 7);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 33);
    }
}
