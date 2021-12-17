use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type Coordinate = (usize, usize);
type CaveMap = HashMap<Coordinate, usize>;

fn parse(input: &str) -> CaveMap {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x, y), c.to_digit(10).unwrap() as usize))
        })
        .collect()
}

fn neighs<'a>(
    map: &'a CaveMap,
    (x, y): Coordinate,
) -> impl Iterator<Item = (Coordinate, usize)> + '_ {
    [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .iter()
        .filter_map(move |(dx, dy)| {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            let ncoord = (nx, ny);
            if let Some(ncost) = map.get(&ncoord) {
                return Some(((nx, ny), *ncost));
            }

            None
        })
}

fn fewer_risky_path(map: &CaveMap, start: Coordinate, goal: Coordinate) -> Option<usize> {
    // Uncomment lines below and add the heuristic cost in the BinaryHeap for A*
    // Dijkstra works faster for this particular puzzle's path because we have to go diagonally
    // but we can move only vertically and horizontally so we can't have a heuristic which can justify
    // the additional cost of A*

    let mut dist: HashMap<Coordinate, usize> = map.keys().map(|&pos| (pos, usize::MAX)).collect();
    // let heuristics = |coord| map[&coord] + (goal.0 - coord.0) + (goal.1 - coord.1);
    // let mut edist: HashMap<Coordinate, usize> = dist.clone();

    // let sh = heuristics(start);

    dist.entry(start).and_modify(|n| *n = 0);
    // edist.entry(start).and_modify(|n| *n = sh);

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start)));

    while let Some(Reverse((cost, pos))) = heap.pop() {
        if pos == goal {
            return Some(cost);
        }

        for (npos, ncost) in neighs(map, pos) {
            let tcost = dist[&pos] + ncost;
            // let tecost = tcost + heuristics(npos);

            if tcost < dist[&npos] {
                // edist.insert(npos, tecost);
                dist.insert(npos, tcost);
                heap.push(Reverse((tcost, npos)));
            }
        }
    }

    None
}

pub fn p1(input: &str) -> usize {
    let map = parse(input);

    let mx = *map.keys().map(|(x, _)| x).max().unwrap();
    let my = *map.keys().map(|(_, y)| y).max().unwrap();

    fewer_risky_path(&map, (0, 0), (mx, my)).unwrap()
}

pub fn p2(input: &str) -> usize {
    let mut map = parse(input);

    let mx = *map.keys().map(|(x, _)| x).max().unwrap();
    let my = *map.keys().map(|(_, y)| y).max().unwrap();

    for (x, y) in (0..=my).flat_map(|y| (0..=mx).map(move |x| (x, y))) {
        for (tx, ty) in (0..5).flat_map(|y| (0..5).map(move |x| (x, y))) {
            let mut new_cost = map[&(x, y)] + (tx + ty);
            if new_cost > 9 {
                new_cost -= 9;
            }
            map.insert((x + tx * (mx + 1), y + ty * (my + 1)), new_cost);
        }
    }

    let mx = *map.keys().map(|(x, _)| x).max().unwrap();
    let my = *map.keys().map(|(_, y)| y).max().unwrap();

    fewer_risky_path(&map, (0, 0), (mx, my)).unwrap()
}

#[test]
fn test_p1() {
    assert_eq!(
        p1("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"),
        40
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"),
        315
    );
}
