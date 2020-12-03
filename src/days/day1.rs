use std::fs::File;
use std::io::Read;

type Data = Vec<u32>;

fn parse(mut input: File) -> Data {
    let mut data = String::new();

    input.read_to_string(&mut data).unwrap();

    data.lines().map(|l| l.parse().unwrap()).collect()
}

fn part1(values: Data) -> Data {
    for first in &values {
        for sec in &values {
            if first + sec == 2020 {
                println!("Part 1 {}", first * sec);
                return values;
            }
        }
    }

    panic!();
}

fn part2(values: Data) {
    for first in &values {
        for sec in &values {
            for three in &values {
                if first + sec + three == 2020 {
                    println!("Part 2 {}", first * sec * three);
                    return;
                }
            }
        }
    }
}

pub fn run() {
    let file = File::open("input/day1.txt").unwrap();
    let mut data = parse(file);

    data = part1(data);
    part2(data);
}
