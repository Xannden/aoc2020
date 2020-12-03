use std::fs::File;
use std::io::Read;

type Data = ();

fn parse(data: String) -> Data {}

fn part1(data: Data) -> Data {
    println!("Part 1");

    data
}

fn part2(data: Data) {
    println!("Part 2",);
}

pub fn run() {
    let mut file = File::open("input/day3.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(data);

    data = part1(data);
    part2(data);
}
