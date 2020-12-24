use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
    ops::Range,
};

pub fn run() {
    let mut file = File::open("input/day24.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let data = parse(data);

    // println!("{:?}", data);

    part1(data.clone());
    part2(data);
}

#[derive(Debug, Clone, Copy)]
enum Move {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Move {
    fn step(&self, pos: (i64, i64, i64)) -> (i64, i64, i64) {
        match self {
            Move::East => (pos.0 + 1, pos.1 - 1, pos.2),
            Move::SouthEast => (pos.0, pos.1 - 1, pos.2 + 1),
            Move::SouthWest => (pos.0 - 1, pos.1, pos.2 + 1),
            Move::West => (pos.0 - 1, pos.1 + 1, pos.2),
            Move::NorthWest => (pos.0, pos.1 + 1, pos.2 - 1),
            Move::NorthEast => (pos.0 + 1, pos.1, pos.2 - 1),
        }
    }
}

fn parse_line(line: &str) -> Vec<Move> {
    let line = line.as_bytes();
    let mut pos = 0;
    let mut moves = Vec::new();

    while pos < line.len() {
        let m = match (line[pos], line.get(pos + 1)) {
            (b'e', _) => {
                pos += 1;
                Move::East
            }
            (b's', Some(b'e')) => {
                pos += 2;
                Move::SouthEast
            }
            (b's', Some(b'w')) => {
                pos += 2;
                Move::SouthWest
            }
            (b'w', _) => {
                pos += 1;
                Move::West
            }
            (b'n', Some(b'e')) => {
                pos += 2;
                Move::NorthEast
            }
            (b'n', Some(b'w')) => {
                pos += 2;
                Move::NorthWest
            }
            _ => unreachable!(),
        };

        moves.push(m);
    }
    moves
}

fn parse(input: String) -> Vec<Vec<Move>> {
    input.lines().map(parse_line).collect()
}

fn part1(input: Vec<Vec<Move>>) {
    println!("Part 1");

    let mut board: HashMap<(i64, i64, i64), bool> = HashMap::new();

    for tile in input {
        let mut pos = (0, 0, 0);
        for m in tile {
            pos = m.step(pos);
        }

        board.entry(pos).and_modify(|t| *t = !*t).or_insert(true);
    }

    let result = board.values().filter(|t| **t).count();

    println!("Result {}", result);
}

fn neighbors(pos: (i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    vec![
        (pos.0 + 1, pos.1 - 1, pos.2),
        (pos.0 + 1, pos.1, pos.2 - 1),
        (pos.0, pos.1 + 1, pos.2 - 1),
        (pos.0 - 1, pos.1 + 1, pos.2),
        (pos.0 - 1, pos.1, pos.2 + 1),
        (pos.0, pos.1 - 1, pos.2 + 1),
    ]
}

fn bounds(board: &HashMap<(i64, i64, i64), bool>) -> (Range<i64>, Range<i64>, Range<i64>) {
    let min_x = board.keys().map(|tile| tile.0).min().unwrap();
    let max_x = board.keys().map(|tile| tile.0).max().unwrap();

    let min_y = board.keys().map(|tile| tile.1).min().unwrap();
    let max_y = board.keys().map(|tile| tile.1).max().unwrap();

    let min_z = board.keys().map(|tile| tile.2).min().unwrap();
    let max_z = board.keys().map(|tile| tile.2).max().unwrap();

    (min_x..max_x, min_y..max_y, min_z..max_z)
}

//This works but is very slow
fn part2(input: Vec<Vec<Move>>) {
    println!("Part 2");

    let mut board: HashMap<(i64, i64, i64), bool> = HashMap::new();

    for tile in input {
        let mut pos = (0, 0, 0);
        for m in tile {
            pos = m.step(pos);
        }

        board.entry(pos).and_modify(|t| *t = !*t).or_insert(true);
    }

    for day in 0..100 {
        let mut new_board = HashMap::new();

        let (x_bound, y_bound, z_bound) = bounds(&board);

        for z in z_bound.start - 1..=z_bound.end + 1 {
            for y in y_bound.start - 1..=y_bound.end + 1 {
                for x in x_bound.start - 1..=x_bound.end + 1 {
                    let neighbor_count = neighbors((x, y, z))
                        .iter()
                        .filter(|pos| board.get(pos) == Some(&true))
                        .count();

                    if (board.get(&(x, y, z)) == Some(&true)
                        && !(neighbor_count == 0 || neighbor_count > 2))
                        || neighbor_count == 2
                    {
                        new_board.insert((x, y, z), true);
                    }
                }
            }
        }

        board = new_board;

        println!("Day {}", day + 1,);
    }

    println!("Result {}", board.values().filter(|t| **t).count());
}
