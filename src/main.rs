#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

type R<T> = Result<T, String>;

fn main() -> R<()> {
    day08()?.tell();
    Ok(())
}

fn day08() -> R<Answer<isize>> {
    let mut answer = Answer::new();
    let mut instructions =
        parse_instructions(&read_file("inputs/day08.txt").ok_or("Failed to read file")?)?;
    let mut gameboy = Gameboy::new(&instructions);
    answer.part1(gameboy.interpret_until_looped().state.accumulator);

    for i in 0..instructions.len() {
        match instructions[i] {
            JMP(x) => instructions[i] = NOP(x),
            NOP(x) => instructions[i] = JMP(x),
            _ => {}
        }

        let mut gameboy = Gameboy::new(&instructions);
        let result = gameboy.interpret_until_looped();
        if let Completed = result.code {
            answer.part2(result.state.accumulator);
            break;
        }

        match instructions[i] {
            JMP(x) => instructions[i] = NOP(x),
            NOP(x) => instructions[i] = JMP(x),
            _ => {}
        }
    }
    Ok(answer)
}

enum Instruction {
    JMP(isize),
    ACC(isize),
    NOP(isize),
}

struct GameboyState {
    pointer: usize,
    accumulator: isize,
}

enum GameboyExitCode {
    Halted,
    Completed,
}

struct GameboyExit {
    code: GameboyExitCode,
    state: GameboyState,
}

fn gb_exit(code: GameboyExitCode, state: GameboyState) -> GameboyExit {
    GameboyExit { code, state }
}
fn gb_halted(state: GameboyState) -> GameboyExit {
    gb_exit(Halted, state)
}
fn gb_completed(state: GameboyState) -> GameboyExit {
    gb_exit(Completed, state)
}

use GameboyExitCode::*;
use Instruction::*;

struct Gameboy<'a> {
    instructions: &'a [Instruction],
    pointer: usize,
    accumulator: isize,
}

impl<'a> Gameboy<'a> {
    fn new(instructions: &'a [Instruction]) -> Gameboy<'a> {
        Gameboy {
            pointer: 0,
            accumulator: 0,
            instructions,
        }
    }
    fn state(&self) -> GameboyState {
        GameboyState {
            pointer: self.pointer,
            accumulator: self.accumulator,
        }
    }
    fn step(&mut self) -> Option<GameboyState> {
        match self.instructions.get(self.pointer)? {
            JMP(arg) => self.pointer = (self.pointer as isize + arg) as usize,
            ACC(arg) => {
                self.accumulator += arg;
                self.pointer += 1;
            }
            _ => {
                self.pointer += 1;
            }
        }

        Some(self.state())
    }
    fn interpret_until<F>(&mut self, mut state_condition: F) -> GameboyExit
    where
        F: FnMut(GameboyState) -> bool,
    {
        while !state_condition(self.state()) {
            if self.step().is_none() {
                return gb_completed(self.state());
            }
        }
        gb_halted(self.state())
    }
    fn interpret_until_looped(&mut self) -> GameboyExit {
        let mut visited = HashSet::new();
        self.interpret_until(|state| !visited.insert(state.pointer))
    }
}

fn parse_instructions(input: &str) -> R<Vec<Instruction>> {
    let mut instructions = Vec::new();
    for line in input.trim().lines() {
        let mut parts = line.split_whitespace();
        let instr = parts.next().ok_or("No instruction found")?;
        let arg = parts.next().ok_or("No argument found")?;
        match instr {
            "jmp" => instructions.push(JMP(arg
                .parse()
                .map_err(|_| format!("Invalid argument: {} {}", instr, arg))?)),
            "acc" => instructions.push(ACC(arg
                .parse()
                .map_err(|_| format!("Invalid argument: {} {}", instr, arg))?)),
            "nop" => instructions.push(NOP(arg
                .parse()
                .map_err(|_| format!("Invalid argument: {} {}", instr, arg))?)),
            _ => {}
        }
    }
    Ok(instructions)
}

fn day07() -> Result<Answer<usize>, String> {
    let mut answer = Answer::new();
    let rules = parse_bag_rules(&read_file("inputs/day07.txt").ok_or("Failed to read file")?)?;
    answer.part1(
        rules
            .keys()
            .filter(|color| can_contain_shiny_gold(&rules, color).unwrap_or(false))
            .count(),
    );
    answer.part2(bag_count(&rules, "shiny gold").unwrap() - 1);
    Ok(answer)
}

struct BagRule {
    pub count: usize,
    pub dependency: String,
}
type BagRules = HashMap<String, Vec<BagRule>>;

fn bag_count(rules: &BagRules, color: &str) -> Option<usize> {
    let contents = rules.get(color)?;
    let count: usize = contents
        .iter()
        .filter_map(|rule| bag_count(rules, &rule.dependency).map(|x| rule.count * x))
        .sum();
    Some(1 + count)
}

fn can_contain_shiny_gold(rules: &BagRules, color: &str) -> Option<bool> {
    let contents = rules.get(color)?;
    Some(
        contents.iter().map(|rule| &rule.dependency).any(|dep| {
            *dep == *"shiny gold" || can_contain_shiny_gold(rules, &dep).unwrap_or(false)
        }),
    )
}

fn parse_bag_rules(file: &str) -> Result<BagRules, String> {
    let mut parsed = HashMap::new();
    for line in file.lines() {
        let mut contains = Vec::new();
        let mut split = line.split(" bags contain ");
        let color = split.next().ok_or("No initial color")?;
        let rules = split.next().ok_or("No dependency colors")?;
        for rule in rules.split(',') {
            let rule = rule
                .trim()
                .replace("bags", "")
                .replace("bag", "")
                .replace('.', "");
            let rule = rule.trim();
            if !(rule.starts_with("no other")) {
                let count = rule[..1]
                    .parse()
                    .map_err(|_| format!("Failed to parse number: {:?}", rule))?;
                let dependency = rule[1..].trim().to_string();
                contains.push(BagRule { count, dependency });
            }
        }
        parsed.insert(color.to_string(), contains);
    }
    Ok(parsed)
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
                unique_answers
                    .iter()
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
