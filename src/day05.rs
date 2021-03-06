use crate::prelude::*;

pub fn day05() -> R<Answer<u32>> {
    let mut answer = Answer::new();
    let values = parse_seats("inputs/day05.txt")?;
    answer.part1(*values.iter().max().ok_or("Seats vec is empty")?);
    for id in 1..1024 {
        let id_down = id - 1;
        let id_up = id + 1;
        if !values.contains(&id) && values.contains(&id_up) && values.contains(&id_down) {
            answer.part2(id);
            break;
        }
    }
    Ok(answer)
}

fn parse_seats(filename: &str) -> R<Vec<u32>> {
    Ok(read_file(filename)?
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
        .collect())
}
