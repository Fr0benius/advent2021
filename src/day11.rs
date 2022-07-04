use std::collections::VecDeque;
use std::collections::HashSet;

use crate::util::{arr2::Arr2, dir::neighbors8};

pub const INPUT: &str = include_str!("../input/11.txt");

pub fn part1(input: &str) -> i64 {
    let mut g = {
        let n = input.lines().count();
        let raw: Vec<_> = input
            .bytes()
            .filter(|c| !c.is_ascii_whitespace())
            .map(|b| b - b'0')
            .collect();
        Arr2::from_raw(n, raw.len() / n, raw)
    };
    let mut res = 0;
    let mut q = VecDeque::new();
    let (n, m) = g.dims();
    for _ in 0..100 {
        let mut flashed = HashSet::new();
        for i in 0..n {
            for j in 0..m {
                g[i][j] += 1;
                if g[i][j] > 9 {
                    flashed.insert((i, j));
                    q.push_back((i, j));
                }
            }
        }
        while let Some((i0, j0)) = q.pop_front() {
            for (i, j) in neighbors8(i0, j0, n, m) {
                g[i][j] += 1;
                if g[i][j] > 9 && flashed.insert((i, j)) {
                    q.push_back((i, j));
                }
            }
        }
        for (i, j) in flashed {
            res += 1;
            g[i][j] = 0;
        }
    }
    res
}

pub fn part2(input: &str) -> i64 {
    let mut g = {
        let n = input.lines().count();
        let raw: Vec<_> = input
            .bytes()
            .filter(|c| !c.is_ascii_whitespace())
            .map(|b| b - b'0')
            .collect();
        Arr2::from_raw(n, raw.len() / n, raw)
    };
    let mut q = VecDeque::new();
    let (n, m) = g.dims();
    for t in 1.. {
        let mut flashed = HashSet::new();
        for i in 0..n {
            for j in 0..m {
                g[i][j] += 1;
                if g[i][j] > 9 {
                    flashed.insert((i, j));
                    q.push_back((i, j));
                }
            }
        }
        while let Some((i0, j0)) = q.pop_front() {
            for (i, j) in neighbors8(i0, j0, n, m) {
                g[i][j] += 1;
                if g[i][j] > 9 && flashed.insert((i, j)) {
                    q.push_back((i, j));
                }
            }
        }
        if flashed.len() == n * m {
            return t;
        }
        for (i, j) in flashed {
            g[i][j] = 0;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1591);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 314);
    }
}
