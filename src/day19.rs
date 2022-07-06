use std::collections::HashSet;

use crate::util::parsing::Gather;

pub const INPUT: &str = include_str!("../input/19.txt");

pub fn part1(input: &str) -> usize {
    let mut sets: Vec<HashSet<_>> = input
        .split("\n\n")
        .map(|blob| {
            blob.lines()
                .skip(1)
                .map(|line| {
                    let (x, y, z): (i32, i32, i32) = line.split(',').gather();
                    [x, y, z]
                })
                .collect()
        })
        .collect();
    let mut tot = sets[0].clone();
    for i in 1..sets.len() {
        'a: for j in 0..i {
            for k in i..sets.len() {
                for s in rot_set(&sets[k]) {
                    if let Some((fit, _)) = find_match(&sets[j], &s) {
                        println!("Round {}: {} matches {}", i, j, k);
                        sets.swap(i, k);
                        sets[i] = fit;
                        tot.extend(sets[i].iter().copied());
                        break 'a;
                    }
                }
            }
        }
    }
    tot.len()
}

pub fn part2(input: &str) -> i32 {
    let mut sets: Vec<HashSet<_>> = input
        .split("\n\n")
        .map(|blob| {
            blob.lines()
                .skip(1)
                .map(|line| {
                    let (x, y, z): (i32, i32, i32) = line.split(',').gather();
                    [x, y, z]
                })
                .collect()
        })
        .collect();
    let mut sc_pos = vec![[0, 0, 0]];
    for i in 1..sets.len() {
        'a: for j in 0..i {
            for k in i..sets.len() {
                for s in rot_set(&sets[k]) {
                    if let Some((fit, delta)) = find_match(&sets[j], &s) {
                        println!("Round {}: {} matches {}", i, j, k);
                        sets.swap(i, k);
                        sets[i] = fit;
                        sc_pos.push(delta);
                        break 'a;
                    }
                }
            }
        }
    }
    let mut res = 0;
    for &[a, b, c] in &sc_pos {
        for &[x, y, z] in &sc_pos {
            res = res.max((a-x).abs() + (b-y).abs() + (c-z).abs());
        }
    }
    res
}

type Pt = [i32; 3];
const EVEN: [(usize, usize, usize); 3] = [(0, 1, 2), (1, 2, 0), (2, 0, 1)];
const ODD: [(usize, usize, usize); 3] = [(1, 0, 2), (2, 1, 0), (0, 2, 1)];

fn rots(p: Pt) -> impl Iterator<Item = Pt> {
    let a = EVEN.into_iter().flat_map(move |(i, j, k)| {
        let (x, y, z) = (p[i], p[j], p[k]);
        [[x, y, z], [-x, -y, z], [-x, y, -z], [x, -y, -z]]
    });
    let b = ODD.into_iter().flat_map(move |(i, j, k)| {
        let (x, y, z) = (p[i], p[j], p[k]);
        [[-x, -y, -z], [-x, y, z], [x, -y, z], [x, y, -z]]
    });
    a.chain(b)
}

fn rot_set(s: &HashSet<Pt>) -> impl Iterator<Item = HashSet<Pt>> {
    let mut res = vec![HashSet::new(); 24];
    for &p0 in s {
        for (i, p) in rots(p0).enumerate() {
            res[i].insert(p);
        }
    }
    res.into_iter()
}

fn find_match(a: &HashSet<Pt>, b: &HashSet<Pt>) -> Option<(HashSet<Pt>, Pt)> {
    for &p0 in a {
        for &p1 in b {
            let mut res = HashSet::new();
            let mut cnt = 0;
            let mut delta = [0; 3];
            for i in 0..3 {
                delta[i] = p0[i] - p1[i];
            }
            for &(mut p) in b {

                for i in 0..3 {
                    p[i] += delta[i];
                }
                res.insert(p);
                if a.contains(&p) {
                    cnt += 1;
                }
            }
            if cnt >= 12 {
                return Some((res, delta));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore] // Slow
    fn test_part1() {
        assert_eq!(part1(INPUT), 408);
    }
    #[test]
    #[ignore] // Slow
    fn test_part2() {
        assert_eq!(part2(INPUT), 13348);
    }
}
