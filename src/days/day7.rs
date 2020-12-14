use std::{collections::HashMap, fs::File, io::Read};

type Data = HashMap<String, Vec<(usize, String)>>;

fn parse(input: String) -> Data {
    let mut data = HashMap::new();

    for line in input.lines() {
        let (bag, contents) = line.split_at(line.find("contain").unwrap());

        let bag = &bag[..bag.find("bags").unwrap()].trim();

        let contents = &contents[7..];

        let mut bags = Vec::new();
        for b in contents
            .split(", ")
            .map(|l| l.trim_matches([' ', '.'].as_ref()))
        {
            if b == "no other bags" {
                break;
            } else {
                let (num, color) = b.split_at(b.find(' ').unwrap());

                let color = &color[..color.find("bag").unwrap()].trim();

                bags.push((num.parse().unwrap(), color.to_string()));
            }
        }

        data.insert(bag.to_string(), bags);
    }

    data
}

pub fn run() {
    let mut file = File::open("input/day7.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(data);

    data = part1(data);
    part2(data);
}

fn part1(data: Data) -> Data {
    println!("Part 1");

    fn contains(data: &Data, key: &str) -> bool {
        if key == "shiny gold" {
            return true;
        }

        for child in &data[key] {
            if contains(data, &child.1) {
                return true;
            }
        }

        false
    }

    let result: usize = data
        .keys()
        .map(|key| {
            if key == "shiny gold" {
                0
            } else if contains(&data, key) {
                1
            } else {
                0
            }
        })
        .sum();

    println!("{}", result);

    data
}

fn part2(data: Data) {
    println!("Part 2",);

    fn count_bags(data: &Data, bag: &str) -> usize {
        let children = &data[bag];

        if children.is_empty() {
            return 1;
        }

        children.iter().map(|child| count_bags(data, &child.1) * child.0).sum::<usize>() + 1
    }

    println!("{}", count_bags(&data, "shiny gold")-1);
}
