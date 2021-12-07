fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn solve<F>(data: &Vec<usize>, f: F) -> usize
where
    F: Fn(usize, usize) -> usize,
{
    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();

    (min..max)
        .map(|desired_position| data.iter().map(|n| f(*n, desired_position)).sum())
        .min()
        .unwrap()
}

pub fn p1(input: &str) -> usize {
    let data = parse(input);

    solve(&data, |n: usize, desired_pos: usize| {
        (n as isize - desired_pos as isize).abs() as usize
    })
}

pub fn p2(input: &str) -> usize {
    let data = parse(input);

    solve(&data, |n: usize, desired_pos: usize| {
        let steps = (n as isize - desired_pos as isize).abs() as usize;
        steps * (steps + 1) / 2
    })
}

#[test]
fn test_p1() {
    assert_eq!(p1("16,1,2,0,4,2,7,1,2,14"), 37);
}

#[test]
fn test_p2() {
    assert_eq!(p2("16,1,2,0,4,2,7,1,2,14"), 168);
}
