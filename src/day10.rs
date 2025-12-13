use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    fmt::Display,
};

use itertools::Itertools;
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

fn run(target: Vec<usize>, buttons: Vec<Vec<usize>>) -> usize {
    let mut queue: VecDeque<_> = [(0, vec![0; target.len()])].into_iter().collect();
    let mut seen = HashSet::new();
    while let Some((n, state)) = queue.pop_front() {
        if !seen.insert(state.clone()) {
            continue;
        }
        if state == target {
            return n;
        }
        for button in &buttons {
            let mut state = state.clone();
            for &i in button {
                state[i] = (state[i] + 1) % 2;
            }
            queue.push_back((n + 1, state));
        }
    }
    panic!()
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        (a, b) = (b, a.rem_euclid(b));
    }
    a.abs()
}

fn lcm(a: isize, b: isize) -> isize {
    a.abs() / gcd(a, b) * b.abs()
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for h in 0..self.m {
            if h != 0 {
                write!(f, "\n")?;
            }
            write!(f, "[")?;
            for k in 0..self.n - 1 {
                write!(f, "{:>3}", self.a[h][k])?;
            }
            write!(f, " |{:>3}]", self.a[h][self.n - 1])?;
        }
        Ok(())
    }
}

impl Display for ReducedMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for h in 0..self.m {
            if h != 0 {
                write!(f, "\n")?;
            }
            write!(f, "{:>3} [", self.leading[h])?;
            for k in 0..self.n {
                write!(f, "{:>3}", self.a[h][k])?;
            }
            write!(f, " ] ={:>3}", self.targets[h])?;
        }
        Ok(())
    }
}

struct Matrix {
    n: usize,
    m: usize,
    a: Vec<Vec<isize>>,
}

struct ReducedMatrix {
    n: usize,
    m: usize,
    targets: Vec<isize>,
    leading: Vec<isize>,
    a: Vec<Vec<isize>>,
}

impl Matrix {
    fn new(target: Vec<usize>, buttons: Vec<Vec<usize>>) -> Self {
        let m = target.len();
        let n = buttons.len() + 1;
        let mut a = vec![vec![0; n]; m];
        for (k, button) in buttons.into_iter().enumerate() {
            for h in button {
                a[h][k] = 1;
            }
        }
        for (h, t) in target.into_iter().enumerate() {
            a[h][n - 1] = t as isize;
        }
        Self { n, m, a }
    }

    fn reduce(Self { n, m, mut a }: Self) -> ReducedMatrix {
        let mut leading_positions = Vec::new();
        let mut a_col_positions = Vec::new();
        let (mut h, mut k) = (0, 0);
        while h < m && k < n - 1 {
            let Some(i_max) = (h..m).find(|&i| a[i][k] != 0) else {
                a_col_positions.push(k);
                k += 1;
                continue;
            };
            leading_positions.push((h, k));
            // println!("{}\n", Self { a: a.clone(), m, n });
            a.swap(h, i_max);
            for i in 0..m {
                if a[i][k] == 0 || i == h {
                    continue;
                }
                let f = lcm(a[i][k], a[h][k]);
                let (f1, f2) = (f / a[i][k], f / a[h][k]);
                (0..n).for_each(|j| a[i][j] = f1 * a[i][j] - f2 * a[h][j]);
                if let Some(..0) = a[i][0..n].iter().find(|&&v| v != 0) {
                    (0..n).for_each(|j| a[i][j] *= -1);
                }
                if let Some(d @ 2..) = a[i][0..n].iter().copied().reduce(gcd) {
                    (0..n).for_each(|j| a[i][j] /= d);
                }
            }
            h += 1;
            k += 1;
        }
        // println!("{}\n", Self { a: a.clone(), m, n });
        a_col_positions.extend(k..n - 1);
        let m = leading_positions.len();
        let targets = (0..m).map(|h| a[h][n - 1]).collect();
        let leading = leading_positions
            .into_iter()
            .map(|(h, k)| a[h][k])
            .collect();
        let a = (0..m)
            .map(|h| a_col_positions.iter().map(|&k| a[h][k]).collect())
            .collect();
        ReducedMatrix {
            n: a_col_positions.len(),
            m,
            targets,
            leading,
            a,
        }
    }
}

impl ReducedMatrix {
    fn solve(&self, presses: Vec<isize>) -> Option<isize> {
        let mut targets = self.targets.clone();
        for h in 0..self.m {
            for k in 0..presses.len() {
                targets[h] -= presses[k] * self.a[h][k];
            }
        }
        if presses.len() < self.n {
            let my_targets = || {
                (0..self.m)
                    .filter(|&h| self.a[h][presses.len()] != 0)
                    .filter(|&h| self.a[h][presses.len()].is_negative() == targets[h].is_negative())
                    .map(|h| targets[h].abs())
            };
            let h = my_targets().max().unwrap_or(0);
            return (0..=h)
                .filter_map(|guess| {
                    let mut presses = presses.clone();
                    presses.push(guess);
                    self.solve(presses)
                })
                .min();
        }
        if (0..self.m).any(|h| targets[h] < 0 || targets[h] % self.leading[h] != 0) {
            return None;
        }
        let presses = (0..self.m)
            .map(|h| targets[h] / self.leading[h])
            .chain(presses);
        Some(presses.sum())
    }
}

pub fn solve(s: &str) -> usize {
    let machines = parse(s).unwrap().1;
    machines
        .into_iter()
        .map(|(target, buttons, _)| run(target, buttons))
        .sum()
}

pub fn solve_2(s: &str) -> isize {
    let machines = parse(s).unwrap().1;
    machines
        .into_iter()
        .enumerate()
        // .skip(1)
        // .take(1)
        .map(|(i, (_, buttons, target))| {
            let m = Matrix::new(target, buttons);
            let m = Matrix::reduce(m);
            print!("Solving {i}:\n{m}\n");
            let result = m.solve(Vec::new()).unwrap();
            println!("{result}");
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
    fn test_matrix() {
        let machines = parse(SAMPLE).unwrap().1;
        for (_, buttons, target) in machines {
            let m = Matrix::new(target, buttons);
            println!("{m}\n");
            let m = Matrix::reduce(m);
            println!("{m}\n");
            break;
        }
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(21, 6), 42);
        assert_eq!(lcm(7, 3), 2);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 33);
    }
}
