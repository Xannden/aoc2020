use std::{fs::File, io::Read};

type Data = Board;

pub fn run() {
    let mut file = File::open("input/day11.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(data);

    part1(&mut data);
    part2(&mut data);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Space,
    EmptyChair,
    OccupiedChair,
}

#[derive(Clone)]
struct Board {
    data: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Board {
    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            None
        } else {
            Some(&self.data[x as usize + y as usize * self.width])
        }
    }

    fn print(&self) {
        for (idx, tile) in self.data.iter().enumerate() {
            match tile {
                Tile::Space => print!("."),
                Tile::EmptyChair => print!("L"),
                Tile::OccupiedChair => print!("#"),
            }

            if idx % self.width == self.width - 1 {
                println!();
            }
        }
    }

    fn step<F: Fn(i32, i32) -> Tile>(&self, rule: F) -> Board {
        let mut new_state = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                new_state.push(rule(x as i32, y as i32));
            }
        }

        Board {
            data: new_state,
            width: self.width,
            height: self.height,
        }
    }
}

fn parse(input: String) -> Data {
    let width = input.find("\r\n").unwrap();
    let height = input.lines().count();

    let mut data = Vec::new();

    for c in input.as_bytes() {
        match c {
            b'L' => data.push(Tile::EmptyChair),
            b'#' => data.push(Tile::OccupiedChair),
            b'.' => data.push(Tile::Space),
            _ => (),
        }
    }

    Board {
        data,
        width,
        height,
    }
}

fn neighbors(x: i32, y: i32) -> Vec<(i32, i32)> {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

fn part1(data: &mut Data) {
    println!("Part 1");

    let mut map = data.clone();

    // println!("Step 0");
    // map.print();

    // let mut step = 0;

    loop {
        let new = map.step(|x, y| {
            let current = map.get_tile(x as i32, y as i32).unwrap();
            let occupied = neighbors(x, y)
                .iter()
                .map(|(x, y)| map.get_tile(*x, *y))
                .filter(|tile| {
                    if let Some(tile) = tile {
                        matches!(tile, Tile::OccupiedChair)
                    } else {
                        false
                    }
                })
                .count();

            match current {
                Tile::EmptyChair if occupied == 0 => Tile::OccupiedChair,
                Tile::OccupiedChair if occupied >= 4 => Tile::EmptyChair,
                tile => *tile,
            }
        });

        if map.data == new.data {
            break;
        }

        map = new;

        // step += 1;
        // println!("Step {}", step);
        // map.print();
    }

    let occupied = map
        .data
        .iter()
        .filter(|tile| matches!(tile, Tile::OccupiedChair))
        .count();

    println!("Occupied {}", occupied);
}

fn directions() -> Vec<(i32, i32)> {
    vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
}

fn part2(data: &mut Data) {
    println!("Part 2");

    let mut map = data.clone();

    // println!("Step 0");
    // map.print();

    // let mut step = 0;

    loop {
        let new = map.step(|x, y| {
            // println!("{} {}", x,y);
            let current = map.get_tile(x as i32, y as i32).unwrap();
            let occupied = directions()
                .iter()
                .map(|dir| {
                    let mut curr = (x + dir.0, y + dir.1);

                    while map.get_tile(curr.0, curr.1) == Some(&Tile::Space) {
                        curr = (curr.0 + dir.0, curr.1 + dir.1);
                    }

                    map.get_tile(curr.0, curr.1)
                })
                .filter(|tile| {
                    if let Some(tile) = tile {
                        matches!(tile, Tile::OccupiedChair)
                    } else {
                        false
                    }
                })
                .count();

            // if *current == Tile::EmptyChair {
            //     println!("{}", occupied);
            // }

            match current {
                Tile::EmptyChair if occupied == 0 => Tile::OccupiedChair,
                Tile::OccupiedChair if occupied >= 5 => Tile::EmptyChair,
                tile => *tile,
            }
        });

        if map.data == new.data {
            break;
        }

        map = new;

        // step += 1;
        // println!("Step {}", step);
        // map.print();
    }

    let occupied = map
        .data
        .iter()
        .filter(|tile| matches!(tile, Tile::OccupiedChair))
        .count();

    println!("Occupied {}", occupied);
}
