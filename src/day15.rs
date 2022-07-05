use std::{cmp::Reverse, collections::BinaryHeap};

use crate::util::{arr2::Arr2, dir::neighbors4};

pub const INPUT: &str = include_str!("../input/15.txt");

pub fn part1(input: &str) -> i64 {
    let raw: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let grid = Arr2::from_fn(raw.len(), raw[0].len(), |i, j| (raw[i][j] - b'0') as i64);
    solve(&grid)
}

pub fn part2(input: &str) -> i64 {
    let raw: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let (n, m) = (raw.len(), raw[0].len());
    let grid = Arr2::from_fn(n * 5, m * 5, |i, j| {
        ((raw[i % n][j % m] - b'0') as i64 + (i / n) as i64 + (j / m) as i64 + 8) % 9 + 1
    });
    solve(&grid)
}

pub fn solve(g: &Arr2<i64>) -> i64 {
    let (n, m) = g.dims();
    let mut d = Arr2::new(n, m, i64::MAX);
    d[0][0] = 0;
    let mut q: BinaryHeap<_> = BinaryHeap::from([(Reverse(0), (0, 0))]);
    while let Some((Reverse(d0), p0)) = q.pop() {
        if d[p0] > d0 {
            continue;
        }
        if p0 == (n - 1, m - 1) {
            break;
        }
        for p in neighbors4(p0.0, p0.1, n, m) {
            if d[p] > d0 + g[p] {
                d[p] = d0 + g[p];
                q.push((Reverse(d[p]), p));
            }
        }
    }
    d[n - 1][m - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 769);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2963);
    }
}
