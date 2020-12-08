use crate::prelude::*;

pub fn day01() -> Option<Answer<i32>> {
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
