use std::collections::HashMap;
use std::collections::HashSet;

use crate::util::parsing::Gather;
pub const INPUT: &str = include_str!("../input/12.txt");

pub fn part1(input: &str) -> i64 {
    solve(input, true)
}

pub fn part2(input: &str) -> i64 {
    solve(input, false)
}

pub fn solve(input: &str, visit_only_once: bool) -> i64 {
    let mut g = HashMap::new();
    for l in input.lines() {
        let (u, v): (&str, &str) = l.split('-').gather();
        g.entry(u).or_insert(vec![]).push(v);
        g.entry(v).or_insert(vec![]).push(u);
    }
    dfs("start", &g, &mut HashSet::new(), visit_only_once)
}

fn dfs<'a>(
    x: &'a str,
    g: &HashMap<&'a str, Vec<&'a str>>,
    seen: &mut HashSet<&'a str>,
    mut small_cave_visited: bool,
) -> i64 {
    let visited = seen.contains(&x);
    if x == "start" && visited {
        return 0;
    }
    if x == "end" {
        return 1;
    }
    let mut keep_cave = false;
    if x.bytes().all(|c| c.is_ascii_lowercase()) && visited {
        if small_cave_visited {
            return 0;
        }
        small_cave_visited = true;
        keep_cave = true;
    }
    seen.insert(x);
    let mut res = 0;
    for &y in &g[&x] {
        res += dfs(y, g, seen, small_cave_visited);
    }
    if !keep_cave {
        seen.remove(&x);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 4707);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 130_493);
    }
}
