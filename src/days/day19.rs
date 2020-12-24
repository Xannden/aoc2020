use std::{collections::HashMap, fs::File, io::Read};

pub fn run() {
    let mut file = File::open("input/day19.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(&data);

    // println!("{:?}", data);

    part1(&data);
    part2(&mut data);
}

#[derive(Debug)]
enum Rule {
    Term(char),
    NonTerm(Vec<Vec<usize>>),
}

struct Input<'i> {
    rules: HashMap<usize, Rule>,
    messages: Vec<&'i str>,
}

fn parse(input: &str) -> Input {
    let sections = input.split("\n\n").collect::<Vec<_>>();

    fn parse_rule(line: &str) -> (usize, Rule) {
        let mut split = line.split(':');

        let num = split.next().unwrap().parse().unwrap();

        let rules = split.next().unwrap().trim();

        let mut options = Vec::new();
        for option in rules.split('|') {
            if option.as_bytes()[0] == b'\"' {
                return (num, Rule::Term(option.as_bytes()[1] as char));
            }

            let mut rules = Vec::new();

            for r in option.split(' ') {
                if r.is_empty() {
                    continue;
                }
                rules.push(r.parse().unwrap());
            }

            options.push(rules);
        }

        (num, Rule::NonTerm(options))
    }

    let rules = sections[0].lines().map(|line| parse_rule(line)).collect();

    let messages = sections[1].lines().collect();

    Input { rules, messages }
}

impl<'i> Input<'i> {
    fn matches(&self, message: &str, rule: usize) -> bool {
        fn find_match<'i>(input: &Input, message: &'i str, rule: usize) -> (bool, &'i str) {
            match &input.rules[&rule] {
                Rule::Term(c) => {
                    if message.starts_with(*c) {
                        return (true, &message[1..]);
                    } else {
                        return (false, message);
                    }
                }
                Rule::NonTerm(options) => {
                    if rule == 8 && options.len() > 1 {
                        let mut temp = message;

                        loop {
                            let (result, new) = find_match(input, temp, 42);
                            if !result {
                                break;
                            }
                            temp = new;
                        }

                        return (temp != message, temp);
                    } else if rule == 11 && options.len() > 1 {
                        let mut result = find_match(input, message, 42);
                        let mut count = 1;
                        while result.0 {
                            result = find_match(input, result.1, 42);
                            count += 1;
                        }

                        for _ in 0..count {
                            result = find_match(input, result.1, 31);

                            if !result.0 {
                                return (false, result.1);
                            }
                        }

                        return (true, result.1);
                    } else {
                        'option: for option in options {
                            let mut message = message;
                            for rule in option {
                                let (result, new) = find_match(input, message, *rule);
                                if !result {
                                    continue 'option;
                                }
                                message = new;
                            }

                            return (true, message);
                        }
                    }
                }
            }
            (false, message)
        }

        find_match(self, message, rule) == (true, "")
    }
}

fn part1<'i>(input: &Input<'i>) {
    // println!("Part 1");

    // let mut sum = 0;
    // for message in &input.messages {
    //     if input.matches(message, 0) {
    //         println!("{}", message);
    //         sum += 1;
    //     }
    // }

    // println!("Valid {}", sum);
    println!();
}

fn part2<'i>(input: &mut Input<'i>) {
    println!("Part 2");

    input
        .rules
        .insert(8, Rule::NonTerm(vec![vec![42], vec![42, 8]]));
    input
        .rules
        .insert(11, Rule::NonTerm(vec![vec![42, 31], vec![42, 11, 31]]));

    let mut sum = 0;
    for message in &input.messages {
        if input.matches(message, 0) {
            println!("{}", message);
            sum += 1;
        }
    }

    println!("Valid {}", sum);
}
