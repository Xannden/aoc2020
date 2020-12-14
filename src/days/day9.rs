use std::{fs::File, io::Read};

type Data = Vec<usize>;

fn parse(input: String) -> Data {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn run() {
    let mut file = File::open("input/day9.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(data);

    data = part1(data);
    part2(data);
}

fn part1(data: Data) -> Data {
    println!("Part 1");

    let num = find_number(&data, 25).unwrap();

    println!("{}", num);

    data
}

fn find_number(data: &[usize], size: usize) -> Option<usize> {
    'main: for window in data.windows(size + 1) {

        for first in &window[..size] {
            for sec in &window[..size] {
                if first != sec && first + sec == window[size] {
                    continue 'main;
                }
            }
        }

        return Some(window[size]);
    }

    None
}

fn part2(data: Data) {
    println!("Part 2",);

    let num = find_number(&data, 25).unwrap();

    for start in 0..data.len() {
        let mut sum = data[start];
        let mut pos = start;

        while sum < num {
            pos += 1;
            sum += data[pos];
        }

        if sum == num {
            println!("{}..={}", start, pos);
            let min = data[start..=pos].iter().min().unwrap();
            let max = data[start..=pos].iter().max().unwrap();

            println!("{}+{}={}", min, max, min + max);
            break;
        }
    }
}
