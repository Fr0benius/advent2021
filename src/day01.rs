use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/01.txt");

pub fn part1(input: &str) -> usize {
    let v: Vec<i64> = input.lines().gather();
    v.windows(2).filter(|&w| w[0] < w[1]).count()
}

pub fn part2(input: &str) -> usize {
    let a: Vec<i64> = input.lines().gather();
    let v: Vec<i64> = a.windows(3).map(|w| w.iter().copied().sum()).collect();
    v.windows(2).filter(|&w| w[0] < w[1]).count()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1553);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1597);
    }
}
