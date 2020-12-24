use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
};

// This code is horribly complicated there is probably a simpler way to do this
// I don't recommend trying to understand this
pub fn run() {
    let mut file = File::open("input/day20.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let data = parse(data);

    // println!("{:?}", data);

    let image = part1(&data);
    part2(image);
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    R0,
    R90,
    R180,
    R270,
    R0F,
    R90F,
    R180F,
    R270F,
}

impl Orientation {
    fn all() -> Vec<Orientation> {
        vec![
            Orientation::R0,
            Orientation::R90,
            Orientation::R180,
            Orientation::R270,
            Orientation::R0F,
            Orientation::R90F,
            Orientation::R180F,
            Orientation::R270F,
        ]
    }
}

enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    fn rotate(&self, ori: Orientation) -> (Side, bool) {
        match self {
            Side::Top => match ori {
                Orientation::R0 => (Side::Top, false),
                Orientation::R90 => (Side::Left, true),
                Orientation::R180 => (Side::Bottom, true),
                Orientation::R270 => (Side::Right, false),
                Orientation::R0F => (Side::Top, true),
                Orientation::R90F => (Side::Right, true),
                Orientation::R180F => (Side::Bottom, false),
                Orientation::R270F => (Side::Left, false),
            },
            Side::Right => match ori {
                Orientation::R0 => (Side::Right, false),
                Orientation::R90 => (Side::Top, false),
                Orientation::R180 => (Side::Left, true),
                Orientation::R270 => (Side::Bottom, true),
                Orientation::R0F => (Side::Left, false),
                Orientation::R90F => (Side::Top, true),
                Orientation::R180F => (Side::Right, true),
                Orientation::R270F => (Side::Bottom, false),
            },
            Side::Bottom => match ori {
                Orientation::R0 => (Side::Bottom, false),
                Orientation::R90 => (Side::Right, true),
                Orientation::R180 => (Side::Top, true),
                Orientation::R270 => (Side::Left, false),
                Orientation::R0F => (Side::Bottom, true),
                Orientation::R90F => (Side::Left, true),
                Orientation::R180F => (Side::Top, false),
                Orientation::R270F => (Side::Right, false),
            },
            Side::Left => match ori {
                Orientation::R0 => (Side::Left, false),
                Orientation::R90 => (Side::Bottom, false),
                Orientation::R180 => (Side::Right, true),
                Orientation::R270 => (Side::Top, true),
                Orientation::R0F => (Side::Right, false),
                Orientation::R90F => (Side::Bottom, true),
                Orientation::R180F => (Side::Left, true),
                Orientation::R270F => (Side::Top, false),
            },
        }
    }
}

#[derive(PartialEq, Eq)]
struct Edge(Vec<bool>);

