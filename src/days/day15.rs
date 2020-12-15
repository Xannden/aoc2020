use std::{collections::HashMap, fs::File, io::Read};

pub fn run() {
    let mut file = File::open("input/day15.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let data = parse(data);

    // println!("{:?}", data);

    part1(data.clone());
    part2(data);
}

#[derive(Clone, Debug)]
struct Sequence {
    map: HashMap<u64, u64>,
    last: u64,
    turn: u64,
}

impl Iterator for Sequence {
    type Item = (u64, u64);

    fn next(&mut self) -> Option<(u64, u64)> {
        let next = if let Some(turn) = self.map.get(&self.last) {
            self.turn - *turn
        } else {
            0
        };

        self.map.insert(self.last, self.turn);

        self.last = next;
        self.turn += 1;

        Some((next, self.turn + 1))
    }
}

fn parse(input: String) -> Sequence {
    let mut map = HashMap::new();
    let mut last = 0;

    for kv in input.split(',').map(|num| num.parse().unwrap()).enumerate() {
        map.insert(kv.1, kv.0 as u64);
        last = kv.1;
    }

    map.remove(&last);

    Sequence {
        turn: map.len() as u64,
        map,
        last,
    }
}

fn part1(ops: Sequence) {
    println!("Part 1");

    for (num, turn) in ops {
        if turn == 2020 {
            println!("Turn: {} is {}", turn, num);
            break;
        }
    }
}

fn part2(ops: Sequence) {
    println!("Part 2");

    for (num, turn) in ops {
        if turn == 30000000 {
            println!("Turn: {} is {}", turn, num);
            break;
        }
    }
}
