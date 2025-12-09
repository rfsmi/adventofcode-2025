use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn parse(s: &str) -> HashSet<(i32, i32)> {
    let mut paper = HashSet::new();
    for (y, row) in s.trim().lines().enumerate() {
        for (x, c) in row.trim().chars().enumerate() {
            if c == '@' {
                paper.insert((y as i32, x as i32));
            }
        }
    }
    paper
}

fn remove_paper(mut paper: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut counts: HashMap<(i32, i32), usize> = paper.iter().map(|&p| (p, 0)).collect();
    for &(y, x) in &paper {
        for (dy, dx) in (-1..2).cartesian_product(-1..2) {
            if (dy, dx) != (0, 0) {
                let p = (y + dy, x + dx);
                counts.entry(p).and_modify(|c| *c += 1);
            }
        }
    }
    for (p, c) in counts {
        if c < 4 {
            paper.remove(&p);
        }
    }
    paper
}

pub fn solve(s: &str) -> usize {
    let paper = parse(s);
    paper.len() - remove_paper(paper).len()
}

pub fn solve_2(s: &str) -> usize {
    let mut paper = parse(s);
    let original_len = paper.len();
    loop {
        let last_len = paper.len();
        paper = remove_paper(paper);
        if paper.len() == last_len {
            break;
        }
    }
    original_len - paper.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 13);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 43);
    }
}
