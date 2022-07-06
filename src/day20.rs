use std::collections::HashSet;

pub const INPUT: &str = include_str!("../input/20.txt");

pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let key = lines.next().unwrap().as_bytes();
    let mut set = HashSet::new();
    for (i, line) in lines.skip(1).enumerate() {
        for (j, c) in line.bytes().enumerate() {
            if c == b'#' {
                set.insert((i as i32, j as i32));
            }
        }
    }
    set = evolve(&set, key, false);
    set = evolve(&set, key, true);
    set.len()
}

pub fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let key = lines.next().unwrap().as_bytes();
    let mut set = HashSet::new();
    for (i, line) in lines.skip(1).enumerate() {
        for (j, c) in line.bytes().enumerate() {
            if c == b'#' {
                set.insert((i as i32, j as i32));
            }
        }
    }
    for _ in 0..25 {
        set = evolve(&set, key, false);
        set = evolve(&set, key, true);
    }
    set.len()
}

fn evolve(set: &HashSet<(i32, i32)>, key: &[u8], flipped: bool) -> HashSet<(i32, i32)> {
    let minx = set.iter().map(|&(x, _)| x).min().unwrap();
    let maxx = set.iter().map(|&(x, _)| x).max().unwrap();
    let miny = set.iter().map(|&(_, y)| y).min().unwrap();
    let maxy = set.iter().map(|&(_, y)| y).max().unwrap();
    let mut res = HashSet::new();
    for x in minx - 1..=maxx + 1 {
        for y in miny - 1..=maxy + 1 {
            let mut u = 0;
            for i in x - 1..=x + 1 {
                for j in y - 1..=y + 1 {
                    u *= 2;
                    if set.contains(&(i, j)) != flipped {
                        u += 1;
                    }
                }
            }
            if (key[u] == b'#') == flipped {
                res.insert((x, y));
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 5464);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 19228);
    }
}
