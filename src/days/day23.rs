use std::{collections::VecDeque, fs::File, io::Read};

pub fn run() {
    let mut file = File::open("input/day23.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let data = parse(data);

    // println!("{:?}", data);

    part1(data.clone());
    part2();
}

fn parse(input: String) -> Vec<u8> {
    input.as_bytes().iter().map(|b| *b - b'0').collect()
}

#[allow(clippy::needless_collect)]
fn part1(mut input: Vec<u8>) {
    println!("Part 1");

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    for turn in 0..10 {
        println!("-- move {} --", turn + 1);
        println!("current cup: {}", input[turn % input.len()]);
        println!("cups: {:?}", input);


        // println!("pickup: {:?}", cups);

        // let mut dest = if cup == min { max } else { cup - 1 };


        // println!("destination: {}", dest);



        println!();
    }

    println!("{:?}", input);
}

fn part2() {
    println!("Part 2");
}
