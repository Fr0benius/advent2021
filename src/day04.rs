use std::collections::HashMap;

use crate::util::{arr2::Arr2, parsing::Gather};

pub const INPUT: &str = include_str!("../input/04.txt");

pub fn part1(input: &str) -> i64 {
    let (_, val, score) = scores(input).into_iter().min().unwrap();
    val * score
}

pub fn part2(input: &str) -> i64 {
    let (_, val, score) = scores(input).into_iter().max().unwrap();
    val * score
}

pub fn scores(input: &str) -> Vec<(usize, i64, i64)> {
    let (raw_nums, raw_cards): (&str, Vec<&str>) = input.split("\n\n").gather();
    let nums: Vec<i64> = raw_nums.split(',').gather();
    let ord: HashMap<_, _> = nums.into_iter().enumerate().map(|(i, k)| (k, i)).collect();
    let cards: Vec<Arr2<i64>> = raw_cards
        .iter()
        .map(|&s| Arr2::from_raw(5, 5, s.split_ascii_whitespace().gather()))
        .collect();
    cards.iter().map(|card| score(card, &ord)).collect()
}

fn score(card: &Arr2<i64>, ord: &HashMap<i64, usize>) -> (usize, i64, i64) {
    let (n, m) = card.dims();
    let (idx, val) = (0..n)
        .map(|i| card.row(i).map(|&x| (ord[&x], x)).max().unwrap())
        .chain((0..m).map(|j| card.col(j).map(|&x| (ord[&x], x)).max().unwrap()))
        .min()
        .unwrap();
    let s = card.iter().filter(|&x| ord[x] > idx).sum();
    (idx, val, s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 21607);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 19012);
    }
}
