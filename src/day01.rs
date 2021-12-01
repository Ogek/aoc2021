fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|n| n.parse().unwrap()).collect()
}

pub fn p1(input: &str) -> usize {
    parse(input).windows(2).filter(|w| w[1] > w[0]).count()
}

pub fn p2(input: &str) -> usize {
    let sums: Vec<usize> = parse(input).windows(3).map(|w| w.iter().sum()).collect();

    sums.windows(2).filter(|w| w[1] > w[0]).count()
}

#[test]
fn test_p1() {
    assert_eq!(
        p1("199
200
208
210
200
207
240
269
260
263"),
        7
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2("199
200
208
210
200
207
240
269
260
263"),
        5
    );
}
