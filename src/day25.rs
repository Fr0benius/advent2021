use crate::util::arr2::Arr2;

pub const INPUT: &str = include_str!("../input/25.txt");

pub fn part1(input: &str) -> i64 {
    let mut g = {
        let n = input.lines().count();
        let v: Vec<u8> = input.bytes().filter(|&b| b != b'\n').collect();
        Arr2::from_raw(n, v.len() / n, v)
    };
    let (n, m) = g.dims();
    for cnt in 1.. {
        let mut moved = false;
        let coords: Vec<_> = g
            .enumerate()
            .filter(|&((i, j), &c)| c == b'>' && g[i][(j + 1) % m] == b'.')
            .map(|(p, _)| p)
            .collect();
        for (i, j) in coords {
            moved = true;
            g.swap((i, j), (i, (j + 1) % m));
        }
        let coords: Vec<_> = g
            .enumerate()
            .filter(|&((i, j), &c)| c == b'v' && g[(i + 1) % n][j] == b'.')
            .map(|(p, _)| p)
            .collect();
        for (i, j) in coords {
            moved = true;
            g.swap((i, j), ((i + 1) % n, j));
        }
        if !moved {
            return cnt;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 39_924_989_499_969);
    }
}
