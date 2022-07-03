use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/02.txt");

pub fn part1(input: &str) -> i64 {
    let v: Vec<(&str, i64)> = input.split_whitespace().gather();
    let (h, d) = v.into_iter().fold((0, 0), |(h, d), (op, x)| match op {
        "forward" => (h + x, d),
        "up" => (h, d - x),
        "down" => (h, d + x),
        _ => unreachable!(),
    });
    h * d
}

pub fn part2(input: &str) -> i64 {
    let v: Vec<(&str, i64)> = input.split_whitespace().gather();
    let (h, d, _) = v
        .into_iter()
        .fold((0, 0, 0), |(h, d, a), (op, x)| match op {
            "forward" => (h + x, d + a * x, a),
            "up" => (h, d, a - x),
            "down" => (h, d, a + x),
            _ => unreachable!(),
        });
    h * d
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2_039_256);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1_856_459_736);
    }
}
