use std::{fs::File, io::Read};

pub fn run() {
    let mut file = File::open("input/day24.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let data = parse(data);

    // println!("{:?}", data);

    part1();
    part2();
}

fn parse(input: String) {}

fn part1() {
    println!("Part 1");
}

fn part2() {
    println!("Part 2");
}
