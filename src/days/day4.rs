use std::fs::File;
use std::io::Read;

type Data = Vec<Passport>;

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

#[derive(Debug)]
enum Height {
    In(u32),
    Cm(u32),
}

impl Height {
    fn parse(s: &str) -> Option<Height> {
        if let Some(pos) = s.find("cm") {
            let (num, _) = s.split_at(pos);

            num.parse().ok().map(Height::Cm)
        } else {
            let split = s.split("in").collect::<Vec<_>>();

            if split.len() != 2 {
                return None;
            }

            split[0].parse().ok().map(Height::In)
        }
    }
}

fn parse(input: String) -> Data {
    let mut passports = Vec::new();
    for data in input.split("\r\n\r\n") {
        let mut passport = Passport::default();
        for fields in data.split([' ', '\n'].as_ref()) {
            let key_value: Vec<_> = fields.split(':').collect();

            match key_value[0] {
                "byr" => passport.byr = Some(key_value[1].trim().to_string()),
                "iyr" => passport.iyr = Some(key_value[1].trim().to_string()),
                "eyr" => passport.eyr = Some(key_value[1].trim().to_string()),
                "hgt" => passport.hgt = Some(key_value[1].trim().to_string()),
                "hcl" => passport.hcl = Some(key_value[1].trim().to_string()),
                "ecl" => passport.ecl = Some(key_value[1].trim().to_string()),
                "pid" => passport.pid = Some(key_value[1].trim().to_string()),
                "cid" => passport.cid = Some(key_value[1].trim().to_string()),
                _ => unreachable!(),
            }
        }

        passports.push(passport);
    }

    passports
}

fn part1(data: Data) -> Data {
    println!("Part 1");

    println!(
        "{} Valid",
        data.iter().filter(|pp| part1_is_valid(pp)).count()
    );

    data
}

fn part1_is_valid(pp: &Passport) -> bool {
    pp.byr.is_some()
        && pp.iyr.is_some()
        && pp.eyr.is_some()
        && pp.hgt.is_some()
        && pp.hcl.is_some()
        && pp.ecl.is_some()
        && pp.pid.is_some()
}

fn part2(data: Data) {
    println!("Part 2",);

    println!(
        "{} Valid",
        data.iter().filter(|pp| part2_is_valid(pp)).count()
    );
}

fn part2_is_valid(pp: &Passport) -> bool {
    if !part1_is_valid(pp) {
        return false;
    }

    let byr_valid = pp
        .byr
        .as_ref()
        .and_then(|num| num.parse::<u32>().ok())
        .filter(|byr| *byr >= 1920 && *byr <= 2002)
        .is_some();
    let iyr_valid = pp
        .iyr
        .as_ref()
        .and_then(|num| num.parse::<u32>().ok())
        .filter(|iyr| *iyr >= 2010 && *iyr <= 2020)
        .is_some();
    let eyr_valid = pp
        .eyr
        .as_ref()
        .and_then(|num| num.parse::<u32>().ok())
        .filter(|eyr| *eyr >= 2020 && *eyr <= 2030)
        .is_some();
    let hgt_valid = pp
        .hgt
        .as_ref()
        .and_then(|hgt| Height::parse(hgt))
        .filter(|hgt| match hgt {
            Height::Cm(cm) => *cm >= 150 && *cm <= 193,
            Height::In(num) => *num >= 59 && *num <= 76,
        })
        .is_some();
    let hcl_valid = pp
        .hcl
        .as_ref()
        .filter(|hcl| {
            let bytes = hcl.as_bytes();
            if hcl.len() == 7 && bytes[0] == b'#' {
                bytes
                    .iter()
                    .skip(1)
                    .filter(|byte| byte.is_ascii_hexdigit())
                    .count()
                    == 6
            } else {
                false
            }
        })
        .is_some();
    let ecl_valid = pp
        .ecl
        .as_ref()
        .filter(|ecl| {
            matches!(
                ecl.as_str(),
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            )
        })
        .is_some();
    let pid_valid = pp
        .pid
        .as_ref()
        .filter(|pid| pid.chars().filter(|c| c.is_ascii_digit()).count() == 9)
        .is_some();

    // println!(
    //     "{} {} {} {} {} {} {}",
    //     byr_valid, iyr_valid, eyr_valid, hgt_valid, hcl_valid, ecl_valid, pid_valid
    // );

    byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid && pid_valid
}

pub fn run() {
    let mut file = File::open("input/day4.txt").unwrap();

    let mut data = String::new();

    file.read_to_string(&mut data).unwrap();

    let mut data = parse(data);

    data = part1(data);
    part2(data);
}
