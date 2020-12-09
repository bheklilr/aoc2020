use crate::prelude::*;

pub fn day09() -> R<Answer<usize>> {
    let mut answer = Answer::new();
    let values: Vec<usize> = parse_file("inputs/day09.txt")?;
    let part1 = part1(&values);
    answer.part1(part1);
    answer.part2(part2(&values, part1));
    Ok(answer)
}

fn part1(values: &[usize]) -> usize {
    for (preamble, i) in values.windows(25).zip(values.iter().skip(25)) {
        let pair = preamble
            .iter()
            .flat_map(|x| preamble.iter().map(move |y| (*x, *y)))
            .filter(|(x, y)| x != y)
            .find(|(x, y)| x + y == *i);
        if pair.is_none() {
            return *i;
        }
    }
    0
}

fn part2(values: &[usize], target: usize) -> usize {
    (2..values.len() - 1)
        .flat_map(|size| values.windows(size))
        .find(|window| window.iter().sum::<usize>() == target)
        .map(|window| window.iter().min().unwrap() + window.iter().max().unwrap())
        .unwrap_or(0)
}
