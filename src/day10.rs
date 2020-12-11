use crate::prelude::*;
use petgraph::visit;

pub fn day10() -> R<Answer<i64>> {
    let mut answer = Answer::new();
    let mut values: Vec<i64> = parse_file("inputs/day10.txt")?;
    values.sort_unstable();
    values.insert(0, 0);
    values.push(values[values.len() - 1] + 3);

    answer.part1(part1(&values));
    answer.part2(part2(&values));
    Ok(answer)
}

fn part1(values: &[i64]) -> i64 {
    let map = values
        .iter()
        .zip(values.iter().skip(1))
        .map(|(x, y)| y - x)
        .fold(HashMap::new(), |mut map, x| {
            map.entry(x).and_modify(|e| *e += 1).or_insert(1);
            map
        });
    map[&1] * map[&3]
}

fn part2(values: &[i64]) -> i64 {
    let mut adapters = values.iter().collect::<Vec<_>>();
    adapters.reverse();
    let mut total = 1;
    let mut optionals = vec![];
    for i in 1..adapters.len() - 1 {
        if (adapters[i - 1] - adapters[i + 1]).abs() <= 3 {
            optionals.push(*adapters[i]);
        }
    }
    for i in 0..optionals.len() {
        let opt = optionals[i];
        if optionals.contains(&(opt + 1)) && optionals.contains(&(opt + 2)) {
            total += 3 * total / 4;
        } else {
            total += total;
        }
    }
    total
}
