use std::{fs::File, io::Read};

type Data = Vec<Step>;

pub fn run() {
    let mut file = File::open("input/day12.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(data);

    // println!("{:?}", data);

    part1(&mut data);
    part2(&mut data);
}

#[derive(Debug, Clone, Copy)]
enum Step {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

fn parse(input: String) -> Data {
    input
        .lines()
        // .inspect(|t| println!("{}", t))
        .map(|line| match &line[0..1] {
            "N" => Step::N(line[1..].parse().unwrap()),
            "S" => Step::S(line[1..].parse().unwrap()),
            "E" => Step::E(line[1..].parse().unwrap()),
            "W" => Step::W(line[1..].parse().unwrap()),
            "L" => Step::L(line[1..].parse().unwrap()),
            "R" => Step::R(line[1..].parse().unwrap()),
            "F" => Step::F(line[1..].parse().unwrap()),
            _ => unreachable!(),
        })
        .collect()
}

struct Ship {
    x: i32,
    y: i32,
    facing: i32,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            facing: 90,
        }
    }

    fn step(&mut self, step: Step) {
        match step {
            Step::N(v) => self.y += v,
            Step::S(v) => self.y -= v,
            Step::E(v) => self.x += v,
            Step::W(v) => self.x -= v,
            Step::L(v) => {
                self.facing -= v;
            }
            Step::R(v) => {
                self.facing += v;
            }
            Step::F(v) => match self.facing % 360 {
                0 => self.y += v,
                90 | -270 => self.x += v,
                180 | -180 => self.y -= v,
                270 | -90 => self.x -= v,
                _ => unreachable!("{}", self.facing),
            },
        }
    }
}

fn part1(data: &mut Data) {
    println!("Part 1");

    let mut ship = Ship::new();
    // println!("{} {} {}", ship.y, ship.x, ship.facing);

    for step in data {
        let before = [ship.y, ship.x, ship.facing];
        ship.step(*step);

        // println!("{:?}->[{}, {}, {}]", before, ship.y, ship.x, ship.facing);
    }

    println!("{}+{}={}", ship.y, ship.x, ship.x.abs() + ship.y.abs());
}

struct Ship2 {
    pos: (i32, i32),
    waypoint: (i32, i32),
}

impl Ship2 {
    fn new() -> Ship2 {
        Ship2 {
            pos: (0, 0),
            waypoint: (10, 1),
        }
    }

    fn step(&mut self, step: Step) {
        match step {
            Step::N(v) => self.waypoint.1 += v,
            Step::S(v) => self.waypoint.1 -= v,
            Step::E(v) => self.waypoint.0 += v,
            Step::W(v) => self.waypoint.0 -= v,
            Step::L(v) => self.rotate_left(v),
            Step::R(v) => self.rotate_right(v),
            Step::F(v) => {
                self.pos = (
                    self.pos.0 + self.waypoint.0 * v,
                    self.pos.1 + self.waypoint.1 * v,
                )
            }
        }
    }

    fn rotate_left(&mut self, amount: i32) {
        match amount {
            90 => self.waypoint = (-self.waypoint.1, self.waypoint.0),
            180 => self.waypoint = (-self.waypoint.0, -self.waypoint.1),
            270 => self.waypoint = (self.waypoint.1, -self.waypoint.0),
            _ => unreachable!(),
        }
    }

    fn rotate_right(&mut self, amount: i32) {
        match amount {
            90 => self.waypoint = (self.waypoint.1, -self.waypoint.0),
            180 => self.waypoint = (-self.waypoint.0, -self.waypoint.1),
            270 => self.waypoint = (-self.waypoint.1, self.waypoint.0),
            _ => unreachable!(),
        }
    }
}

fn part2(data: &mut Data) {
    println!("Part 2");

    let mut ship = Ship2::new();
    // println!("{} {} {}", ship.y, ship.x, ship.facing);

    for step in data {
        let before = [ship.pos, ship.waypoint];
        ship.step(*step);

        println!("{:?}->[{:?}, {:?}]", before, ship.pos, ship.waypoint);
    }

    println!("{}+{}={}", ship.pos.0, ship.pos.1, ship.pos.0.abs() + ship.pos.1.abs());
}
