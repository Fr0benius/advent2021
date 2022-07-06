use crate::util::parsing::re_parser;

pub const INPUT: &str = include_str!("../input/24.txt");

pub fn part1(input: &str) -> i64 {
    let mut st = vec![];
    let mut res = [0i64; 14];
    for (i, (div, a, b)) in get_triples(input).into_iter().enumerate() {
        if div == 1 {
            st.push((i, b));
        } else {
            let (j, mut d) = st.pop().unwrap();
            d += a;
            assert!(d.abs() < 9);
            if d > 0 {
                res[i] = 9;
                res[j] = 9 - d;
            } else {
                res[j] = 9;
                res[i] = 9 + d;
            }
        }
    }
    res.into_iter().fold(0, |a, b| 10 * a + b)
}

pub fn part2(input: &str) -> i64 {
    let mut st = vec![];
    let mut res = [0i64; 14];
    for (i, (div, a, b)) in get_triples(input).into_iter().enumerate() {
        if div == 1 {
            st.push((i, b));
        } else {
            let (j, mut d) = st.pop().unwrap();
            d += a;
            assert!(d.abs() < 9);
            if d > 0 {
                res[i] = 1 + d;
                res[j] = 1;
            } else {
                res[j] = 1 - d;
                res[i] = 1;
            }
        }
    }
    res.into_iter().fold(0, |a, b| 10 * a + b)
}

fn get_triples(input: &str) -> Vec<(i64, i64, i64)> {
    let commands: Vec<&str> = r"
mul x 0
add x z
mod x 26
div z (.*)
add x (.*)
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y (.*)
mul y x
add z y"
        .trim()
        .lines()
        .collect();
    let re = commands.join(";");
    
input
        .split("inp w\n")
        .skip(1)
        .map(|blob| blob.trim().replace('\n',";")).map(|blob| re_parser(&re)(&blob)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 39_924_989_499_969);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 16_811_412_161_117);
    }
}
