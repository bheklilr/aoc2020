use crate::prelude::*;

type Int = usize;

pub fn day13() -> R<Answer<Int>> {
    let mut answer = Answer::new();
    let values = parse_("")?;
    answer.part1(0);
    answer.part2(0);
    Ok(answer)
}

fn parse_(input: &str) -> R<Vec<Int>> {
    Ok(vec![])
}
