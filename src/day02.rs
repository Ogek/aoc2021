use std::str::FromStr;

#[derive(Debug)]
enum Move {
    Forward(usize),
    Up(usize),
    Down(usize),
}

#[derive(Default, Debug)]
struct Submarine {
    x: usize,
    y: usize,
    aim: usize,
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = Move> + 'a {
    input.lines().map(|m| Move::from_str(m).unwrap())
}

pub fn p1(input: &str) -> usize {
    let submarine = parse(input).fold(Submarine::default(), |submarine, m| submarine.make_move(m));

    submarine.x * submarine.y
}

pub fn p2(input: &str) -> usize {
    let submarine = parse(input).fold(Submarine::default(), |submarine, m| submarine.make_move2(m));

    submarine.x * submarine.y
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            Some(("forward", n)) => Ok(Move::Forward(n.parse().unwrap())),
            Some(("up", n)) => Ok(Move::Up(n.parse().unwrap())),
            Some(("down", n)) => Ok(Move::Down(n.parse().unwrap())),
            _ => Err(()),
        }
    }
}

impl Submarine {
    fn make_move(self, m: Move) -> Submarine {
        match m {
            Move::Forward(n) => Submarine {
                x: self.x + n,
                ..self
            },
            Move::Up(n) => Submarine {
                y: self.y.checked_sub(n).unwrap_or(0),
                ..self
            },
            Move::Down(n) => Submarine {
                y: self.y + n,
                ..self
            },
        }
    }

    fn make_move2(self, m: Move) -> Submarine {
        match m {
            Move::Forward(n) => Submarine {
                x: self.x + n,
                y: self.y + n * self.aim,
                ..self
            },
            Move::Up(n) => Submarine {
                aim: self.aim.checked_sub(n).unwrap_or(0),
                ..self
            },
            Move::Down(n) => Submarine {
                aim: self.aim + n,
                ..self
            },
        }
    }
}

#[test]
fn test_p1() {
    assert_eq!(
        p1("forward 5
down 5
forward 8
up 3
down 8
forward 2"),
        150
    );
}

#[test]
fn test_p2() {
    assert_eq!(
        p2("forward 5
down 5
forward 8
up 3
down 8
forward 2"),
        900
    );
}
