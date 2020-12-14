use std::{collections::HashSet, fs::File, io::Read};

type Data = Vec<Op>;

#[derive(Debug)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse(input: String) -> Data {
    let mut ops = Vec::new();

    for line in input.lines() {
        let mut split = line.split(' ');
        let op = split.next().unwrap();
        let arg = split.next().unwrap();

        ops.push(match op {
            "nop" => Op::Nop(arg.parse().unwrap()),
            "acc" => Op::Acc(arg.parse().unwrap()),
            "jmp" => Op::Jmp(arg.parse().unwrap()),
            _ => continue,
        });
    }

    ops
}

pub fn run() {
    let mut file = File::open("input/day8.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(data);

    data = part1(data);
    part2(data);
}

fn part1(data: Data) -> Data {
    println!("Part 1");

    println!("{}", run_program(&data).1);

    data
}

fn run_program(data: &[Op]) -> (bool, i32) {
    let mut acc = 0;
    let mut ip = 0i32;

    let mut set = HashSet::new();

    while (ip as usize) < data.len() {
        // println!("@{} acc:{} {:?}", ip, acc, data[ip as usize]);

        if !set.insert(ip) {
            // println!("Result is {}", acc);
            return (false, acc);
        }

        match data[ip as usize] {
            Op::Nop(_) => ip += 1,
            Op::Acc(arg) => {
                acc += arg;

                ip += 1;
            }
            Op::Jmp(arg) => ip += arg,
        }
    }

    (true, acc)
}

fn part2(mut data: Data) {
    println!("Part 2");

    for op in 0..data.len() {
        match data[op] {
            Op::Nop(arg) => data[op] = Op::Jmp(arg),
            Op::Jmp(arg) => data[op] = Op::Nop(arg),
            _ => (),
        }

        let result = run_program(&data);

        if result.0 {
            println!("{}", result.1);
            break;
        }

        match data[op] {
            Op::Nop(arg) => data[op] = Op::Jmp(arg),
            Op::Jmp(arg) => data[op] = Op::Nop(arg),
            _ => (),
        }
    }
}
