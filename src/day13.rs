use std::collections::HashSet;

type Coordinate = (usize, usize);

#[derive(Debug)]
enum Instruction {
    FoldX(usize),
    FoldY(usize),
}

#[derive(Debug)]
struct Origami {
    dots: HashSet<Coordinate>,
}

fn parse(input: &str) -> (Origami, Vec<Instruction>) {
    let (dots, instructions) = input.split_once("\n\n").unwrap();
    (
        Origami::new(
            dots.lines()
                .map(|l| {
                    l.split_once(",")
                        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                        .unwrap()
                })
                .collect(),
        ),
        instructions
            .lines()
            .map(|l| {
                let n = l.split("=").last().unwrap().parse().unwrap();
                if l.contains("y") {
                    return Instruction::FoldY(n);
                }
                Instruction::FoldX(n)
            })
            .collect(),
    )
}

pub fn p1(input: &str) -> usize {
    let (origami, instructions) = parse(input);

    instructions
        .iter()
        .take(1)
        .fold(origami, do_origami)
        .dots
        .len()
}

pub fn p2(input: &str) -> () {
    let (origami, instructions) = parse(input);

    let folded = instructions.iter().fold(origami, do_origami);

    folded.print();
}

impl Origami {
    fn new(dots: HashSet<Coordinate>) -> Self {
        Self { dots }
    }

    fn print(&self) {
        let mx = *self.dots.iter().map(|(x, _)| x).max().unwrap();
        let my = *self.dots.iter().map(|(_, y)| y).max().unwrap();

        for y in 0..=my {
            for x in 0..=mx {
                if self.dots.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

fn do_origami(mut origami: Origami, instruction: &Instruction) -> Origami {
    origami.dots = origami
        .dots
        .iter()
        .map(|(x, y)| match instruction {
            Instruction::FoldX(n) => (*n - (*x as isize - *n as isize).abs() as usize, *y),
            Instruction::FoldY(n) => (*x, *n - (*y as isize - *n as isize).abs() as usize),
        })
        .collect();

    origami
}

#[test]
fn test_p1() {
    assert_eq!(
        p1("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"),
        17
    );
}
