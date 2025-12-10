use std::collections::{HashMap, HashSet};

fn parse(s: &str) -> ((i32, i32), HashSet<(i32, i32)>) {
    let mut splitters = HashSet::new();
    let mut start = None;
    for (y, line) in s.trim().lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            match c {
                '^' => {
                    splitters.insert((y as i32, x as i32));
                }
                'S' => start = Some((y as i32, x as i32)),
                '.' => continue,
                _ => panic!(),
            }
        }
    }
    (start.unwrap(), splitters)
}

pub fn solve(s: &str) -> usize {
    let (start, splitters) = parse(s);
    let &max_y = splitters.iter().map(|(y, _)| y).max().unwrap();
    let mut beams: HashSet<_> = [start.1].into_iter().collect();
    let mut splits = 0;
    for y in 1..=max_y {
        let mut new_beams = HashSet::new();
        for x in beams {
            if splitters.contains(&(y, x)) {
                splits += 1;
                new_beams.extend([x - 1, x + 1]);
            } else {
                new_beams.insert(x);
            }
        }
        beams = new_beams;
    }
    splits
}

pub fn solve_2(s: &str) -> usize {
    let (start, splitters) = parse(s);
    let &max_y = splitters.iter().map(|(y, _)| y).max().unwrap();
    let &max_x = splitters.iter().map(|(_, x)| x).max().unwrap();
    let mut counts: HashMap<(i32, i32), usize> = [(start, 1)].into_iter().collect();
    for y in 1..=max_y {
        for x in -1..=max_x + 1 {
            if splitters.contains(&(y, x)) {
                continue;
            }
            let n: usize = [x - 1, x + 1]
                .into_iter()
                .filter(|&x| splitters.contains(&(y, x)))
                .chain([x])
                .filter_map(|x| counts.get(&(y - 1, x)))
                .sum();
            counts.insert((y, x), n);
        }
    }
    (-1..=max_x + 1)
        .filter_map(|x| counts.get(&(max_y, x)))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &'static str = "
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............";

    #[test]
    fn test_sample() {
        assert_eq!(solve(SAMPLE), 21);
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(solve_2(SAMPLE), 40);
    }
}
