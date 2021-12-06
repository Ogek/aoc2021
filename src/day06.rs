fn parse(input: &str) -> [usize; 9] {
    input
        .split(",")
        .map(|n| n.parse().unwrap())
        .fold([0; 9], |mut v, x: usize| {
            v[x] += 1;
            v
        })
}

fn solve(mut data: [usize; 9], days: usize) -> usize {
    for _ in 0..days {
        let new_fishes = data[0];
        data.rotate_left(1);
        data[6] += new_fishes
    }

    data.iter().sum()
}

pub fn p1(input: &str) -> usize {
    let fishes = parse(input);

    solve(fishes, 80)
}

pub fn p2(input: &str) -> usize {
    let fishes = parse(input);

    solve(fishes, 256)
}

#[test]
fn test_p1() {
    assert_eq!(p1("3,4,3,1,2"), 5934);
}

#[test]
fn test_p2() {
    assert_eq!(p2("3,4,3,1,2"), 26984457539);
}
