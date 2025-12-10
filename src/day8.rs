use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::many0,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Clone, Copy)]
enum Node {
    Root { size: usize },
    Child { parent: usize },
}

struct UnionFind {
    items: HashMap<(i64, i64, i64), usize>,
    nodes: Vec<Node>,
}

impl UnionFind {
    fn new(items: impl IntoIterator<Item = (i64, i64, i64)>) -> Self {
        let items: HashMap<_, _> = items.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
        Self {
            nodes: vec![Node::Root { size: 1 }; items.len()],
            items,
        }
    }

    fn find(&self, mut i: usize) -> (usize, usize) {
        loop {
            match self.nodes[i] {
                Node::Root { size } => return (i, size),
                Node::Child { parent } => i = parent,
            }
        }
    }

    fn merge(&mut self, a: (i64, i64, i64), b: (i64, i64, i64)) -> Option<bool> {
        let &a = self.items.get(&a)?;
        let &b = self.items.get(&b)?;
        let (a, a_size) = self.find(a);
        let (b, b_size) = self.find(b);
        if a == b {
            return Some(false);
        }
        let (parent, child) = if a_size > b_size { (a, b) } else { (b, a) };
        self.nodes[child] = Node::Child { parent };
        self.nodes[parent] = Node::Root {
            size: a_size + b_size,
        };
        Some(true)
    }

    fn iter_unions(&self) -> impl Iterator<Item = usize> + '_ {
        self.nodes.iter().filter_map(|n| match n {
            Node::Root { size } => Some(*size),
            Node::Child { .. } => None,
        })
    }
}

fn parse(s: &str) -> IResult<&str, Vec<(i64, i64, i64)>> {
    let num = || map_res(digit1, str::parse);
    let coord = tuple((num(), preceded(tag(","), num()), preceded(tag(","), num())));
    many0(preceded(multispace0, coord))(s)
}

fn run(s: &str, n_connections: usize) -> usize {
    let items = parse(s).unwrap().1;
    let mut uf = UnionFind::new(items.clone());
    let mut pairs: Vec<(_, _)> = items.into_iter().tuple_combinations().collect();
    pairs.sort_by_key(|&((x1, y1, z1), (x2, y2, z2))| {
        (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)
    });
    for &(a, b) in &pairs[0..n_connections] {
        uf.merge(a, b);
    }
    uf.iter_unions().sorted().rev().take(3).product()
}

pub fn solve(s: &str) -> usize {
    run(s, 1000)
}

pub fn solve_2(s: &str) -> i64 {
    let items = parse(s).unwrap().1;
    let mut uf = UnionFind::new(items.clone());
    let mut pairs: Vec<(_, _)> = items.into_iter().tuple_combinations().collect();
    pairs.sort_by_key(|&((x1, y1, z1), (x2, y2, z2))| {
        (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)
    });
    let mut last = None;
    for (a, b) in pairs {
        if uf.merge(a, b) == Some(true) {
            last = Some(a.0 * b.0);
        }
    }
    last.unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689";

    #[test]
    fn test_sample() {
        assert_eq!(run(SAMPLE, 10), 40);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 25272);
    }
}
