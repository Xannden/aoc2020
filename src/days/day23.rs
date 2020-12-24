use std::{collections::VecDeque, fs::File, io::Read};

pub fn run() {
    let mut file = File::open("input/day23.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let data = parse(data);

    // println!("{:?}", data);

    part1(&data);
    part2(&data);
}

fn parse(input: String) -> Vec<usize> {
    input
        .as_bytes()
        .iter()
        .map(|b| (*b - b'0') as usize)
        .collect()
}

fn part1(input: &[usize]) {
    println!("Part 1");

    let max = *input.iter().max().unwrap();
    let mut state = [input[0]; 10];

    for val in input.windows(2) {
        state[val[0]] = val[1];
    }

    let mut current = input[0];
    for turn in 0..100 {
        // println!("-- move {} --", turn + 1);
        // print!("cups: ({})", current);

        // let mut temp = state[current];
        // while temp != current {
        //     print!(" {}", temp);
        //     temp = state[temp];
        // }
        // println!();

        //Pick up the next three
        let mut temp = current;
        let mut pickup = Vec::new();
        for _ in 0..3 {
            pickup.push(state[temp]);
            temp = state[temp];
        }
        state[current] = state[temp];
        // println!("pick up: {:?}", pickup);

        let mut dest = if current == 1 { max } else { current - 1 };
        while pickup.contains(&dest) {
            dest = if dest == 1 { max } else { dest - 1 };
        }

        // println!("destination: {}", dest);
        state[pickup[2]] = state[dest];
        state[dest] = pickup[0];

        // println!("{:?}", state);

        current = state[current];
    }

    let mut pos = state[1];
    let mut result = String::new();
    while pos != 1 {
        result.push_str(&pos.to_string());
        pos = state[pos];
    }

    println!("Result {}", result);
}

fn part2(input: &[usize]) {
    println!("Part 2");

    let max = 1000000;
    let mut state = Vec::with_capacity(1000000);

    for idx in 1..=1000000 {
        state.push(idx);
    }

    for val in input.windows(2) {
        state[val[0]] = val[1];
    }

    state[*input.last().unwrap()] = 10;
    state.push(input[0]);

    // println!("{:?}", state);

    let mut current = input[0];
    for turn in 0..10000000 {
        // println!("-- move {} --", turn + 1);
        // print!("cups: ({})", current);

        // let mut temp = state[current];
        // while temp != current {
        //     print!(" {}", temp);
        //     temp = state[temp];
        // }
        // println!();

        //Pick up the next three
        let mut temp = current;
        let mut pickup = Vec::new();
        for _ in 0..3 {
            pickup.push(state[temp]);
            temp = state[temp];
        }
        state[current] = state[temp];
        // println!("pick up: {:?}", pickup);

        let mut dest = if current == 1 { max } else { current - 1 };
        while pickup.contains(&dest) {
            dest = if dest == 1 { max } else { dest - 1 };
        }

        // println!("destination: {}", dest);
        state[pickup[2]] = state[dest];
        state[dest] = pickup[0];

        // println!("{:?}", state);

        current = state[current];
    }

    println!("Result {}", state[state[state[0]]] * state[state[0]]);
}
