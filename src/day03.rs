pub fn p1(input: &str) -> usize {
    let mut gamma = 0;
    let mut epsilon = 0;
    let lines = parse(input);

    for pos in 0..lines[0].len() {
        gamma <<= 1;
        epsilon <<= 1;

        if mcb(&lines, pos) == 1 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    gamma * epsilon
}

pub fn p2(input: &str) -> usize {
    let lines = parse(input);
    bit_criteria(lines.to_vec(), true) * bit_criteria(lines.to_vec(), false)
}

fn bit_criteria(mut lines: Vec<&str>, most: bool) -> usize {
    let mut pos = 0;

    while lines.len() > 1 {
        if mcb(&lines, pos) == 1 {
            lines.retain(|x| x.chars().nth(pos).unwrap() == if most { '1' } else { '0' })
        } else {
            lines.retain(|x| x.chars().nth(pos).unwrap() == if most { '0' } else { '1' })
        }
        pos += 1;
    }

    usize::from_str_radix(lines[0], 2).unwrap()
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn mcb(lines: &[&str], pos: usize) -> usize {
    let ones = lines
        .iter()
        .map(|l| l.chars().nth(pos).unwrap())
        .filter(|c| *c == '1')
        .count();

    if ones >= (lines.len() - ones) {
        return 1;
    }
    0
}

#[test]
fn test_p1() {
    assert_eq!(
        p1("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"),
        198
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2("00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"),
        230
    );
}
