use parse_display::FromStr;
use std::{collections::HashMap, fmt::Debug};
use std::{fs::File, io::Read};

pub fn run() {
    let mut file = File::open("input/day16.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let data = parse(data);

    // println!("{:?}", data);

    part1(&data);
    part2(&data);
}

#[derive(FromStr, Eq, PartialEq, Hash)]
#[display("{name}: {r1.0}-{r1.1} or {r2.0}-{r2.1}")]
struct Field {
    name: String,
    #[from_str(default)]
    r1: (u64, u64),
    #[from_str(default)]
    r2: (u64, u64),
}

impl Field {
    fn is_valid(&self, value: u64) -> bool {
        (value >= self.r1.0 && value <= self.r1.1) || (value >= self.r2.0 && value <= self.r2.1)
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct Data {
    fields: Vec<Field>,
    ticket: Vec<u64>,
    nearby: Vec<Vec<u64>>,
}

fn parse(input: String) -> Data {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    let fields = sections[0]
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();

    let ticket = sections[1]
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let nearby = sections[2]
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    Data {
        fields,
        ticket,
        nearby,
    }
}

fn invalid_sum(fields: &[Field], ticket: &[u64]) -> u64 {
    ticket
        .iter()
        .filter(|n| fields.iter().all(|f| !f.is_valid(**n)))
        .sum()
}

fn part1(data: &Data) {
    println!("Part 1");

    let result = data
        .nearby
        .iter()
        .map(|ticket| invalid_sum(&data.fields, ticket))
        .sum::<u64>();

    println!("{:#?}", result);
}

fn is_ticket_valid(fields: &[Field], ticket: &[u64]) -> bool {
    for n in ticket {
        if fields.iter().all(|f| !f.is_valid(*n)) {
            return false;
        }
    }

    true
}

fn part2(data: &Data) {
    println!("Part 2");

    let valid = data
        .nearby
        .iter()
        .filter(|t| is_ticket_valid(&data.fields, t))
        .collect::<Vec<_>>();

    fn is_field_valid(field: &Field, tickets: &[&Vec<u64>], pos: usize) -> bool {
        for ticket in tickets {
            if !field.is_valid(ticket[pos]) {
                return false;
            }
        }

        true
    }

    let mut possible = vec![Vec::new(); data.fields.len()];

    #[allow(clippy::needless_range_loop)]
    for idx in 0..data.fields.len() {
        for field in &data.fields {
            if is_field_valid(field, &valid, idx) {
                possible[idx].push(field);
            }
        }
    }

    // println!("{:#?}", possible);

    let mut mapping = HashMap::new();

    loop {
        let result = possible
            .iter_mut()
            .enumerate()
            .filter(|(_, f)| !f.is_empty())
            .find(|(_, f)| f.len() == 1);

        if let Some((idx, field)) = result {
            mapping.insert(field[0], idx);

            let field = field[0];

            for poss in &mut possible {
                if let Some(idx) = poss.iter().position(|f| f.name == field.name) {
                    poss.remove(idx);
                }
            }
        } else {
            break;
        }
    }

    let mut result = 1;

    for (key, value) in mapping {
        if key.name.starts_with("departure") {
            result *= data.ticket[value];
        }
    }

    println!("Result {}", result);
}
