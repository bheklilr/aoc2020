use crate::prelude::*;

pub fn day09(filename: &str) -> R<Answer<usize>> {
    let mut answer = Answer::new();
    let values: Vec<usize> = parse_file(filename)?;
    let part1 = part1(&values);
    answer.part1(part1);
    answer.part2(part2(&values, part1));
    Ok(answer)
}

fn part1(values: &[usize]) -> usize {
    let mut preamble: Vec<usize> = values[..25].into();
    for i in values.iter().skip(25) {
        let pair = preamble
            .iter()
            .flat_map(|x| preamble.iter().map(move |y| (*x, *y)))
            .filter(|(x, y)| x != y)
            .find(|(x, y)| x + y == *i);
        if pair.is_some() {
            preamble.push(*i);
            preamble = preamble[1..].into();
        } else {
            return *i;
        }
    }
    0
}

fn part2(values: &[usize], target: usize) -> usize {
    for i in 2..values.len() - 1 {
        for window in values.windows(i) {
            if window.iter().sum::<usize>() == target {
                return window.iter().min().unwrap() + window.iter().max().unwrap();
            }
        }
    }
    0
}
