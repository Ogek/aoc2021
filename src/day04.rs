use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Board {
    cells: HashMap<usize, (usize, usize)>,
    marked: HashSet<(usize, usize)>,
}

fn parse<'a>(input: &'a str) -> (impl Iterator<Item = usize> + 'a, Vec<Board>) {
    let mut lines = input.split("\n\n");

    (
        lines.next().unwrap().split(',').map(|n| n.parse().unwrap()),
        lines
            .map(|raw_board| {
                raw_board
                    .lines()
                    .enumerate()
                    .map(|(y, b_line)| {
                        b_line
                            .split_whitespace()
                            .enumerate()
                            .filter(|(_, n)| !n.is_empty())
                            .map(move |(x, n)| (n.parse::<usize>().unwrap(), (x, y)))
                    })
                    .flatten()
            })
            .map(|b| Board {
                cells: b.collect::<HashMap<usize, (usize, usize)>>(),
                marked: HashSet::new(),
            })
            .collect(),
    )
}

pub fn p1(input: &str) -> usize {
    let (numbers, mut boards) = parse(input);

    for n in numbers {
        for board in &mut boards {
            board.mark(n);

            if board.is_winner() {
                return board.get_unmarked_values().sum::<usize>() * n;
            }
        }
    }

    unreachable!()
}

pub fn p2(input: &str) -> usize {
    let (numbers, mut boards) = parse(input);

    for n in numbers {
        for board in &mut boards {
            board.mark(n);
        }
        if boards.len() == 1 {
            return boards[0].get_unmarked_values().sum::<usize>() * n;
        }
        boards.retain(|board| !board.is_winner());
    }

    unreachable!()
}

impl Board {
    fn is_winner(&self) -> bool {
        (0..5).any(|x| self.marked.iter().filter(|(x1, _)| *x1 == x).count() == 5)
            || (0..5).any(|y| self.marked.iter().filter(|(_, y1)| *y1 == y).count() == 5)
    }

    fn get_unmarked_values(&self) -> impl Iterator<Item = &usize> {
        self.cells
            .iter()
            .filter(|(_, c)| !self.marked.contains(&c))
            .map(|(v, _)| v)
    }

    fn mark(&mut self, n: usize) {
        if let Some(c) = self.cells.get(&n) {
            self.marked.insert(*c);
        }
    }
}

#[test]
fn test_p1() {
    assert_eq!(
        p1(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
        ),
        4512
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
        ),
        1924
    );
}
