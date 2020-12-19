use std::{collections::HashSet, fs::File, io::Read};

pub fn run() {
    let mut file = File::open("input/day17.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let data = parse(data);

    // println!("{:?}", data);

    part1(data.clone());
    part2(data);
}

#[derive(Clone, Debug)]
struct Extent {
    start: i64,
    end: i64,
}

impl Extent {
    fn new(start: i64, end: i64) -> Extent {
        Extent { start, end }
    }

    fn contains(&self, val: i64) -> bool {
        val >= self.start && val < self.end
    }

    fn len(&self) -> usize {
        (self.end - self.start) as usize
    }
}

#[derive(Clone)]
struct State {
    set: HashSet<(i64, i64, i64, i64)>,
}

impl State {
    fn get_bounds(&self) -> (Extent, Extent, Extent, Extent) {
        let mut x_range = (i64::MAX, i64::MIN);
        let mut y_range = (i64::MAX, i64::MIN);
        let mut z_range = (i64::MAX, i64::MIN);
        let mut w_range = (i64::MAX, i64::MIN);

        for (x, y, z, w) in &self.set {
            if *x < x_range.0 {
                x_range.0 = *x;
            }

            if *x > x_range.1 {
                x_range.1 = *x;
            }

            if *y < y_range.0 {
                y_range.0 = *y;
            }

            if *y > y_range.1 {
                y_range.1 = *y;
            }

            if *z < z_range.0 {
                z_range.0 = *z;
            }

            if *z > z_range.1 {
                z_range.1 = *z;
            }

            if *w < w_range.0 {
                w_range.0 = *w;
            }

            if *w > w_range.1 {
                w_range.1 = *w;
            }
        }

        (
            Extent::new(x_range.0, x_range.1 + 1),
            Extent::new(y_range.0, y_range.1 + 1),
            Extent::new(z_range.0, z_range.1 + 1),
            Extent::new(w_range.0, w_range.1 + 1),
        )
    }

    fn print(&self) {
        let (x_range, y_range, z_range, w_range) = self.get_bounds();
        for w in w_range.start..w_range.end {
            for z in z_range.start..z_range.end {
                println!("z={}, w={}", z, w);
                for y in y_range.start..y_range.end {
                    for x in x_range.start..x_range.end {
                        if self.set.contains(&(x, y, z, w)) {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
                println!();
            }
        }
    }
}

fn parse(input: String) -> State {
    let mut set = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                set.insert((x as i64, y as i64, 0, 0));
            }
        }
    }

    State { set }
}

fn neighbors((x, y, z): (i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    vec![
        (x - 1, y - 1, z - 1),
        (x, y - 1, z - 1),
        (x + 1, y - 1, z - 1),
        (x - 1, y, z - 1),
        (x, y, z - 1),
        (x + 1, y, z - 1),
        (x - 1, y + 1, z - 1),
        (x, y + 1, z - 1),
        (x + 1, y + 1, z - 1),
        (x - 1, y - 1, z),
        (x, y - 1, z),
        (x + 1, y - 1, z),
        (x - 1, y, z),
        (x + 1, y, z),
        (x - 1, y + 1, z),
        (x, y + 1, z),
        (x + 1, y + 1, z),
        (x - 1, y - 1, z + 1),
        (x, y - 1, z + 1),
        (x + 1, y - 1, z + 1),
        (x - 1, y, z + 1),
        (x, y, z + 1),
        (x + 1, y, z + 1),
        (x - 1, y + 1, z + 1),
        (x, y + 1, z + 1),
        (x + 1, y + 1, z + 1),
    ]
}

fn step(state: State) -> State {
    let mut new_state = HashSet::new();

    let (x_range, y_range, z_range, _) = state.get_bounds();

    for z in z_range.start - 1..z_range.end + 1 {
        for y in y_range.start - 1..y_range.end + 1 {
            for x in x_range.start - 1..x_range.end + 1 {
                let count = neighbors((x, y, z))
                    .iter()
                    .filter(|pos| state.set.contains(&(pos.0, pos.1, pos.2, 0)))
                    .count();

                if state.set.contains(&(x, y, z, 0)) && count == 2 || count == 3 {
                    new_state.insert((x, y, z, 0));
                }
            }
        }
    }

    // println!("{:#?}", new_state);

    State { set: new_state }
}

fn part1(state: State) {
    println!("Part 1");

    state.print();

    let mut state = state;

    for idx in 0..6 {
        // println!("After {} cycle:", idx + 1);
        state = step(state);
        // state.print();
    }

    println!("{}", state.set.len());
}

fn neighbors2((x, y, z, w): (i64, i64, i64, i64)) -> Vec<(i64, i64, i64, i64)> {
    vec![
        (x - 1, y - 1, z - 1, w - 1),
        (x, y - 1, z - 1, w - 1),
        (x + 1, y - 1, z - 1, w - 1),
        (x - 1, y, z - 1, w - 1),
        (x, y, z - 1, w - 1),
        (x + 1, y, z - 1, w - 1),
        (x - 1, y + 1, z - 1, w - 1),
        (x, y + 1, z - 1, w - 1),
        (x + 1, y + 1, z - 1, w - 1),
        (x - 1, y - 1, z, w - 1),
        (x, y - 1, z, w - 1),
        (x + 1, y - 1, z, w - 1),
        (x - 1, y, z, w - 1),
        (x, y, z, w - 1),
        (x + 1, y, z, w - 1),
        (x - 1, y + 1, z, w - 1),
        (x, y + 1, z, w - 1),
        (x + 1, y + 1, z, w - 1),
        (x - 1, y - 1, z + 1, w - 1),
        (x, y - 1, z + 1, w - 1),
        (x + 1, y - 1, z + 1, w - 1),
        (x - 1, y, z + 1, w - 1),
        (x, y, z + 1, w - 1),
        (x + 1, y, z + 1, w - 1),
        (x - 1, y + 1, z + 1, w - 1),
        (x, y + 1, z + 1, w - 1),
        (x + 1, y + 1, z + 1, w - 1),
        (x - 1, y - 1, z - 1, w),
        (x, y - 1, z - 1, w),
        (x + 1, y - 1, z - 1, w),
        (x - 1, y, z - 1, w),
        (x, y, z - 1, w),
        (x + 1, y, z - 1, w),
        (x - 1, y + 1, z - 1, w),
        (x, y + 1, z - 1, w),
        (x + 1, y + 1, z - 1, w),
        (x - 1, y - 1, z, w),
        (x, y - 1, z, w),
        (x + 1, y - 1, z, w),
        (x - 1, y, z, w),
        (x + 1, y, z, w),
        (x - 1, y + 1, z, w),
        (x, y + 1, z, w),
        (x + 1, y + 1, z, w),
        (x - 1, y - 1, z + 1, w),
        (x, y - 1, z + 1, w),
        (x + 1, y - 1, z + 1, w),
        (x - 1, y, z + 1, w),
        (x, y, z + 1, w),
        (x + 1, y, z + 1, w),
        (x - 1, y + 1, z + 1, w),
        (x, y + 1, z + 1, w),
        (x + 1, y + 1, z + 1, w),
        (x - 1, y - 1, z - 1, w + 1),
        (x, y - 1, z - 1, w + 1),
        (x + 1, y - 1, z - 1, w + 1),
        (x - 1, y, z - 1, w + 1),
        (x, y, z - 1, w + 1),
        (x + 1, y, z - 1, w + 1),
        (x - 1, y + 1, z - 1, w + 1),
        (x, y + 1, z - 1, w + 1),
        (x + 1, y + 1, z - 1, w + 1),
        (x - 1, y - 1, z, w + 1),
        (x, y - 1, z, w + 1),
        (x + 1, y - 1, z, w + 1),
        (x - 1, y, z, w + 1),
        (x, y, z, w + 1),
        (x + 1, y, z, w + 1),
        (x - 1, y + 1, z, w + 1),
        (x, y + 1, z, w + 1),
        (x + 1, y + 1, z, w + 1),
        (x - 1, y - 1, z + 1, w + 1),
        (x, y - 1, z + 1, w + 1),
        (x + 1, y - 1, z + 1, w + 1),
        (x - 1, y, z + 1, w + 1),
        (x, y, z + 1, w + 1),
        (x + 1, y, z + 1, w + 1),
        (x - 1, y + 1, z + 1, w + 1),
        (x, y + 1, z + 1, w + 1),
        (x + 1, y + 1, z + 1, w + 1),
    ]
}

fn step2(state: State) -> State {
    let mut new_state = HashSet::new();

    let (x_range, y_range, z_range, w_range) = state.get_bounds();
    for w in w_range.start - 1..w_range.end + 1 {
        for z in z_range.start - 1..z_range.end + 1 {
            for y in y_range.start - 1..y_range.end + 1 {
                for x in x_range.start - 1..x_range.end + 1 {
                    let count = neighbors2((x, y, z, w))
                        .iter()
                        .filter(|pos| state.set.contains(&(pos.0, pos.1, pos.2, pos.3)))
                        .count();

                    if state.set.contains(&(x, y, z, w)) && count == 2 || count == 3 {
                        new_state.insert((x, y, z, w));
                    }
                }
            }
        }
    }

    // println!("{:#?}", new_state);

    State { set: new_state }
}

fn part2(state: State) {
    println!("Part 2");

    state.print();

    let mut state = state;

    for idx in 0..6 {
        // println!("After {} cycle:", idx + 1);
        state = step2(state);
        // state.print();
    }

    println!("{}", state.set.len());
}
