use std::collections::HashMap;

use crate::util::parsing::re_parser;

pub const INPUT: &str = include_str!("../input/05.txt");

pub fn part1(input: &str) -> i64 {
    let parse = re_parser(r"(.*),(.*) -> (.*),(.*)");
    let mut map = HashMap::new();
    for line in input.lines() {
        let (mut x1, mut y1, x2, y2) :(i64, i64, i64, i64) = parse(line);
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        if dx != 0 && dy != 0 {
            continue;
        }
        loop {
            *map.entry((x1, y1)).or_insert(0) += 1;
            if (x1, y1) == (x2, y2) {
                break;
            }
            x1 += dx;
            y1 += dy;
        }
    }
    map.into_values().filter(|&v| v >= 2).count() as i64
}

pub fn part2(input: &str) -> i64 {
    let parse = re_parser(r"(.*),(.*) -> (.*),(.*)");
    let mut map = HashMap::new();
    for line in input.lines() {
        let (mut x1, mut y1, x2, y2) :(i64, i64, i64, i64) = parse(line);
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        loop {
            *map.entry((x1, y1)).or_insert(0) += 1;
            if (x1, y1) == (x2, y2) {
                break;
            }
            x1 += dx;
            y1 += dy;
        }
    }
    map.into_values().filter(|&v| v >= 2).count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6267);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 20196);
    }
}
