use std::fs::File;
use std::io::Read;

type Data = Vec<Line>;

struct Line {
    min: usize,
    max: usize,
    character: char,
    pass: String,
}

fn parse(data: String) -> Data {
    println!("Day 2");

    let mut lines = Vec::new();

    for line in data.lines() {
        let mut split = line.split([' ', ':', '-'].as_ref());
        lines.push(Line {
            min: split.next().unwrap().parse().unwrap(),
            max: split.next().unwrap().parse().unwrap(),
            character: split.next().unwrap().parse().unwrap(),
            pass: split.nth(1).unwrap().to_string(),
        })
    }

    lines
}

fn part1(data: Data) -> Data {
    fn is_valid(line: &Line) -> bool {
        let matches = line.pass.matches(line.character).count();

        matches <= line.max && matches >= line.min
    }

    let valid = data.iter().filter(|line| is_valid(line)).count();

    println!("Part 1 {}", valid);

    data
}

fn part2(data: Data) {
    fn is_valid(line: &Line) -> bool {
        let mut chars = line.pass.chars();

        let first = chars.nth(line.min - 1) == Some(line.character);
        let second = chars.nth(line.max - line.min - 1) == Some(line.character);

        first ^ second
    }

    println!(
        "Part 2 {}",
        data.iter().filter(|line| is_valid(line)).count()
    );
}

pub fn run() {
    let mut file = File::open("input/day2.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(data);

    data = part1(data);
    part2(data);
}
