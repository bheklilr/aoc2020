#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
// use chrono::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    // let days: Vec<fn() -> Option<Answer<i32>>> = vec![
    //     day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    // ];
    // let local = Local::now();
    // let answer = days.get(local.day() as usize).map(|f| f()).flatten();
    if let Some(a) = day06() {
        a.tell()
    } else {
        println!("Failed to calculate");
    }
    Ok(())
}

fn day06() -> Option<Answer<usize>> {
    let mut answer = Answer::new();
    let values = parse_customs_answers("inputs/day06.txt")?;
    answer.part1(values.iter().map(|answers| answers.len()).sum());
    let values = parse_customs_answers2("inputs/day06.txt")?;
    answer.part2(
        values
            .iter()
            .map(|group| {
                let mut unique_answers: HashMap<&char, usize> = HashMap::new();
                for person in group {
                    for answer in person {
                        unique_answers
                            .entry(answer)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                }
                let people = group.len();
                unique_answers.iter()
                    .filter(|(_ans, count)| **count == people)
                    .count()
            })
            .sum(),
    );
    Some(answer)
}

fn parse_customs_answers2(filename: &str) -> Option<Vec<Vec<Vec<char>>>> {
    Some(
        read_file(filename)?
            .replace('\r', "")
            .split("\n\n")
            .map(|chunk| chunk.lines().map(|line| line.chars().collect()).collect())
            .collect(),
    )
}

fn parse_customs_answers(filename: &str) -> Option<Vec<HashSet<char>>> {
    Some(
        read_file(filename)?
            .replace('\r', "")
            .split("\n\n")
            .map(|chunk| chunk.replace('\n', "").chars().collect())
            .collect(),
    )
}

fn day05() -> Option<Answer<u32>> {
    let mut answer = Answer::new();
    let values = parse_seats("inputs/day05.txt")?;
    answer.part1(*values.iter().max()?);
    for id in 1..1024 {
        let id_down = id - 1;
        let id_up = id + 1;
        if !values.contains(&id) && values.contains(&id_up) && values.contains(&id_down) {
            answer.part2(id);
            break;
        }
    }
    Some(answer)
}

fn parse_seats(filename: &str) -> Option<Vec<u32>> {
    Some(
        read_file(filename)?
            .trim()
            .lines()
            .map(|line| {
                let line_bits = line
                    .trim()
                    .replace('F', "0")
                    .replace('B', "1")
                    .replace('L', "0")
                    .replace('R', "1");
                u32::from_str_radix(&line_bits, 2).unwrap()
            })
            .collect(),
    )
}

fn day04() -> Option<Answer<usize>> {
    use Field::*;
    let mut answer = Answer::new();
    let passports = parse_passports("inputs/day04.txt")?;
    answer.part1(
        passports
            .iter()
            .filter(|passport| {
                [BYR, IYR, EYR, HGT, HCL, ECL, PID]
                    .iter()
                    .all(|field| passport.contains_key(&field))
            })
            .count(),
    );
    answer.part2(
        passports
            .iter()
            .filter_map(|passport| {
                let byr = passport.get(&BYR)?.parse::<u32>().ok()?;
                let iyr = passport.get(&IYR)?.parse::<u32>().ok()?;
                let eyr = passport.get(&EYR)?.parse::<u32>().ok()?;
                let hgt = passport.get(&HGT)?;
                let hcl = passport.get(&HCL)?;
                let ecl = passport.get(&ECL)?;
                let pid = passport.get(&PID)?;
                if (byr < 1920) || (2002 < byr) {
                    println!("byr: {:?}", passport);
                    return None;
                }
                if (iyr < 2010) || (2020 < iyr) {
                    println!("iyr: {:?}", passport);
                    return None;
                }
                if (eyr < 2020) || (2030 < eyr) {
                    println!("eyr: {:?}", passport);
                    return None;
                }
                if hgt.ends_with("cm") {
                    let height = hgt.strip_suffix("cm")?.parse::<u32>().ok()?;
                    if (height < 150) || (193 < height) {
                        println!("hgt: {:?}", passport);
                        return None;
                    }
                } else if hgt.ends_with("in") {
                    let height = hgt.strip_suffix("in")?.parse::<u32>().ok()?;
                    if (height < 59) || (76 < height) {
                        println!("hgt: {:?}", passport);
                        return None;
                    }
                } else {
                    println!("hgt: {:?}", passport);
                    return None;
                }
                if !hcl.strip_prefix('#')?.chars().all(|c| c.is_digit(16)) {
                    println!("hcl: {:?}", passport);
                    return None;
                }
                match &ecl[..] {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Some(()),
                    _ => None,
                }?;
                if (pid.len() != 9) || (!pid.chars().all(char::is_numeric)) {
                    println!("pid: {:?}", passport);
                    return None;
                }
                Some(passport)
            })
            .count(),
    );
    Some(answer)
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Field {
    BYR,
    IYR,
    EYR,
    HGT,
    HCL,
    ECL,
    PID,
    CID,
}

type Passport = HashMap<Field, String>;

fn parse_passports(filename: &str) -> Option<Vec<Passport>> {
    use Field::*;
    let text = read_file(filename)?;
    let chunks: Vec<&str> = text.split("\r\n\r\n").collect();
    Some(
        chunks
            .iter()
            .map(|input| {
                input
                    .split_whitespace()
                    .filter_map(|field| {
                        let parts: Vec<&str> = field.split(':').collect();
                        let field = match parts[0] {
                            "byr" => Some(BYR),
                            "iyr" => Some(IYR),
                            "eyr" => Some(EYR),
                            "hgt" => Some(HGT),
                            "hcl" => Some(HCL),
                            "ecl" => Some(ECL),
                            "pid" => Some(PID),
                            "cid" => Some(CID),
                            _ => None,
                        }?;
                        Some((field, parts[1].trim().to_owned()))
                    })
                    .collect()
            })
            .collect(),
    )
}

fn day03() -> Option<Answer<usize>> {
    let mut answer = Answer::new();
    let map = parse_tree_map("inputs/day03.txt")?;
    answer.part1(trees_hit(&map, (3, 1))?);
    answer.part2(
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .filter_map(|slope| trees_hit(&map, *slope))
            .product(),
    );
    Some(answer)
}

fn trees_hit(map: &[Vec<bool>], slope: (usize, usize)) -> Option<usize> {
    let len = map.len();
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;
    while y < len {
        let row = map.get(y)?;
        let cell = *row.get(x % row.len())?;
        if cell {
            tree_count += 1;
        }
        x += slope.0;
        y += slope.1;
    }
    Some(tree_count)
}

fn parse_tree_map(filename: &str) -> Option<Vec<Vec<bool>>> {
    Some(
        read_file(filename)?
            .lines()
            .map(|line| line.bytes().map(|c| c == b'#').collect::<Vec<bool>>())
            .filter(|trees| !trees.is_empty())
            .collect(),
    )
}

fn day02() -> Option<Answer<i32>> {
    let mut answer = Answer::new();
    let values: &Vec<PasswordPolicy> = &parse_passwords("inputs/day02.txt")?;
    answer.part1(
        values
            .iter()
            .filter(|pp| pp.is_valid_sled_rental_pw())
            .count() as i32,
    );

    answer.part2(values.iter().filter(|pp| pp.is_valid_toboggan_pw()).count() as i32);

    Some(answer)
}

#[derive(Debug)]
struct PasswordPolicy {
    pub min: i32,
    pub max: i32,
    pub letter: char,
    pub password: String,
}

impl PasswordPolicy {
    pub fn is_valid_sled_rental_pw(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count() as i32;
        (self.min <= count) && (count <= self.max)
    }

    pub fn is_valid_toboggan_pw(&self) -> bool {
        let first = self
            .password
            .chars()
            .nth(self.min as usize - 1)
            .unwrap_or('.');
        let second = self
            .password
            .chars()
            .nth(self.max as usize - 1)
            .unwrap_or('.');
        println!("{:?}: {} {}", self, first, second);
        (first == self.letter) ^ (second == self.letter)
    }
}

impl FromStr for PasswordPolicy {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PASSWORD_POLICY_RE: Regex = Regex::new(r"(.+)-(.+) (.): (.+)").unwrap();
        }
        let captures = PASSWORD_POLICY_RE
            .captures(input)
            .ok_or(format!("Invalid password policy: {}", &input))?;
        let min = captures[1]
            .parse()
            .map_err(|_| format!("Failed to parse min: {}", &captures[0]))?;
        let max = captures[2].parse().or(Err("Failed to parse max"))?;
        let letter = captures[3].chars().next().ok_or("Failed to parse letter")?;
        let password = (&captures[4]).to_string();
        Ok(PasswordPolicy {
            min,
            max,
            letter,
            password,
        })
    }
}

