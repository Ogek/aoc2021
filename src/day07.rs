fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

pub fn p1(input: &str) -> usize {
    let data = parse(input);
    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();

    (min..max)
        .map(|desired_position| {
            data.iter()
                .map(move |n| (*n as isize - desired_position as isize).abs() as usize)
                .sum()
        })
        .min()
        .unwrap()
}

pub fn p2(input: &str) -> usize {
    let data = parse(input);
    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();

    (min..max)
        .map(|desired_position| {
            data.iter()
                .map(move |n| {
                    let steps = (*n as isize - desired_position as isize).abs() as usize;
                    steps * (steps + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[test]
fn test_p1() {
    assert_eq!(p1("16,1,2,0,4,2,7,1,2,14"), 37);
}

#[test]
fn test_p2() {
    assert_eq!(p2("16,1,2,0,4,2,7,1,2,14"), 168);
}
