use crate::prelude::*;

type Int = usize;

pub fn day14() -> R<Answer<Int>> {
    let mut answer = Answer::new();
    let values = parse_program(&read_file("inputs/day14.txt")?);
    answer.part1(Computer::new().execute(&values));
    answer.part2(Computer::new().execute2(&values));
    Ok(answer)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BitMask {
    Zero,
    One,
    X,
}

use BitMask::*;

#[derive(Debug, Clone)]
enum Instruction {
    Mask(Vec<BitMask>),
    Assign(Int, Int),
}

use Instruction::*;

#[derive(Debug, Clone)]
struct Computer {
    mem: HashMap<Int, Int>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            mem: HashMap::new(),
        }
    }

    fn execute(&mut self, program: &[Instruction]) -> Int {
        let mut mask = vec![];
        for instr in program {
            if let Mask(new_mask) = instr {
                mask = new_mask.to_vec();
            } else if let Assign(addr, value) = instr {
                self.mem.insert(*addr, apply_mask(&mask, *value));
            }
        }
        self.mem.values().sum()
    }

    fn execute2(&mut self, program: &[Instruction]) -> Int {
        let mut mask = vec![];
        for instr in program {
            if let Mask(new_mask) = instr {
                mask = new_mask.to_vec();
            } else if let Assign(addr, value) = instr {
                for floating_addr in apply_mask_addr(&mask, *addr) {
                    self.mem.insert(floating_addr, *value);
                }
            }
        }
        self.mem.values().sum()
    }
}

fn apply_mask(mask: &[BitMask], value: Int) -> Int {
    let new_bin: String = format!("{:0>36}", format!("{:b}", value))
        .chars()
        .zip(mask)
        .map(|(v, m)| match m {
            Zero => '0',
            One => '1',
            X => v,
        })
        .collect();
    Int::from_str_radix(&new_bin, 2).unwrap()
}

fn replace_x(addrs: Vec<Vec<BitMask>>) -> Vec<Vec<BitMask>> {
    let mut new_addrs: Vec<Vec<BitMask>> = vec![];
    for addr in addrs {
        new_addrs.push(addr.iter().copied().collect());
        new_addrs.push(addr.iter().copied().collect());
    }
    let idx = new_addrs[0]
        .iter()
        .enumerate()
        .find(|(_, a)| **a == X)
        .unwrap()
        .0;
    for (i, bit) in (0..new_addrs.len()).zip(vec![Zero, One].iter().cycle()) {
        new_addrs[i][idx] = *bit;
    }

    new_addrs
}

fn apply_mask_addr(mask: &[BitMask], addr: Int) -> Vec<Int> {
    let mut addrs = vec![];
    let masked: Vec<_> = format!("{:0>36}", format!("{:b}", addr))
        .chars()
        .zip(mask)
        .map(|(v, m)| match m {
            Zero => match v {
                '0' => Zero,
                '1' => One,
                _ => panic!("Huh?"),
            },
            One => One,
            X => X,
        })
        .collect();
    addrs.push(masked);

    while addrs.iter().any(|addr| addr.contains(&X)) {
        addrs = replace_x(addrs);
    }

    addrs
        .iter()
        .map(|masked| {
            Int::from_str_radix(
                &masked
                    .iter()
                    .map(|m| match m {
                        Zero => '0',
                        One => '1',
                        X => panic!("Huh2?"),
                    })
                    .collect::<String>(),
                2,
            )
            .unwrap()
        })
        .collect()
}

fn parse_program(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line.starts_with("mask") {
                let mask = line.split(" = ").nth(1).unwrap();
                Mask(
                    mask.chars()
                        .map(|c| match c {
                            '0' => Zero,
                            '1' => One,
                            'X' => X,
                            _ => panic!("Invalid mask input"),
                        })
                        .collect(),
                )
            } else {
                let mut parts = line.split(" = ");
                let addr_part = parts.next().unwrap();
                let addr = addr_part
                    .strip_prefix("mem[")
                    .unwrap()
                    .strip_suffix("]")
                    .unwrap()
                    .parse()
                    .unwrap();
                let value = parts.next().unwrap().parse().unwrap();
                Assign(addr, value)
            }
        })
        .collect()
}

#[test]
fn test_apply_mask_addr() {
    println!("{:?}", replace_x(replace_x(replace_x(vec![vec![X, X, X]]))));
}