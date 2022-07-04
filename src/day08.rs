use std::collections::HashMap;

use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/08.txt");

pub fn part1(input: &str) -> i64 {
    let mut res = 0;
    for l in input.lines() {
        let s = l.split(" | ").nth(1).unwrap();
        for w in s.split_ascii_whitespace() {
            if [2, 3, 4, 7].contains(&w.len()) {
                res += 1;
            }
        }
    }
    res
}

pub fn part2(input: &str) -> i64 {
    let mut res = 0;
    for l in input.lines() {
        let (s1, s2): (&str, &str) = l.split(" | ").gather();
        let digits: Vec<_> = s1.split_ascii_whitespace().collect();
        let mut map = HashMap::new();
        for &d in &digits {
            match d.len() {
                2 => map.insert(1, d),
                3 => map.insert(7, d),
                4 => map.insert(4, d),
                7 => map.insert(8, d),
                _ => continue,
            };
        }
        for &d in &digits {
            if d.len() == 5 {
                if isin(map[&1], d) {
                    map.insert(3, d);
                }
            } else if d.len() == 6 {
                if isin(map[&4], d) {
                    map.insert(9, d);
                }
                if !isin(map[&1], d) {
                    map.insert(6, d);
                }
            }
        }
        for &d in &digits {
            if d.len() == 5 {
                if isin(d, map[&6]) {
                    map.insert(5, d);
                } else if d != map[&3] {
                    map.insert(2, d);
                }
            } else if d.len() == 6 && d != map[&6] && d != map[&9] {
                map.insert(0, d);
            }
        }
        assert_eq!(map.len(), 10);
        let mut num = 0;
        for w in s2.split_ascii_whitespace() {
            for (&k, &d) in &map {
                if same(d, w) {
                    num = num * 10 + k;
                    break;
                }
            }
        }
        res += num;
    }
    res
}

fn isin(s: &str, t: &str) -> bool {
    s.chars().all(|c| t.contains(c))
}

fn same(s: &str, t: &str) -> bool {
    isin(s, t) && isin(t, s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 521);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1_016_804);
    }
}
