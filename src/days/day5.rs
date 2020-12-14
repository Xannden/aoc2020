use std::fs::File;
use std::io::Read;

type Data = Vec<usize>;

fn parse(input: String) -> Data {
    input
        .lines()
        .map(|line| parse_line(line)).map(|(row, col)| row * 8 + col)
        .collect::<Vec<_>>()
}

fn parse_line(line: &str) -> (usize, usize) {
    let temp = line
        .replace(['F', 'L'].as_ref(), "0")
        .replace(['B', 'R'].as_ref(), "1");

    let (row, col) = temp.split_at(7);

    (
        usize::from_str_radix(row, 2).unwrap(),
        usize::from_str_radix(col, 2).unwrap(),
    )
}

fn part1(data: Data) -> Data {
    println!("Part 1");

    let max = data.iter().max().unwrap();

    println!("Max is {}", max);

    data
}

fn part2(mut data: Data) {
    println!("Part 2",);

    data.sort_unstable();

    for id in data.windows(2)
    {
        if id[0]+1 != id[1] {
            println!("Id is {:?}", id[0] + 1);
        }
    }
}

pub fn run() {
    let mut file = File::open("input/day5.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(data);

    data = part1(data);
    part2(data);
}
