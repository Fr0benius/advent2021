pub const INPUT: &str = include_str!("../input/18.txt");

pub fn part1(input: &str) -> i32 {
    let snails: Vec<_> = input.lines().map(SnailNumber::new).collect();
    let mut sum = snails[0].clone();
    for snail in snails.iter().skip(1) {
        sum = SnailNumber::add(&sum, snail);
    }
    sum.magnitude()
}

pub fn part2(input: &str) -> i32 {
    let snails: Vec<_> = input.lines().map(SnailNumber::new).collect();
    let mut res = 0;
    for i in 0..snails.len() {
        for j in 0..snails.len() {
            if i != j {
                res = res.max(SnailNumber::add(&snails[i], &snails[j]).magnitude());
            }
        }
    }
    res
}

use Node::*;
#[derive(Clone, Copy, Debug)]
enum Node {
    Leaf(i32),
    Pair(usize, usize),
}

impl Node {
    fn val(&self) -> i32 {
        match *self {
            Leaf(x) => x,
            Pair(..) => 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SnailNumber {
    data: Vec<Node>,
    root: usize,
    next: Vec<Option<usize>>,
    prev: Vec<Option<usize>>,
}

impl SnailNumber {
    pub fn new(s: &str) -> Self {
        let mut data = vec![];
        let root = Self::parse(&mut s.bytes(), &mut data);
        let mut next = vec![None; data.len()];
        let mut prev = vec![None; data.len()];
        Self::find_links(root, &data, &mut next, &mut prev);
        Self {
            data,
            root,
            next,
            prev,
        }
    }
    pub fn magnitude(&self) -> i32 {
        self.eval(self.root)
    }
    pub fn add(a: &Self, b: &Self) -> Self {
        let mut res = a.clone();
        let (n, m) = (a.data.len(), b.data.len());
        res.data.extend(b.data.iter().map(|&node| match node {
            Leaf(_) => node,
            Pair(l, r) => Pair(n + l, n + r),
        }));
        res.data.push(Pair(a.root, b.root + n));
        res.root = n + m;
        res.prev = vec![None; res.data.len()];
        res.next = vec![None; res.data.len()];
        Self::find_links(res.root, &res.data, &mut res.next, &mut res.prev);
        res.normalize();
        res
    }
    fn parse<I: Iterator<Item = u8>>(iter: &mut I, data: &mut Vec<Node>) -> usize {
        let c = iter.next().unwrap();
        if c == b'[' {
            let a = Self::parse(iter, data);
            iter.next();
            let b = Self::parse(iter, data);
            iter.next();
            data.push(Pair(a, b));
        } else {
            data.push(Leaf((c - b'0') as i32));
        }
        data.len() - 1
    }
    fn find_links(
        k: usize,
        data: &Vec<Node>,
        next: &mut Vec<Option<usize>>,
        prev: &mut Vec<Option<usize>>,
    ) -> (usize, usize) {
        match data[k] {
            Leaf(_) => (k, k),
            Pair(a, b) => {
                let (l, m1) = Self::find_links(a, data, next, prev);
                let (m2, r) = Self::find_links(b, data, next, prev);
                next[m1] = Some(m2);
                prev[m2] = Some(m1);
                (l, r)
            }
        }
    }
    fn normalize(&mut self) {
        loop {
            if self.explode() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }
    fn explode(&mut self) -> bool {
        if let Some(k) = self.find_deep(self.root, 0, 4) {
            if let Pair(l, r) = self.data[k] {
                let x = self.data[l].val();
                let y = self.data[r].val();
                self.data[k] = Leaf(0);
                if let Some(p) = self.prev[l] {
                    self.next[p] = Some(k);
                    self.prev[k] = Some(p);
                    self.data[p] = Leaf(self.data[p].val() + x);
                }
                if let Some(n) = self.next[r] {
                    self.next[k] = Some(n);
                    self.prev[n] = Some(k);
                    self.data[n] = Leaf(self.data[n].val() + y);
                }
                return true;
            }
            unreachable!();
        }
        false
    }
    fn find_deep(&self, k: usize, cur_dep: i32, dep_lim: i32) -> Option<usize> {
        if let Pair(l, r) = self.data[k] {
            if cur_dep >= dep_lim {
                return Some(k);
            }
            if let Some(x) = self.find_deep(l, cur_dep + 1, dep_lim) {
                return Some(x);
            }
            if let Some(x) = self.find_deep(r, cur_dep + 1, dep_lim) {
                return Some(x);
            }
        }
        None
    }

    fn split(&mut self) -> bool {
        if let Some(k) = self.find_big(self.root) {
            let v = self.data[k].val();
            let n = self.data.len();
            self.data.push(Leaf(v / 2));
            self.data.push(Leaf((v + 1) / 2));
            self.data[k] = Pair(n, n + 1);
            // self.prev[k] = None;
            // self.next[k] = None;
            self.prev.extend([None, None].into_iter());
            self.next.extend([None, None].into_iter());
            if let Some(p) = self.prev[k] {
                self.prev[n] = Some(p);
                self.next[p] = Some(n);
            }
            if let Some(z) = self.next[k] {
                self.prev[z] = Some(n + 1);
                self.next[n + 1] = Some(z);
            }
            self.prev[n + 1] = Some(n);
            self.next[n] = Some(n + 1);
            true
        } else {
            false
        }
    }
    fn find_big(&self, k: usize) -> Option<usize> {
        match self.data[k] {
            Leaf(x) => return if x >= 10 { Some(k) } else { None },
            Pair(l, r) => {
                if let Some(x) = self.find_big(l) {
                    return Some(x);
                }
                if let Some(x) = self.find_big(r) {
                    return Some(x);
                }
            }
        }
        None
    }

    fn eval(&self, k: usize) -> i32 {
        match self.data[k] {
            Leaf(x) => x,
            Pair(l, r) => self.eval(l) * 3 + self.eval(r) * 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3763);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4664);
    }
}
