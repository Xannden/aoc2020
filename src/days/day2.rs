use std::fs::File;
use std::io::Read;

type Data = String;



fn parse(mut input: File) -> Data {
    println!("Day 2");
    let mut data = String::new();

    input.read_to_string(&mut data).unwrap();

    data
}

fn part1(data: Data) -> Data {
    let mut valid = 0;

    for line in data.lines() {
        let mut split = line.split([' ', '-', ':'].as_ref());
        let low: usize = split.next().unwrap().parse().unwrap();
        let high: usize = split.next().unwrap().parse().unwrap();
        let char = split.next().unwrap().chars().next().unwrap();

        let pass = split.nth(1).unwrap();

        let count = pass.chars().filter(|c| c == &char).count();

        if count <= high && count >= low {
            valid += 1;
        }
    }

    println!("Part 1 {}", valid);

    data
}

fn part2(data: Data) {
    let mut valid = 0;

    for line in data.lines() {
        let mut split = line.split(|c| c == '-' || c == ':' || c == ' ');
        let mut low: usize = split.next().unwrap().parse().unwrap();
        let mut high: usize = split.next().unwrap().parse().unwrap();
        let char = split.next().unwrap().chars().next().unwrap() as u8;

        low -= 1;
        high -= 1;

        let pass = split.nth(1).unwrap().as_bytes();

        if ((pass[low] == char) && (pass[high] != char))
            || ((pass[low] != char) && (pass[high] == char))
        {
            valid += 1;
        }
    }

    println!("Part 2 {}", valid);
}

pub fn run() {
    let file = File::open("input/day2.txt").unwrap();
    let mut data = parse(file);

    data = part1(data);
    part2(data);
}