impl Edge {
    fn print(&self) {
        for v in &self.0 {
            if *v {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
}

impl std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in &self.0 {
            if *v {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile(u32, Orientation);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl Pos {
    fn neighbors(&self) -> Vec<Pos> {
        vec![
            Pos(self.0, self.1 - 1),
            Pos(self.0, self.1 + 1),
            Pos(self.0 - 1, self.1),
            Pos(self.0 + 1, self.1),
        ]
    }
}

struct Board {
    tiles: HashMap<u32, Vec<bool>>,
}

impl Board {
    fn new(tiles: HashMap<u32, Vec<bool>>) -> Board {
        Board { tiles }
    }

    fn ids(&self) -> Vec<u32> {
        self.tiles.keys().cloned().collect()
    }

    fn get_edge(&self, tile: Tile, side: Side) -> Edge {
        let (side, flip) = side.rotate(tile.1);

        let mut data = match side {
            Side::Top => Vec::from(&self.tiles.get(&tile.0).unwrap()[..10]),
            Side::Right => {
                let data = self.tiles.get(&tile.0).unwrap();
                let mut edge = Vec::new();

                for idx in (9..100).step_by(10) {
                    edge.push(data[idx]);
                }

                edge
            }
            Side::Bottom => Vec::from(&self.tiles.get(&tile.0).unwrap()[90..]),
            Side::Left => {
                let data = self.tiles.get(&tile.0).unwrap();
                let mut edge = Vec::new();

                for idx in (0..100).step_by(10) {
                    edge.push(data[idx]);
                }
                edge
            }
        };

        if flip {
            data.reverse();
        }

        Edge(data)
    }
}

fn parse(input: String) -> Board {
    let mut tiles = HashMap::new();

    for tile in input.split("\n\n") {
        let mut lines = tile.lines();
        let id = &lines.next().unwrap()[5..];
        let id = id[..id.len() - 1].parse::<u32>().unwrap();

        let mut data = Vec::new();
        for c in lines.flat_map(|line| line.as_bytes()) {
            match c {
                b'.' => data.push(false),
                b'#' => data.push(true),
                _ => (),
            }
        }

        tiles.insert(id, data);
    }

    Board::new(tiles)
}

fn is_valid(board: &Board, puzzle: &HashMap<Pos, Tile>, pos: Pos, tile: Tile) -> bool {
    if let Some(top) = puzzle.get(&Pos(pos.0, pos.1 + 1)) {
        let bottom_edge = board.get_edge(*top, Side::Bottom);
        let top_edge = board.get_edge(tile, Side::Top);
        if bottom_edge != top_edge {
            return false;
        }

        // println!(
        //     "Tile: {:?} bottom is {:?} and Tile: {:?} top is {:?}",
        //     top, bottom_edge, tile, top_edge
        // );
    }

    if let Some(left) = puzzle.get(&Pos(pos.0 - 1, pos.1)) {
        let right_edge = board.get_edge(*left, Side::Right);
        let left_edge = board.get_edge(tile, Side::Left);
        if right_edge != left_edge {
            return false;
        }

        // println!(
        //     "Tile: {:?} right is {:?} and Tile: {:?} left is {:?}",
        //     left, right_edge, tile, left_edge
        // );
    }

    if let Some(right) = puzzle.get(&Pos(pos.0 + 1, pos.1)) {
        let left_edge = board.get_edge(*right, Side::Left);
        let right_edge = board.get_edge(tile, Side::Right);
        if left_edge != right_edge {
            return false;
        }

        // println!(
        //     "Tile: {:?} left is {:?} and Tile: {:?} right is {:?}",
        //     right, left_edge, tile, right_edge
        // );
    }

    if let Some(bottom) = puzzle.get(&Pos(pos.0, pos.1 - 1)) {
        let top_edge = board.get_edge(*bottom, Side::Top);
        let bottom_edge = board.get_edge(tile, Side::Bottom);
        if bottom_edge != top_edge {
            return false;
        }

        // println!(
        //     "Tile: {:?} top is {:?} and Tile: {:?} bottom is {:?}",
        //     bottom, top_edge, tile, bottom_edge
        // );
    }

    true
}

fn part1(board: &Board) -> (Vec<bool>, usize) {
    println!("Part 1");

    let mut ids = board.ids();
    let mut puzzle: HashMap<Pos, Tile> = HashMap::new();

    puzzle.insert(Pos(0, 0), Tile(ids.remove(0), Orientation::R180F));

    let mut frontier = VecDeque::from(Pos(0, 0).neighbors());

    while !frontier.is_empty() {
        let pos = frontier.pop_front().unwrap();

        if puzzle.get(&pos).is_some() {
            continue;
        }

        // println!("Pos: {:?}", pos);
        'ids: for idx in 0..ids.len() {
            for o in Orientation::all() {
                if is_valid(board, &puzzle, pos, Tile(ids[idx], o)) {
                    puzzle.insert(pos, Tile(ids[idx], o));
                    ids.remove(idx);
                    frontier.extend(pos.neighbors().iter().filter(|n| puzzle.get(*n).is_none()));
                    break 'ids;
                }
            }
        }
    }

    let x_min = puzzle.keys().map(|key| key.0).min().unwrap();
    let x_max = puzzle.keys().map(|key| key.0).max().unwrap();
    let y_min = puzzle.keys().map(|key| key.1).min().unwrap();
    let y_max = puzzle.keys().map(|key| key.1).max().unwrap();

    let top_left = puzzle.get(&Pos(x_min, y_min)).unwrap().0 as u64;
    let top_right = puzzle.get(&Pos(x_max, y_min)).unwrap().0 as u64;
    let bot_left = puzzle.get(&Pos(x_min, y_max)).unwrap().0 as u64;
    let bot_right = puzzle.get(&Pos(x_max, y_max)).unwrap().0 as u64;

    println!("{}", top_left * top_right * bot_left * bot_right);

    combine(board, &puzzle, (Pos(x_min, y_min), Pos(x_max, y_max)))
}

fn combine(board: &Board, puzzle: &HashMap<Pos, Tile>, bounds: (Pos, Pos)) -> (Vec<bool>, usize) {
    let mut data: Vec<bool> = Vec::with_capacity(puzzle.len() * 8 * 8);
    let width = bounds.1 .0 - bounds.0 .0 + 1;
    let height = bounds.1 .1 - bounds.0 .1 + 1;

    println!("Width: {}, Height: {}", width, height);

    for tile_y in (bounds.0 .1..=bounds.1 .1).rev() {
        for y in 1..=8 {
            for tile_x in bounds.0 .0..=bounds.1 .0 {
                let tile = puzzle[&Pos(tile_x, tile_y)];
                // println!("({}, {}) {:?}", tile_x, tile_y, tile);
                let tile_data = &board.tiles[&tile.0];
                for x in 1..=8 {
                    let x_rev = 9 - x;
                    let y_rev = 9 - y;

                    match tile.1 {
                        Orientation::R0 => data.push(tile_data[x + y * 10]),
                        Orientation::R90 => data.push(tile_data[y + x_rev * 10]),
                        Orientation::R180 => data.push(tile_data[x_rev + y_rev * 10]),
                        Orientation::R270 => data.push(tile_data[y_rev + x * 10]),
                        Orientation::R0F => data.push(tile_data[x_rev + y * 10]),

                        Orientation::R90F => data.push(tile_data[y_rev + x_rev * 10]),
                        Orientation::R180F => data.push(tile_data[x + y_rev * 10]),
                        Orientation::R270F => data.push(tile_data[y + x * 10]),
                    }
                }
            }
        }
    }

    (data, width as usize * 8)
}

struct Image {
    data: Box<[bool]>,
    width: usize,
    height: usize,
    ori: Orientation,
}

impl Image {
    fn get(&self, pos: (usize, usize)) -> bool {
        let x = pos.0;
        let y = pos.1;

        let x_rev = self.width - x - 1;
        let y_rev = self.height - y - 1;

        // println!("{} {} {}", x_rev, y_rev, y + x_rev * self.width);

        match self.ori {
            Orientation::R0 => self.data[x + y * self.width],
            Orientation::R90 => self.data[y + x_rev * self.width],
            Orientation::R180 => self.data[x_rev + y_rev * self.width],
            Orientation::R270 => self.data[y_rev + x * self.width],
            Orientation::R0F => self.data[x_rev + y * self.width],

            Orientation::R90F => self.data[y_rev + x_rev * self.width],
            Orientation::R180F => self.data[x + y_rev * self.width],
            Orientation::R270F => self.data[y + x * self.width],
        }
    }
}

static MONSTER: &[u8] = "                  # #    ##    ##    ### #  #  #  #  #  #   ".as_bytes();

fn is_monster(image: &Image, pos: (usize, usize)) -> bool {
    for y in 0..3 {
        for x in 0..20 {
            if MONSTER[x + y * 20] == b'#' && !image.get((pos.0 + x, pos.1 + y)) {
                // println!("({}, {}) = {}", pos.0 + x, pos.1 + y, image.get((pos.0 + x, pos.1 + y)));
                return false;
            }
        }
    }

    true
}

fn part2(image: (Vec<bool>, usize)) {
    println!("Part 2");

    let mut image = Image {
        height: image.0.len() / image.1,
        width: image.1,
        data: image.0.into_boxed_slice(),
        ori: Orientation::R0,
    };

    let mut monster_count = 0;

    for o in Orientation::all() {
        println!("{:?}", o);
        image.ori = o;
        for y in 0..=(image.height - 3) {
            for x in 0..=(image.width - 20) {
                if is_monster(&image, (x, y)) {
                    monster_count += 1;
                    println!("MONSTER");
                }
                // println!("({}, {})", x, y);
            }
        }
    }

    let count = image.data.iter().filter(|c| **c).count();

    println!("Result: {}", count - monster_count * 15);
}
