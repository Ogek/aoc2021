use std::collections::HashSet;

type CavesMap = Vec<Vec<usize>>;
type Coordinate = (usize, usize);
type Height = usize;

fn parse(input: &str) -> CavesMap {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn iter_caves_with_coord(caves: &CavesMap) -> impl Iterator<Item = (Height, Coordinate)> + '_ {
    caves
        .iter()
        .enumerate()
        .flat_map(move |(y, cl)| cl.iter().enumerate().map(move |(x, h)| (*h, (x, y))))
}

fn get_neigh<'a>(
    caves: &'a CavesMap,
    coord: Coordinate,
) -> impl Iterator<Item = (Height, Coordinate)> + 'a {
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .filter_map(move |(dx, dy)| {
            let x = coord.0 as isize + dx;
            let y = coord.1 as isize + dy;

            if y < 0 || y >= caves.len() as isize {
                return None;
            }

            if x < 0 || x >= caves[0].len() as isize {
                return None;
            }

            let x = x as usize;
            let y = y as usize;

            return Some((caves[y][x], (x, y)));
        })
}

fn is_low(caves: &CavesMap, (height, coord): (Height, Coordinate)) -> bool {
    get_neigh(&caves, coord)
        .chain(std::iter::once((height, coord)))
        .map(|(height, _)| height)
        .min()
        .unwrap()
        == height
}

fn low_points(caves: &CavesMap) -> impl Iterator<Item = (Height, Coordinate)> + '_ {
    iter_caves_with_coord(&caves).filter(move |cave| is_low(&caves, *cave))
}

pub fn p1(input: &str) -> usize {
    let caves = parse(input);

    low_points(&caves).map(|(h, _)| h + 1).sum()
}

fn find_basins<'a>(caves: &CavesMap) -> impl Iterator<Item = HashSet<Coordinate>> + '_ {
    fn compute_basin(caves: &CavesMap, current: (Height, Coordinate)) -> HashSet<Coordinate> {
        std::iter::once(current.1)
            .chain(
                get_neigh(caves, current.1)
                    .filter(move |(neigh, _)| current.0 < *neigh && *neigh < 9)
                    .flat_map(|neigh| compute_basin(caves, neigh)),
            )
            .collect()
    }

    low_points(caves).map(|info| compute_basin(caves, info))
}

pub fn p2(input: &str) -> usize {
    let caves = parse(input);

    let mut basins = find_basins(&caves)
        .map(|basin| basin.len())
        .collect::<Vec<usize>>();

    basins.sort();

    basins.iter().rev().take(3).product()
}

#[test]
fn test_p1() {
    assert_eq!(
        p1("2199943210
3987894921
9856789892
8767896789
9899965678"),
        15
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2("2199943210
3987894921
9856789892
8767896789
9899965678"),
        1134
    );
}

#[test]
fn test_neigh() {
    let caves = parse(
        "2199943210
3987894921
9856789892
8767896789
9899965678",
    );

    let neigh: Vec<usize> = get_neigh(&caves, (0, 0)).map(|(n, _)| n).collect();

    assert_eq!(neigh, vec![3, 1]);

    let neigh: Vec<usize> = get_neigh(&caves, (5, 3)).map(|(n, _)| n).collect();

    assert_eq!(neigh, [6, 6, 8, 8]);
}
