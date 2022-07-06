use std::collections::{BinaryHeap, HashMap, VecDeque};

use crate::util::{arr2::Arr2, dir::neighbors4};

pub const INPUT: &str = "";
pub const INPUT1: &str = include_str!("../input/23.txt");
pub const INPUT2: &str = include_str!("../input/23-2.txt");

type Pt = (usize, usize);
pub fn solve(input: &str) -> i64 {
    let n = input.lines().count();
    let init: Arr2<u8> = Arr2::from_raw(n, 13, input.bytes().filter(|&b| b != b'\n').collect());
    let end_state = {
        let mut state = init.clone();
        for i in 2..n - 1 {
            for j in 0..4 {
                state[i][2 * j + 3] = b'A' + j as u8;
            }
        }
        state
    };
    let mut q: BinaryHeap<(i64, Arr2<u8>)> = BinaryHeap::from([(0, init.clone())]);
    let mut d: HashMap<Arr2<u8>, i64> = HashMap::from([(init, 0)]);
    while let Some((d0, state)) = q.pop() {
        let d0 = -d0;
        if d0 > d[&state] {
            continue;
        }
        if state == end_state {
            return d0;
        }
        for (pt0, &c) in state.enumerate() {
            if !c.is_ascii_uppercase() {
                continue;
            }
            for (pt, delta) in find_targets(&state, pt0) {
                let mut new_state = state.clone();
                new_state.swap(pt0, pt);
                let entry = d.entry(new_state.clone()).or_insert(i64::MAX);
                if *entry > d0 + delta {
                    *entry = d0 + delta;
                    q.push((-(d0 + delta), new_state));
                }
            }
        }
    }
    0
}

fn find_targets(state: &Arr2<u8>, start: Pt) -> Vec<(Pt, i64)> {
    let delta = [1, 10, 100, 1000][(state[start] - b'A') as usize];
    let (n, m) = state.dims();
    let mut q = VecDeque::from([start]);
    let mut d = HashMap::from([(start, 0)]);
    while let Some(pt0) = q.pop_front() {
        let d0 = d[&pt0];
        for pt in neighbors4(pt0.0, pt0.1, n, m) {
            if state[pt] == b'.' {
                let entry = d.entry(pt).or_insert(i64::MAX);
                if *entry > d0 + delta {
                    *entry = d0 + delta;
                    q.push_back(pt);
                }
            }
        }
    }
    if start.0 == 1 {
        for i in (2..n-1).rev() {
            let target = (i, 3 + 2 * (state[start] - b'A') as usize);
            if let Some(&d0) = d.get(&target) {
                return vec![(target, d0)];
            }
            if state[target] != state[start] {
                return vec![];
            }
            
        }
        vec![]
    } else {
        d.into_iter()
            .filter(|&(pt, _)| pt.0 == 1 && ![3, 5, 7, 9].contains(&pt.1))
            .collect()
    }
}

pub fn part1(_dummy: &str) -> i64 {
    solve(INPUT1)
}

pub fn part2(_dummy: &str) -> i64 {
    solve(INPUT2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(""), 13520);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 48708);
    }
}
