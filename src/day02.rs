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

fn make_move(submarine: Submarine, m: Move) -> Submarine {
    match m {
        Move::Forward(n) => Submarine {
            x: submarine.x + n,
            ..submarine
        },
        Move::Up(n) => Submarine {
            y: submarine.y.checked_sub(n).unwrap_or(0),
            ..submarine
        },
        Move::Down(n) => Submarine {
            y: submarine.y + n,
            ..submarine
        },
    }
}

pub fn p1(input: &str) -> usize {
    let submarine = parse(input).fold(Submarine::default(), make_move);

    submarine.x * submarine.y
}

fn make_move2(submarine: Submarine, m: Move) -> Submarine {
    match m {
        Move::Forward(n) => Submarine {
            x: submarine.x + n,
            y: submarine.y + n * submarine.aim,
            ..submarine
        },
        Move::Up(n) => Submarine {
            aim: submarine.aim.checked_sub(n).unwrap_or(0),
            ..submarine
        },
        Move::Down(n) => Submarine {
            aim: submarine.aim + n,
            ..submarine
        },
    }
}

pub fn p2(input: &str) -> usize {
    let submarine = parse(input).fold(Submarine::default(), make_move2);

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
