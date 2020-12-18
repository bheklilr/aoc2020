use crate::prelude::*;

type Int = usize;
type Rule = Vec<(Int, Int)>;

struct Rules {
    dep_locaction: Rule,
    dep_station: Rule,
    dep_platform: Rule,
    dep_track: Rule,
    dep_date: Rule,
    dep_time: Rule,
    arrival_location: Rule,
    arrival_station: Rule,
    arrival_platform: Rule,
    arrival_track: Rule,
    class: Rule,
    duration: Rule,
    price: Rule,
    route: Rule,
    row: Rule,
    seat: Rule,
    train: Rule,
    typ: Rule,
    wagon: Rule,
    zone: Rule,
}

fn rules() -> Rules {
    Rules {
        dep_locaction: vec![(34, 269), (286, 964)],
        dep_station: vec![(27, 584), (609, 973)],
        dep_platform: vec![(49, 135), (155, 974)],
        dep_track: vec![(36, 248), (255, 954)],
        dep_date: vec![(50, 373), (381, 974)],
        dep_time: vec![(49, 454), (472, 967)],
        arrival_location: vec![(33, 900), (925, 968)],
        arrival_station: vec![(46, 699), (706, 965)],
        arrival_platform: vec![(42, 656), (666, 967)],
        arrival_track: vec![(49, 408), (425, 950)],
        class: vec![(30, 626), (651, 957)],
        duration: vec![(43, 109), (127, 964)],
        price: vec![(33, 778), (795, 952)],
        route: vec![(37, 296), (315, 966)],
        row: vec![(28, 318), (342, 965)],
        seat: vec![(33, 189), (208, 959)],
        train: vec![(49, 536), (552, 968)],
        typ: vec![(46, 749), (772, 949)],
        wagon: vec![(29, 386), (401, 954)],
        zone: vec![(34, 344), (368, 954)],
    }
}

type Ticket = Vec<Int>;

fn my_ticket() -> Ticket {
    vec![
        109, 101, 79, 127, 71, 59, 67, 61, 173, 157, 163, 103, 83, 97, 73, 167, 53, 107, 89, 131,
    ]
}

pub fn day16() -> R<Answer<Int>> {
    let mut answer = Answer::new();
    let tickets = parse(&read_file("inputs/day16.txt")?)?;
    answer.part1(part1(&tickets));
    answer.part2(part2(&tickets));
    Ok(answer)
}

fn matches(rule: &Rule, num: Int) -> bool {
    rule.iter()
        .any(|(start, stop)| *start <= num && num <= *stop)
}

fn matches_any(num: Int) -> bool {
    let rules = rules();
    matches(&rules.dep_locaction, num)
        || matches(&rules.dep_station, num)
        || matches(&rules.dep_platform, num)
        || matches(&rules.dep_track, num)
        || matches(&rules.dep_date, num)
        || matches(&rules.dep_time, num)
        || matches(&rules.arrival_location, num)
        || matches(&rules.arrival_station, num)
        || matches(&rules.arrival_platform, num)
        || matches(&rules.arrival_track, num)
        || matches(&rules.class, num)
        || matches(&rules.duration, num)
        || matches(&rules.price, num)
        || matches(&rules.route, num)
        || matches(&rules.row, num)
        || matches(&rules.seat, num)
        || matches(&rules.train, num)
        || matches(&rules.typ, num)
        || matches(&rules.wagon, num)
        || matches(&rules.zone, num)
}

fn matchings(column: Vec<Int>) -> Vec<bool> {
    let rules = rules();
    vec![
        column.iter().all(|num| matches(&rules.dep_locaction, *num)),
        column.iter().all(|num| matches(&rules.dep_station, *num)),
        column.iter().all(|num| matches(&rules.dep_platform, *num)),
        column.iter().all(|num| matches(&rules.dep_track, *num)),
        column.iter().all(|num| matches(&rules.dep_date, *num)),
        column.iter().all(|num| matches(&rules.dep_time, *num)),
        column.iter().all(|num| matches(&rules.arrival_location, *num)),
        column.iter().all(|num| matches(&rules.arrival_station, *num)),
        column.iter().all(|num| matches(&rules.arrival_platform, *num)),
        column.iter().all(|num| matches(&rules.arrival_track, *num)),
        column.iter().all(|num| matches(&rules.class, *num)),
        column.iter().all(|num| matches(&rules.duration, *num)),
        column.iter().all(|num| matches(&rules.price, *num)),
        column.iter().all(|num| matches(&rules.route, *num)),
        column.iter().all(|num| matches(&rules.row, *num)),
        column.iter().all(|num| matches(&rules.seat, *num)),
        column.iter().all(|num| matches(&rules.train, *num)),
        column.iter().all(|num| matches(&rules.typ, *num)),
        column.iter().all(|num| matches(&rules.wagon, *num)),
        column.iter().all(|num| matches(&rules.zone, *num)),
    ]
}

fn part1(tickets: &Vec<Ticket>) -> Int {
    tickets
        .iter()
        .flat_map(|ticket| ticket.iter())
        .filter(|num| !matches_any(**num))
        .sum()
}

fn part2(tickets: &Vec<Ticket>) -> Int {
    let valid_tickets: Vec<Ticket> = tickets
        .iter()
        .filter(|ticket| ticket.iter().all(|num| matches_any(*num)))
        .map(|vec| vec.to_vec())
        .collect();
    let mut transposed = Vec::with_capacity(valid_tickets.len() * valid_tickets[0].len());
    for col in 0..valid_tickets[0].len() {
        let mut r = Vec::with_capacity(valid_tickets.len());
        for row in &valid_tickets {
            r.push(row[col]);
        }
        transposed.push(r);
    }
    let mut matching_values: Vec<_> = transposed.iter().map(|column| matchings(column.to_vec())).collect();

    let my = my_ticket();
    my[1] * my[2] * my[4] * my[6] * my[14] * my[19]
}

fn parse(input: &str) -> R<Vec<Ticket>> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().map_err(|e| format!("{}", e)))
                .collect()
        })
        .collect()
}