fn day01() -> Option<Answer<i32>> {
    let mut answer = Answer::new();
    let values: Vec<i32> = parse_file("inputs/day01.txt")?;
    let len: usize = values.len();
    for i in 0..len {
        for j in i..len {
            if (values[i] + values[j]) == 2020 {
                answer.part1(values[i] * values[j]);
            } else {
                for k in j..len {
                    if (values[i] + values[j] + values[k]) == 2020 {
                        answer.part2(values[i] * values[j] * values[k]);
                        break;
                    }
                }
            }
        }
    }
    Some(answer)
}

fn read_file(filename: &str) -> Option<String> {
    let mut file = File::open(filename).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;

    Some(contents)
}

fn parse_file<T: FromStr>(filename: &str) -> Option<Vec<T>> {
    Some(
        read_file(filename)?
            .split_whitespace()
            .filter_map(|line| line.parse::<T>().ok())
            .collect(),
    )
}

fn parse_passwords(filename: &str) -> Option<Vec<PasswordPolicy>> {
    Some(
        read_file(filename)?
            .lines()
            .filter_map(|line| line.parse::<PasswordPolicy>().ok())
            .collect(),
    )
}

struct Answer<T> {
    _part1: Option<T>,
    _part2: Option<T>,
}

impl<T: Copy + Default + Display> Answer<T> {
    fn new() -> Answer<T> {
        Answer {
            _part1: None,
            _part2: None,
        }
    }

    fn part1(&mut self, p1: T) {
        self._part1 = Some(p1);
    }

    fn part2(&mut self, p2: T) {
        self._part2 = Some(p2);
    }

    fn tell(&self) {
        print!("Part 1: ");
        println!("{}", self._part1.unwrap_or_default());
        print!("Part 2: ");
        println!("{}", self._part2.unwrap_or_default());
    }
}

fn day07() -> Option<Answer<i32>> {
    let mut answer = Answer::new();
    let values: Vec<i32> = parse_file("inputs/day07.txt")?;
    Some(answer)
}

fn day08() -> Option<Answer<i32>> {
    let mut answer = Answer::new();
    let values: Vec<i32> = parse_file("inputs/day08.txt")?;
    Some(answer)
}

fn day09() -> Option<Answer<i32>> {
    let mut answer = Answer::new();
    let values: Vec<i32> = parse_file("inputs/day09.txt")?;
    Some(answer)
}

fn day10() -> Option<Answer<i32>> {
    let mut answer = Answer::new();
    let values: Vec<i32> = parse_file("inputs/day10.txt")?;
    Some(answer)
}
