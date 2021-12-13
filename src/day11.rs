const N: usize = 10;
const THRESHOLD: i8 = 10;
type Octopuses = [i8; N * N];

pub fn p1(input: &str) -> usize {
    let octopuses = &mut parse(input);
    (0..100).map(|_| next_step(octopuses)).sum()
}

pub fn p2(input: &str) -> usize {
    let octopuses = &mut parse(input);
    (1..).find(|_| next_step(octopuses) == N * N).unwrap()
}

fn parse(input: &str) -> Octopuses {
    input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn next_step(octopuses: &mut Octopuses) -> usize {
    octopuses.iter_mut().for_each(|x| match *x {
        -1 => *x = 1,
        _ => *x += 1,
    });

    (0..N * N)
        .filter_map(|pos| (octopuses[pos] >= THRESHOLD).then(|| flash(octopuses, pos)))
        .sum()
}

fn flash(octopuses: &mut Octopuses, pos: usize) -> usize {
    octopuses[pos] = -1;
    1 + [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .map(move |(di, dj)| {
        (
            ((pos / N) as isize + di) as usize,
            ((pos % N) as isize + dj) as usize,
        )
    })
    .filter(|&(ii, jj)| ii < N && jj < N)
    .map(|(ii, jj)| ii * N + jj)
    .filter_map(|pos| {
        (octopuses[pos] != -1)
            .then(|| octopuses[pos] += 1)
            .and((octopuses[pos] >= THRESHOLD).then(|| flash(octopuses, pos)))
    })
    .sum::<usize>()
}

#[test]
fn test_p1() {
    assert_eq!(
        p1("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"),
        1656
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"),
        195
    );
}
