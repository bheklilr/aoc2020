use crate::prelude::*;

pub fn day10() -> R<Answer<i32>> {
    let mut answer = Answer::new();
    let _values: Vec<i32> = parse_file("inputs/day10.txt")?;
    answer.part1(0);
    answer.part2(0);
    Ok(answer)
}
