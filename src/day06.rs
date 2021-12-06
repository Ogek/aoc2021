use std::collections::VecDeque;

fn parse(input: &str) -> VecDeque<usize> {
    input
        .split(",")
        .map(|n| n.parse().unwrap())
        .fold(VecDeque::from([0; 9]), |mut v, x: usize| {
            v[x] += 1;
            v
        })
}
fn simulation(data: &mut VecDeque<usize>, days: usize) {
    for _ in 0..days {
        data.rotate_left(1);
        data[6] += data[8];
    }
}

pub fn p1(input: &str) -> usize {
    let mut fishes = parse(input);

    simulation(&mut fishes, 80);

    fishes.iter().sum()
}

pub fn p2(input: &str) -> usize {
    let mut fishes = parse(input);

    simulation(&mut fishes, 256);

    fishes.iter().sum()
}

#[test]
fn test_p1() {
    assert_eq!(p1("3,4,3,1,2"), 5934);
}

#[test]
fn test_p2() {
    assert_eq!(p2("3,4,3,1,2"), 26984457539);
}
