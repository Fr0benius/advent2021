use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/07.txt");

pub fn part1(input: &str) -> i64 {
    let mut crabs: Vec<i64> = input.trim().split(',').gather();
    let n = crabs.len();
    crabs.sort_unstable();
    crabs.iter().map(|&x| (x - crabs[n / 2]).abs()).sum()
}

pub fn part2(input: &str) -> i64 {
    let mut crabs: Vec<i64> = input.trim().split(',').gather();
    let n = crabs.len();
    crabs.sort_unstable();
    (crabs[0]..=crabs[n - 1]).map(|k| {
        crabs.iter().map(|&x| {
            let d = (x - k).abs();
            d * (d + 1) / 2
        }).sum()
    }).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 343_441);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 98_925_151);
    }
}
