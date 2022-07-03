pub const INPUT: &str = include_str!("../input/03.txt");

pub fn part1(input: &str) -> i64 {
    let v: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.bytes().map(|x| x - b'0').collect())
        .collect();
    let n = v.len();
    let m = v[0].len();
    let (mut g, mut e) = (0, 0);
    for j in 0..m {
        g *= 2;
        e *= 2;
        let ones = v.iter().filter(|&x| x[j] == 1).count();
        if ones * 2 > n {
            g += 1;
        } else {
            e += 1;
        }
    }
    g * e
}

pub fn part2(input: &str) -> i64 {
    let init: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.bytes().map(|x| x - b'0').collect())
        .collect();
    let m = init[0].len();
    let mut v = init.clone();
    for j in 0..m {
        if v.len() == 1 {
            break;
        }
        let ones: Vec<_> = v.iter().cloned().filter(|x| x[j] == 1).collect();
        let zeroes: Vec<_> = v.iter().cloned().filter(|x| x[j] == 0).collect();
        v = if ones.len() >= zeroes.len() {
            ones
        } else {
            zeroes
        };
    }
    let oxy = v[0].iter().fold(0, |x, &b| x * 2 + b as i64);
    let mut v = init;
    for j in 0..m {
        if v.len() == 1 {
            break;
        }
        let ones: Vec<_> = v.iter().cloned().filter(|x| x[j] == 1).collect();
        let zeroes: Vec<_> = v.iter().cloned().filter(|x| x[j] == 0).collect();
        v = if ones.len() < zeroes.len() {
            ones
        } else {
            zeroes
        };
    }
    let scr = v[0].iter().fold(0, |x, &b| x * 2 + b as i64);
    oxy * scr
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2_724_524);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2_775_870);
    }
}
