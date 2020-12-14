use crate::prelude::*;
use rayon::prelude::*;

type Int = usize;

pub fn day13() -> R<Answer<Int>> {
    let mut answer = Answer::new();
    let schedule = parse_schedule(&read_file("inputs/day13.txt")?)?;
    let (min_bus_id, earliest) = schedule
        .bus_ids
        .iter()
        .map(|id| {
            let earliest = *id * (1 + schedule.start / *id);
            (*id, earliest)
        })
        .min_by_key(|(_, earliest)| earliest - schedule.start)
        .unwrap_or((0, 0));
    answer.part1(min_bus_id * (earliest - schedule.start));
    answer.part2(part2(schedule.constraints));
    Ok(answer)
}

struct BusSchedule {
    start: Int,
    bus_ids: Vec<Int>,
    constraints: Vec<Option<Int>>,
}

fn parse_schedule(input: &str) -> R<BusSchedule> {
    let mut lines = input.lines();
    let start = lines
        .next()
        .ok_or("No start")?
        .parse()
        .map_err(|e| format!("Failed to parse, {}", e))?;
    let schedule = lines.next().ok_or("No schedule")?;
    let bus_ids = schedule
        .split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse().map_err(|e| format!("Failed to parse, {}", e)))
        .collect::<R<_>>()?;
    let constraints = schedule
        .split(',')
        .map(|s| {
            if s == "x" {
                None
            } else {
                Some(s.parse().unwrap())
            }
        })
        .collect();
    Ok(BusSchedule {
        start,
        bus_ids,
        constraints,
    })
}

fn part2(constraints: Vec<Option<Int>>) -> Int {
    let ids_with_offsets: Vec<_> = constraints
        .iter()
        .enumerate()
        .filter_map(|(c, ox)| {
            if let Some(x) = ox {
                Some((c, *x))
            } else {
                None
            }
        })
        .collect();
    let mut solution = 0;
    let mut lcd = 1;
    for (c, x) in ids_with_offsets {
        while (solution + c) % x != 0 {
            solution += lcd;
        }
        lcd *= x;
    }
    solution
}
