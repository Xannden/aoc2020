use std::{fs::File, io::Read, iter::repeat};

type Data = Vec<usize>;

fn parse(input: String) -> Data {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn run() {
    let mut file = File::open("input/day10.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(data);

    part1(&mut data);
    part2(&mut data);
}

fn part1(data: &mut Data) {
    println!("Part 1");

    data.push(0);
    data.sort_unstable();
    data.push(data.last().unwrap() + 3);

    let mut one_diff = 0;
    let mut three_diff = 0;

    for pair in data.windows(2) {
        if pair[1] - pair[0] == 1 {
            one_diff += 1;
        } else if pair[1] - pair[0] == 3 {
            three_diff += 1;
        } else {
            println!("{}-{}", pair[1], pair[0])
        }
    }

    println!("{}*{}={}", one_diff, three_diff, one_diff * three_diff);
}

fn part2(data: &mut Data) {
    println!("Part 2");

    let mut paths = vec![1i64];
    paths.extend(repeat(0).take(data.len() - 1));

    for (idx, num) in data.iter().enumerate() {
        let mut check = idx + 1;
        while check < data.len() && data[check] - data[idx] <= 3 {
            paths[check] += paths[idx];
            check += 1;
        }
    }

    println!("{} Paths", paths.last().unwrap());
}
