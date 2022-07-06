use crate::util::{parsing::re_parser, bin_search::BinSearch};

pub const INPUT: &str = include_str!("../input/22.txt");

fn solve(cuboids: &[Cuboid]) -> i64 {
    let mut x_vals : Vec<i64> = cuboids.iter().flat_map(|c| [c.xmin, c.xmax+1]).collect();
    x_vals.sort_unstable();
    x_vals.dedup();
    let n = x_vals.len();
    let mut y_vals : Vec<i64> = cuboids.iter().flat_map(|c| [c.ymin, c.ymax+1]).collect();
    y_vals.sort_unstable();
    y_vals.dedup();
    let m = y_vals.len();
    let mut z_vals : Vec<i64> = cuboids.iter().flat_map(|c| [c.zmin, c.zmax+1]).collect();
    z_vals.sort_unstable();
    z_vals.dedup();
    let l = z_vals.len();
    let mut on = vec![vec![vec![false; l]; m];n];
    for c in cuboids {
        let i0 = x_vals.lower_bound(&c.xmin);
        let i1 = x_vals.lower_bound(&(c.xmax + 1));
        let j0 = y_vals.lower_bound(&c.ymin);
        let j1 = y_vals.lower_bound(&(c.ymax + 1));
        let k0 = z_vals.lower_bound(&c.zmin);
        let k1 = z_vals.lower_bound(&(c.zmax + 1));
        for i in i0..i1 {
            for j in j0..j1 {
                for k in k0..k1 {
                    on[i][j][k] = c.on;
                }
            }
        }
    }
    let mut res = 0;
    for i in 0..n {
        for j in 0..m {
            for k in 0..l {
                if on[i][j][k] {
                    res += (x_vals[i+1] - x_vals[i]) *(y_vals[j+1] - y_vals[j]) *(z_vals[k+1] - z_vals[k]) ;
                }
            }
        }
    }
    res
}

pub fn part1(input: &str) -> i64 {
    let parse = re_parser(r"(.*) x=(.*)\.\.(.*),y=(.*)\.\.(.*),z=(.*)\.\.(.*)");
    let cuboids: Vec<_> = input
        .lines()
        .filter_map(|line| {
            let (op, xmin, xmax, ymin, ymax, zmin, zmax): (&str, _, _, _, _, _, _) = parse(line);
            let cuboid = Cuboid {
                on: op == "on",
                xmin,
                xmax,
                ymin,
                ymax,
                zmin,
                zmax,
            };
            if [xmin, xmax, ymin, ymax, zmin, zmax]
                .into_iter()
                .all(|x| (-50..=50).contains(&x))
            {
                Some(cuboid)
            } else {
                None
            }
        })
        .collect();
    solve(&cuboids)
}

pub fn part2(input: &str) -> i64 {
    let parse = re_parser(r"(.*) x=(.*)\.\.(.*),y=(.*)\.\.(.*),z=(.*)\.\.(.*)");
    let cuboids: Vec<_> = input
        .lines()
        .map(|line| {
            let (op, xmin, xmax, ymin, ymax, zmin, zmax): (&str, _, _, _, _, _, _) = parse(line);
            Cuboid {
                on: op == "on",
                xmin,
                xmax,
                ymin,
                ymax,
                zmin,
                zmax,
            }
        })
        .collect();
    solve(&cuboids)
}

#[derive(Clone, Copy, Debug)]
struct Cuboid {
    on: bool,
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
    zmin: i64,
    zmax: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 603_661);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1_237_264_238_382_479);
    }
}
