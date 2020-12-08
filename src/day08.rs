use crate::prelude::*;

pub fn day08() -> R<Answer<isize>> {
    let mut answer = Answer::new();
    let mut instructions =
        parse_instructions(&read_file("inputs/day08.txt")?)?;
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

#[derive(Copy, Clone)]
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

fn exit(code: GameboyExitCode, state: GameboyState) -> GameboyExit {
    GameboyExit { code, state }
}

use GameboyExitCode::*;
use Instruction::*;

struct Gameboy<'a> {
    instructions: &'a [Instruction],
    state: GameboyState,
}

impl<'a> Gameboy<'a> {
    fn new(instructions: &'a [Instruction]) -> Gameboy<'a> {
        Gameboy {
            state: GameboyState {
                pointer: 0,
                accumulator: 0,
            },
            instructions,
        }
    }
    fn step(&mut self) -> Option<GameboyState> {
        match self.instructions.get(self.state.pointer)? {
            JMP(arg) => self.state.pointer = (self.state.pointer as isize + arg) as usize,
            ACC(arg) => {
                self.state.accumulator += arg;
                self.state.pointer += 1;
            }
            _ => {
                self.state.pointer += 1;
            }
        }

        Some(self.state)
    }
    fn interpret_until<F>(&mut self, mut state_condition: F) -> GameboyExit
    where
        F: FnMut(GameboyState) -> bool,
    {
        while !state_condition(self.state) {
            if self.step().is_none() {
                return exit(Completed, self.state);
            }
        }
        exit(Halted, self.state)
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
