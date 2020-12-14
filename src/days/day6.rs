use std::{collections::HashMap, fs::File};
use std::{collections::HashSet, io::Read};

type Data = Vec<Vec<String>>;

fn parse(input: String) -> Data {
    let mut data = Vec::new();
    for group in input.split("\n\n") {
        let mut group_data = Vec::new();
        for c in group.lines() {
            group_data.push(c.to_string());
        }

        data.push(group_data);
    }

    data
}

fn part1(data: Data) -> Data {
    println!("Part 1");

    let result = data.iter().fold(0, |acc, group| {
        let mut set = HashSet::new();

        for c in group.iter().flat_map(|l| l.chars()) {
            set.insert(c);
        }

        acc + set.len()
    });

    println!("Sum {}", result);

    data
}

fn part2(data: Data) {
    println!("Part 2",);

    let mut result = 0;

    for group in &data {
        let mut map = HashMap::new();

        let entries = group.len();
        for c in group.iter().flat_map(|l| l.chars()) {
            map.entry(c).and_modify(|c| *c += 1).or_insert(1);
        }

        result += map.values().filter(|c| **c == entries).count();

    }

    println!("Sum {}", result);
}

pub fn run() {
    let mut file = File::open("input/day6.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(data);

    data = part1(data);
    part2(data);
}
