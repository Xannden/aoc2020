use std::{fs::File, io::Read};

pub fn run() {
    let mut file = File::open("input/day13.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(data);

    // println!("{:?}", data);

    part1(&mut data);
    part2(&mut data);
}

fn parse(input: String) -> (usize, Vec<Option<usize>>) {
    let mut lines = input.lines();

    let start_time = lines.next().unwrap().parse().unwrap();

    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|bus| bus.parse().ok())
        .collect();

    (start_time, buses)
}

fn part1((start, buses): &mut (usize, Vec<Option<usize>>)) {
    let mut time = *start;

    loop {
        for bus in buses.iter() {
            if let Some(bus) = &bus {
                if time % *bus == 0 {
                    let wait = time - *start;
                    println!("{}-{}={}", time, start, wait);
                    println!("{}*{}={}", wait, *bus, wait * *bus);
                    return;
                }
            }
        }

        time += 1;
    }
}

fn part2((start, buses): &mut (usize, Vec<Option<usize>>)) {
    let buses = buses
        .iter()
        .enumerate()
        .filter_map(|(offset, bus)| {
            if let Some(bus) = bus {
                Some((offset, *bus))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut step = buses[0].1;
    let mut time = 0;
    let mut check_bus = 1;

    // This works by finding where the first two buses line up correctly
    // once you have that you know that they will always align on the least common multiple
    // of the bus ids so we take larger step every time we find a new bus aligning with the ones
    // we already found
    loop {
        // This is a loop incase we happen to find the position where multiple busses align at the same time
        while check_bus < buses.len() && (time + buses[check_bus].0) % buses[check_bus].1 == 0 {
            // This only works because all of the buses are prime numbers
            // This should be step = lcm(step, buses[check_bus].1)
            //  but because they are prime lcm is equvalent to just multiplying them
            step *= buses[check_bus].1;
            check_bus += 1;
        }

        // This is just to leave the loop when we have all buses aligned correctly
        if check_bus >= buses.len() {
            break;
        }

        time += step;
    }

    println!("Time: {}", time);
}
