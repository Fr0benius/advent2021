use std::collections::{VecDeque, HashSet};

use crate::util::dir::neighbors4;

pub const INPUT: &str = include_str!("../input/09.txt");

pub fn part1(input: &str) -> i64 {
    let g: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let (n, m) = (g.len(), g[0].len());
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            if neighbors4(i, j, n, m).all(|(a, b)| g[a][b] > g[i][j]) {
                res += (g[i][j] - b'0') as i64 + 1;
            }
        }
    }
    res
}

pub fn part2(input: &str) -> i64 {
    let g: Vec<Vec<u8>> = input.lines().map(|l| l.bytes().collect()).collect();
    let (n, m) = (g.len(), g[0].len());
    let mut seen = HashSet::new();
    let mut counts = vec![];
    for i in 0..n {
        for j in 0..m {
            if g[i][j] == b'9' || seen.contains(&(i, j)) {
                continue;
            }
            let mut q = VecDeque::from([(i, j)]);
            seen.insert((i, j));
            let mut cnt = 0;
            while let Some((r, c)) = q.pop_front() {
                cnt += 1;
                for (r1, c1) in neighbors4(r, c, n, m) {
                    if g[r1][c1] == b'9' || seen.contains(&(r1, c1)) {
                        continue;
                    }
                    seen.insert((r1, c1));
                    q.push_back((r1, c1));
                }
            }
            counts.push(cnt);
        }
    }
    counts.sort_unstable();
    counts.reverse();
    counts[0] * counts[1] * counts[2]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 577);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1_069_200);
    }
}
