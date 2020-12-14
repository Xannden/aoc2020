use std::{collections::HashMap, fs::File, io::Read};

pub fn run() {
    let mut file = File::open("input/day14.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let data = parse(data);

    // println!("{:?}", data);

    part1(&data);
    part2(&data);
}

struct Device {
    mask: String,
    mem: HashMap<u64, u64>,
}

enum Op {
    Mask(String),
    Mem(u64, u64),
}

fn parse(input: String) -> Vec<Op> {
    let mut ops = Vec::new();

    let mem = regex::Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();
    let mask = regex::Regex::new(r"mask = (.*)").unwrap();

    for line in input.lines() {
        if mem.is_match(line) {
            let captures = mem.captures(line).unwrap();

            ops.push(Op::Mem(
                line[captures.get(1).unwrap().range()].parse().unwrap(),
                line[captures.get(2).unwrap().range()].parse().unwrap(),
            ));
        } else {
            let captures = mask.captures(line).unwrap();

            ops.push(Op::Mask(line[captures.get(1).unwrap().range()].to_string()));
        }
    }

    ops
}

impl Device {
    fn new() -> Device {
        Device {
            mask: String::new(),
            mem: HashMap::new(),
        }
    }

    fn run_op(&mut self, op: &Op) {
        match op {
            Op::Mask(mask) => {
                self.mask = mask.clone();
            }
            Op::Mem(addr, val) => {
                let val = self.apply_mask(*val);

                self.mem.insert(*addr, val);
            }
        }
    }

    fn apply_mask(&self, val: u64) -> u64 {
        let mut result = val;

        for (idx, c) in self.mask.as_bytes().iter().rev().enumerate() {
            match c {
                b'0' => result &= !(1 << idx),
                b'1' => result |= 1 << idx,
                _ => (),
            }
        }

        result
    }
}

fn part1(ops: &[Op]) {
    println!("Part 1");

    let mut device = Device::new();

    for op in ops {
        device.run_op(op);
    }

    println!("Sum {}", device.mem.values().sum::<u64>());
}

impl Device {
    #[allow(clippy::clippy::naive_bytecount)]
    fn run_op_v2(&mut self, op: &Op) {
        match op {
            Op::Mask(mask) => self.mask = mask.clone(),
            Op::Mem(addr, val) => {
                let floating_count = self.mask.as_bytes().iter().filter(|c| **c == b'X').count();

                for n in 0..(1 << floating_count) {
                    let addr = self.apply_mask_v2(*addr, n);

                    // println!("mem[{}] = {}", addr, val);

                    self.mem.insert(addr, *val);
                }
            }
        }
    }

    fn apply_mask_v2(&self, val: u64, num: u64) -> u64 {
        let mut result = val;

        let mut num_idx = 0;

        for (idx, c) in self.mask.as_bytes().iter().rev().enumerate() {
            match c {
                b'0' => (),
                b'1' => result |= 1 << idx,
                b'X' => {
                    //Clear floating bit
                    result &= !(1 << idx);

                    result |= get_bit(num, num_idx) << idx;

                    num_idx += 1;
                }
                _ => unreachable!(),
            }
        }

        result
    }
}

fn get_bit(num: u64, bit: u8) -> u64 {
    num >> bit & 1
}

fn part2(ops: &[Op]) {
    println!("Part 2");

    let mut device = Device::new();

    for op in ops {
        device.run_op_v2(op);
    }

    println!("Sum {}", device.mem.values().sum::<u64>());
}
