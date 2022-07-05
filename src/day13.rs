use std::collections::HashSet;

use crate::util::misc::render_2d;
use crate::util::parsing::re_parser;
use crate::util::parsing::Gather;
pub const INPUT: &str = include_str!("../input/13.txt");

pub fn part1(input: &str) -> usize {
    let (pts, folds) = parse(input);
    fold(pts, folds[0]).len()
}

fn fold(pts: HashSet<(i64, i64)>, (coord, val): (&str, i64)) -> HashSet<(i64, i64)> {
    let mut res = HashSet::new();
    for (mut x, mut y) in pts {
        if coord == "x" && x > val {
            x = 2 * val - x;
        } else if coord == "y" && y > val {
            y = 2 * val - y;
        }
        res.insert((x, y));
    }
    res
}

pub fn part2(input: &str) -> String {
    let (mut pts, folds) = parse(input);
    for f in folds {
        pts = fold(pts, f);
    }
    render_2d(pts.iter().map(|&(x, y)| (y, x)))
}

fn parse(input: &str) -> (HashSet<(i64, i64)>, Vec<(&str, i64)>) {
    let (blob1, blob2): (&str, &str) = input.split("\n\n").gather();
    let pts: HashSet<(i64, i64)> = blob1.lines().map(|l| l.split(',').gather()).collect();
    let parse = re_parser(r"fold along (.*)=(.*)");
    let folds = blob2.lines().map(parse).collect();
    (pts, folds)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 850);
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            part2(INPUT), r"
 00  0  0  00   00  000   00   00  0  0
0  0 0  0 0  0 0  0 0  0 0  0 0  0 0  0
0  0 0000 0    0    0  0 0    0  0 0  0
0000 0  0 0 00 0    000  0 00 0000 0  0
0  0 0  0 0  0 0  0 0    0  0 0  0 0  0
0  0 0  0  000  00  0     000 0  0  00 
"[1..]
            .to_owned()
        );
    }
}
