type Coordinate = (i32, i32);

fn parse(input: &str) -> TargetArea {
    let (_, raw_coord) = input.split_once(": x=").unwrap();

    let (xs, ys) = raw_coord.split_once(", y=").unwrap();

    let (x1, x2) = xs
        .split_once("..")
        .map(|(x1, x2)| (x1.parse().unwrap(), x2.parse().unwrap()))
        .unwrap();

    let (y1, y2) = ys
        .split_once("..")
        .map(|(y1, y2)| (y1.parse().unwrap(), y2.parse().unwrap()))
        .unwrap();

    TargetArea {
        tl: (std::cmp::min(x1, x2), std::cmp::max(y1, y2)),
        br: (std::cmp::max(x1, x2), std::cmp::min(y1, y2)),
    }
}

fn find_good_shots<'a>(ta: &'a TargetArea) -> impl Iterator<Item = Shot> + 'a {
    (1..=ta.br.0).flat_map(move |vx| {
        (ta.br.1..=500).filter_map(move |vy| {
            let mut shot = Shot::new((0, 0), (vx, vy));

            while shot.pos.1 > ta.br.1 {
                shot.step();

                if shot.has_reached_ta(ta) {
                    return Some(shot);
                }
            }

            None
        })
    })
}

pub fn p1(input: &str) -> i32 {
    let target = parse(input);

    find_good_shots(&target)
        .max_by_key(|shot| shot.highest_y)
        .unwrap()
        .highest_y
}

pub fn p2(input: &str) -> usize {
    let target = parse(input);

    find_good_shots(&target).count()
}

#[derive(Debug)]
struct TargetArea {
    tl: Coordinate,
    br: Coordinate,
}

#[derive(Debug, Clone)]
struct Shot {
    pos: Coordinate,
    velocity: (i32, i32),

    highest_y: i32,
    init_velocity: (i32, i32),
}

impl Shot {
    fn new(pos: Coordinate, velocity: (i32, i32)) -> Self {
        Self {
            pos,
            velocity,
            init_velocity: velocity,
            highest_y: 0,
        }
    }

    fn has_reached_ta(&self, ta: &TargetArea) -> bool {
        self.pos.0 >= ta.tl.0
            && self.pos.0 <= ta.br.0
            && self.pos.1 >= ta.br.1
            && self.pos.1 <= ta.tl.1
    }

    fn step(&mut self) {
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;

        self.highest_y = self.highest_y.max(self.pos.1);

        self.velocity.1 -= 1;

        if self.velocity.0 > 0 {
            self.velocity.0 -= 1
        } else if self.velocity.0 < 0 {
            self.velocity.0 += 1
        }
    }
}

#[test]
fn test_p1_1() {
    assert_eq!(p1("target area: x=20..30, y=-10..-5"), 45);
}

#[test]
fn test_p2() {
    assert_eq!(p2("target area: x=20..30, y=-10..-5"), 112);
}
