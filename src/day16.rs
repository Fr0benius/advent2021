pub const INPUT: &str = include_str!("../input/16.txt");

pub fn part1(input: &str) -> u64 {
    let data: Vec<u8> = input
        .trim()
        .as_bytes()
        .chunks(2)
        .map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap())
        .collect();
    version(&data, &mut 0)
}

pub fn part2(input: &str) -> u64 {
    let data: Vec<u8> = input
        .trim()
        .as_bytes()
        .chunks(2)
        .map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap())
        .collect();
    eval(&data, &mut 0)
}

fn get_bits(data: &[u8], ix: &mut usize, k: usize) -> u64 {
    *ix += k;
    (*ix - k..*ix).fold(0, |a, i| a * 2 + ((data[i / 8] >> (7 - i % 8)) & 1) as u64)
}

fn version(data: &[u8], ix: &mut usize) -> u64 {
    let mut v = get_bits(data, ix, 3);
    if get_bits(data, ix, 3) == 4 {
        while get_bits(data, ix, 1) == 1 {
            *ix += 4;
        }
        *ix += 4;
        return v;
    }
    if get_bits(data, ix, 1) == 0 {
        let end = *ix + get_bits(data, ix, 15) as usize;
        while *ix < end {
            v += version(data, ix);
        }
    } else {
        for _ in 0..get_bits(data, ix, 11) {
            v += version(data, ix);
        }
    }
    v
}

fn eval(data: &[u8], ix: &mut usize) -> u64 {
    *ix += 3;
    let op = get_bits(data, ix, 3);
    if op == 4 {
        let mut v = 0;
        loop {
            let cont = get_bits(data, ix, 1) != 0;
            v = v * 16 + get_bits(data, ix, 4);
            if !cont {
                break;
            }
        }
        return v;
    }

    let mut vals = vec![];
    if get_bits(data, ix, 1) == 0 {
        let end = get_bits(data, ix, 15) as usize + *ix;
        while *ix < end {
            vals.push(eval(data, ix));
        }
    } else {
        for _ in 0..get_bits(data, ix, 11) {
            vals.push(eval(data, ix));
        }
    }
    match op {
        0 => vals.into_iter().sum(),
        1 => vals.into_iter().product(),
        2 => vals.into_iter().min().unwrap(),
        3 => vals.into_iter().max().unwrap(),
        5 => (vals[0] > vals[1]) as u64,
        6 => (vals[0] < vals[1]) as u64,
        7 => (vals[0] == vals[1]) as u64,
        _ => unreachable!(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 934);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 912_901_337_844);
    }
}
