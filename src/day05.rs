use std::collections::HashMap;

type Point = (isize, isize);
#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

fn parse_point(raw_point: &str) -> Point {
    raw_point
        .split_once(",")
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .unwrap()
}

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let (start, end) = l.split_once(" -> ").unwrap();

            Line {
                p1: parse_point(start),
                p2: parse_point(end),
            }
        })
        .collect()
}

pub fn p1(input: &str) -> usize {
    let lines = parse(input);

    let map: HashMap<Point, usize> = lines
        .iter()
        .filter(|line| !line.is_diagonal())
        .flat_map(|line| line.iter_points())
        .fold(HashMap::new(), |mut acc, point| {
            *acc.entry(point).or_insert(0) += 1;
            acc
        });

    map.values().filter(|v| **v >= 2).count()
}

pub fn p2(input: &str) -> usize {
    let lines = parse(input);

    let map: HashMap<Point, usize> =
        lines
            .iter()
            .flat_map(|line| line.iter_points())
            .fold(HashMap::new(), |mut acc, point| {
                *acc.entry(point).or_insert(0) += 1;
                acc
            });

    map.values().filter(|v| **v >= 2).count()
}

impl Line {
    fn iter_points(&self) -> impl Iterator<Item = Point> + '_ {
        let x_diff = (self.p1.0 - self.p2.0).abs();
        let y_diff = (self.p1.1 - self.p2.1).abs();

        let dx = (self.p2.0 - self.p1.0).signum();
        let dy = (self.p2.1 - self.p1.1).signum();

        let dist = std::cmp::max(x_diff, y_diff);

        (0..=dist).map(move |i| {
            let x = self.p1.0 + (dx * i as isize);
            let y = self.p1.1 + (dy * i as isize);

            (x, y)
        })
    }

    fn is_diagonal(&self) -> bool {
        self.p1.0 != self.p2.0 && self.p1.1 != self.p2.1
    }
}

#[test]
fn test_p1() {
    assert_eq!(
        p1("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"),
        5
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2("0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"),
        12
    );
}

#[test]
fn test_points() {
    let line = Line {
        p1: (1, 1),
        p2: (3, 3),
    };
    assert_eq!(
        line.iter_points().collect::<Vec<_>>(),
        vec![(1, 1), (2, 2), (3, 3)]
    );

    let line = Line {
        p1: (0, 0),
        p2: (3, 3),
    };
    assert_eq!(
        line.iter_points().collect::<Vec<_>>(),
        vec![(0, 0), (1, 1), (2, 2), (3, 3)]
    );

    let line = Line {
        p1: (9, 7),
        p2: (7, 9),
    };
    assert_eq!(
        line.iter_points().collect::<Vec<_>>(),
        vec![(9, 7), (8, 8), (7, 9)]
    );

    let line = Line {
        p1: (8, 0),
        p2: (0, 8),
    };
    assert_eq!(
        line.iter_points().collect::<Vec<_>>(),
        vec![
            (8, 0),
            (7, 1),
            (6, 2),
            (5, 3),
            (4, 4),
            (3, 5),
            (2, 6),
            (1, 7),
            (0, 8)
        ]
    );

    let line = Line {
        p1: (0, 8),
        p2: (8, 0),
    };

    assert_eq!(
        line.iter_points().collect::<Vec<_>>(),
        vec![
            (0, 8),
            (1, 7),
            (2, 6),
            (3, 5),
            (4, 4),
            (5, 3),
            (6, 2),
            (7, 1),
            (8, 0)
        ]
    );
}
