use std::{
    collections::{HashMap, HashSet},
    iter::zip,
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    multi::many0,
    sequence::{preceded, separated_pair},
    IResult,
};

fn parse(s: &str) -> IResult<&str, Vec<(i64, i64)>> {
    let num = || map_res(digit1, str::parse);
    let coord = separated_pair(num(), tag(","), num());
    many0(preceded(multispace0, coord))(s)
}

pub fn solve(s: &str) -> i64 {
    let items = parse(s).unwrap().1;
    items
        .into_iter()
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| (y1 - y2 + 1) * (x1 - x2 + 1))
        .max()
        .unwrap()
}

fn compress(ps: &[i64]) -> (Vec<i64>, Vec<i64>) {
    let mut fwd = HashMap::new();
    let mut bwd = Vec::new();
    for &p in ps.iter().unique().sorted() {
        fwd.insert(p, fwd.len() as i64);
        bwd.push(p);
    }
    let ps = ps.into_iter().map(|p| *fwd.get(p).unwrap()).collect();
    (ps, bwd)
}

fn dir((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> (i64, i64) {
    match (x2 - x1, y2 - y1) {
        (0, 0) => panic!("zero-length"),
        (0, ..0) => (0, -1),
        (0, 1..) => (0, 1),
        (..0, 0) => (-1, 0),
        (1.., 0) => (1, 0),
        (_, _) => panic!("not axis-aligned"),
    }
}

pub fn solve_2(s: &str) -> i64 {
    let items = parse(s).unwrap().1;
    let xs: Vec<_> = items.iter().map(|(x, _)| *x).collect();
    let ys: Vec<_> = items.iter().map(|(_, y)| *y).collect();
    let (xs, x_key) = compress(&xs);
    let (ys, y_key) = compress(&ys);
    let mut points = Vec::new();
    let mut normals = Vec::new();
    let mut winding: i64 = 0;
    for ((x0, y0), (mut x1, mut y1), (x2, y2)) in
        zip(xs.clone(), ys.clone()).circular_tuple_windows()
    {
        let (dx, dy) = dir((x1, y1), (x2, y2));
        let (dx0, dy0) = dir((x0, y0), (x1, y1));
        winding += if dy0 * dx > dx0 * dy { -1 } else { 1 };
        while (x1, y1) != (x2, y2) {
            points.push((x1, y1));
            normals.push((-dy, dx)); // Assuming clockwise
            (x1, y1) = (x1 + dx, y1 + dy);
        }
    }
    let winding = winding / winding.abs();
    let mut stack: Vec<_> = zip(&points, normals)
        .map(|((x, y), (nx, ny))| (x + winding * nx, y + winding * ny))
        .collect();
    let mut tiles: HashSet<_> = points.into_iter().collect();
    while let Some((x, y)) = stack.pop() {
        if tiles.insert((x, y)) {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                stack.push((x + dx, y + dy));
            }
        }
    }
    zip(xs, ys)
        .into_iter()
        .tuple_combinations()
        .filter(|&((x1, y1), (x2, y2))| {
            (x1.min(x2)..=x1.max(x2))
                .cartesian_product(y1.min(y2)..=y1.max(y2))
                .all(|p| tiles.contains(&p))
        })
        .map(|((x1, y1), (x2, y2))| {
            let (x1, y1) = (x_key[x1 as usize], y_key[y1 as usize]);
            let (x2, y2) = (x_key[x2 as usize], y_key[y2 as usize]);
            (1 + (y1 - y2).abs()) * (1 + (x1 - x2).abs())
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 50);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 24);
    }
}
