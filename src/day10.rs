use std::collections::HashMap;

pub const INPUT: &str = include_str!("../input/10.txt");

pub fn part1(input: &str) -> i64 {
    let brackets = b"()[]{}<>";
    let mut map = HashMap::new();
    for i in 0..4 {
        map.insert(brackets[2 * i], brackets[2 * i + 1]);
        map.insert(brackets[2 * i + 1], brackets[2 * i]);
    }
    let scores = HashMap::from([(b')', 3), (b']', 57), (b'}', 1197), (b'>', 25137)]);
    input
        .lines()
        .map(|line| {
            let mut st = vec![];
            for c in line.bytes() {
                if let Some(&score) = scores.get(&c) {
                    if st.last() == map.get(&c) {
                        st.pop();
                    } else {
                        return score;
                    }
                } else {
                    st.push(c);
                }
            }
            0
        })
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let brackets = b"()[]{}<>";
    let mut map = HashMap::new();
    for i in 0..4 {
        map.insert(brackets[2 * i], brackets[2 * i + 1]);
        map.insert(brackets[2 * i + 1], brackets[2 * i]);
    }
    let scores = HashMap::from([(b')', 1), (b']', 2), (b'}', 3), (b'>', 4)]);
    let mut res = input
        .lines()
        .filter_map(|line| {
            let mut st = vec![];
            for c in line.bytes() {
                if scores.get(&c).is_some() {
                    if st.last() == map.get(&c) {
                        st.pop();
                    } else {
                        return None;
                    }
                } else {
                    st.push(c);
                }
            }
            let mut score = 0;
            while let Some(open) = st.pop() {
                score = score * 5 + scores[&map[&open]];
            }
            Some(score)
        })
        .collect::<Vec<_>>();
    res.sort_unstable();
    res[res.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 319_329);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 3_515_583_998);
    }
}
