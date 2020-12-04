use std::fs::File;
use std::io::Read;

type Data = Map;

struct Map {
    data: Box<[bool]>,
    width: usize,
    height: usize,
}

impl Map {
    fn is_tree(&self, x: usize, y: usize) -> bool {
        self.data[(x % self.width) + (y * self.width)]
    }

    fn print(&self) {
        println!("Width:{}, Height:{}", self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.is_tree(x, y) { "#" } else { "." });
            }
            println!();
        }
    }
}

fn parse(input: String) -> Data {
    let mut data = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'.' => data.push(false),
                b'#' => data.push(true),
                _ => break,
            }
            width = x + 1;
        }
        height = y + 1;
    }

    Map {
        data: data.into_boxed_slice(),
        width,
        height,
    }
}

fn part1(data: Data) -> Data {
    println!("Part 1");

    println!("Hit {} trees", check_slope(&data, (3, 1)));

    data
}

fn part2(data: Data) {
    println!("Part 2",);

    let first = check_slope(&data, (1, 1));
    let second = check_slope(&data, (3, 1));
    let third = check_slope(&data, (5, 1));
    let fourth = check_slope(&data, (7, 1));
    let fifth = check_slope(&data, (1, 2));

    println!("Result: {}", first * second * third * fourth * fifth);
}

fn check_slope(data: &Data, slope: (usize, usize)) -> i64 {
    let (mut x, mut y) = (0, 0);

    let mut trees = 0;
    while y < data.height {
        if data.is_tree(x, y) {
            trees += 1;
        }

        y += slope.1;
        x += slope.0;
    }

    trees
}

pub fn run() {
    let mut file = File::open("input/day3.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(data);

    data = part1(data);
    part2(data);
}
