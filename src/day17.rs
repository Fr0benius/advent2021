use crate::util::parsing::re_parser;

pub const INPUT: &str = include_str!("../input/17.txt");

pub fn part1(input: &str) -> i64 {
    let (minx, maxx, miny, maxy): (i64, i64, i64, i64) =
        re_parser(r"target area: x=(.*)\.\.(.*), y=(.*)\.\.(.*)")(input.trim());
    let y = (0..=miny.abs())
        .filter(|&y| (1..=maxx).any(|x| sim(x, y, (minx, maxx), (miny, maxy))))
        .max()
        .unwrap();
    y * (y + 1) / 2
}

pub fn part2(input: &str) -> usize {
    let (minx, maxx, miny, maxy): (i64, i64, i64, i64) =
        re_parser(r"target area: x=(.*)\.\.(.*), y=(.*)\.\.(.*)")(input.trim());
    (miny..=miny.abs())
        .map(|y| {
            (1..=maxx)
                .filter(|&x| sim(x, y, (minx, maxx), (miny, maxy)))
                .count()
        })
        .sum()
}

fn sim(mut vx: i64, mut vy: i64, (minx, maxx): (i64, i64), (miny, maxy): (i64, i64)) -> bool {
    let mut x = 0;
    let mut y = 0;
    loop {
        if (minx..=maxx).contains(&x) && (miny..=maxy).contains(&y) {
            return true;
        }
        if x > maxx || (x < minx && vx == 0) {
            return false;
        }
        if y < miny {
            return false;
        }
        x += vx;
        y += vy;
        vx = 0i64.max(vx - 1);
        vy -= 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 12090);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 5059);
    }
}
