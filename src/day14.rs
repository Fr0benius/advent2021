use std::collections::HashMap;

use crate::util::{counter::Counter, parsing::Gather};

pub const INPUT: &str = include_str!("../input/14.txt");

pub fn part1(input: &str) -> i64 {
    solve(input, 10)
}

pub fn part2(input: &str) -> i64 {
    solve(input, 40)
}

type Pr = (u8, u8);
pub fn solve(input: &str, times: usize) -> i64 {
    let mut lines = input.lines();
    let orig: &str = lines.gather();
    let rules: HashMap<Pr, [Pr; 2]> = lines
        .skip(1)
        .map(|l| {
            let (pair, c): (&str, char) = l.split(" -> ").gather();
            let pair = pair.as_bytes();
            let (a, b, c) = (pair[0], pair[1], c as u8);
            ((a, b), [(a, c), (c, b)])
        })
        .collect();
    let mut cnt = orig.as_bytes().iter().copied().counter();
    let mut pcnt = orig.as_bytes().windows(2).map(|w| (w[0], w[1])).counter();
    for _ in 0..times {
        let mut tmp = HashMap::new();
        for (pr, k) in pcnt {
            for nxt in rules[&pr] {
                *tmp.entry(nxt).or_insert(0) += k;
            }
            *cnt.entry(rules[&pr][0].1).or_insert(0) += k;
        }
        pcnt = tmp;
    }
    cnt.values().copied().max().unwrap() as i64 - cnt.values().copied().min().unwrap() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2851);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 10_002_813_279_337);
    }
}
