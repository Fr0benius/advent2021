use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/21.txt");

pub fn part1(input: &str) -> i64 {
    let (p1, p2): (i64, i64) = input
        .lines()
        .map(|l| l.split(": ").nth(1).unwrap())
        .gather();
    let mut pos = [p1 - 1, p2 - 1];
    let mut score = [0, 0];
    let mut d = 0;
    let mut times = 0;
    let mut roll = || {
        times += 1;
        d = d % 100 + 1;
        d
    };
    let mut roll3 = || roll() + roll() + roll();
    loop {
        for i in 0..2 {
            pos[i] = (pos[i] + roll3()) % 10;
            score[i] += pos[i] + 1;
            if score[i] >= 1000 {
                return times * score[1 - i];
            }
        }
    }
}

pub fn part2(input: &str) -> i64 {
    let (pos1, pos2): (usize, usize) = input
        .lines()
        .map(|l| l.split(": ").nth(1).unwrap())
        .gather();
    let mut dp = [[[[[0i64; 2]; 10]; 10]; 21]; 21];
    dp[0][0][pos1 - 1][pos2 - 1][0] = 1;
    let mut tot1 = 0;
    let mut tot2 = 0;
    for s1 in 0..21 {
        for s2 in 0..21 {
            for p1 in 0..10 {
                for p2 in 0..10 {
                    for d1 in 1..=3 {
                        for d2 in 1..=3 {
                            for d3 in 1..=3 {
                                let pos = (p1 + d1 + d2 + d3) % 10;
                                let sc = s1 + pos + 1;
                                if sc >= 21 {
                                    tot1 += dp[s1][s2][p1][p2][0];
                                } else {
                                    dp[sc][s2][pos][p2][1] += dp[s1][s2][p1][p2][0];
                                }
                                let pos = (p2 + d1 + d2 + d3) % 10;
                                let sc = s2 + pos + 1;
                                if sc >= 21 {
                                    tot2 += dp[s1][s2][p1][p2][1];
                                } else {
                                    dp[s1][sc][p1][pos][0] += dp[s1][s2][p1][p2][1];
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    tot1.max(tot2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 752_247);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 221_109_915_584_112);
    }
}
