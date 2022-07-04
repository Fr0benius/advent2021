use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/06.txt");

pub fn solve(input: &str, times: usize) -> i64 {
    let v: Vec<usize> = input.trim().split(',').gather();
    let mut cnt = [0i64; 9];
    for k in v {
        cnt[k] += 1;
    }
    for _ in 0..times {
        let tmp = cnt;
        cnt[0..8].copy_from_slice(&tmp[1..9]);
        cnt[8] = tmp[0];
        cnt[6] += tmp[0];
    }
    cnt.into_iter().sum()
}

pub fn part1(input: &str) -> i64 {
    solve(input, 80)
}

pub fn part2(input: &str) -> i64 {
    solve(input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 356_190);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1_617_359_101_538);
    }
}
