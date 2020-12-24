use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::Read,
    iter::FromIterator,
};

pub fn run() {
    let mut file = File::open("input/day22.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let data = parse(data);

    // println!("{:?}", data);

    part1((data.0.clone(), data.1.clone()));
    part2(data);
}

fn parse(input: String) -> (Vec<usize>, Vec<usize>) {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let player_1 = sections[0]
        .lines()
        .skip(1)
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let player_2 = sections[1]
        .lines()
        .skip(1)
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (player_1, player_2)
}

fn part1((mut player_1, mut player_2): (Vec<usize>, Vec<usize>)) {
    println!("Part 1");

    while !player_1.is_empty() && !player_2.is_empty() {
        // println!("P1: {:?}", player_1);
        // println!("P2: {:?}", player_2);
        if player_1[0] > player_2[0] {
            player_1.rotate_left(1);
            player_1.push(player_2.remove(0));
        } else {
            player_2.rotate_left(1);
            player_2.push(player_1.remove(0));
        }
    }

    // println!("P1: {:?}", player_1);
    // println!("P2: {:?}", player_2);

    let score: usize = if player_1.is_empty() {
        player_2
    } else {
        player_1
    }
    .iter()
    .rev()
    .enumerate()
    .map(|(idx, &card)| (idx + 1) * card)
    .sum();

    println!("Winners Score: {}", score);
}

fn part2((player_1, player_2): (Vec<usize>, Vec<usize>)) {
    println!("Part 2");

    fn game(player_1: &[usize], player_2: &[usize]) -> (bool, Vec<usize>, Vec<usize>) {
        let mut player_1 = Vec::from(player_1);
        let mut player_2 = Vec::from(player_2);
        let mut seen = HashSet::new();

        while !player_1.is_empty() && !player_2.is_empty() {
            // println!("P1 {:?}", player_1);
            // println!("P2 {:?}", player_2);

            if !seen.insert((player_1.clone(), player_2.clone())) {
                return (true, player_1, player_2);
            }

            let winner = if player_1.len() > player_1[0] && player_2.len() > player_2[0] {
                // println!("Recurse");
                game(&player_1[1..=player_1[0]], &player_2[1..=player_2[0]]).0
            } else {
                player_1[0] > player_2[0]
            };

            if winner {
                player_1.rotate_left(1);
                player_1.push(player_2.remove(0));
            } else {
                player_2.rotate_left(1);
                player_2.push(player_1.remove(0));
            }
        }

        // println!("P1 {:?}", player_1);
        // println!("P2 {:?}", player_2);

        (player_2.is_empty(), player_1, player_2)
    }

    let result = game(&player_1, &player_2);

    let score: usize = if result.1.is_empty() {
        result.2
    } else {
        result.1
    }
    .iter()
    .rev()
    .enumerate()
    .map(|(idx, &card)| (idx + 1) * card)
    .sum();

    println!("Winners Score: {}", score);
}
