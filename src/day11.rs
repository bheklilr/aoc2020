use crate::prelude::*;
use std::cmp::*;
use std::iter::FromIterator;

type Int = i32;

pub fn day11() -> R<Answer<Int>> {
    let mut answer = Answer::new();
    let seats = parse_seats(&read_file("inputs/day11.txt")?)?;
    // print_layout(&seats);
    answer.part1(iterate_by(&seats, step1));
    answer.part2(iterate_by(&seats, step2));
    Ok(answer)
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Cell {
    Empty,
    Occupied,
    Floor,
}

fn print_layout(seats: &Layout) {
    for row in seats {
        for seat in row {
            print!(
                "{}",
                match seat {
                    Empty => "L",
                    Occupied => "#",
                    Floor => ".",
                }
            );
        }
        println!();
    }
}

use Cell::*;

type Layout = Vec<Vec<Cell>>;

fn parse_seats(input: &str) -> R<Layout> {
    Ok(input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .filter_map(|c| match c {
                    'L' => Some(Empty),
                    '#' => Some(Occupied),
                    '.' => Some(Floor),
                    _ => None,
                })
                .collect()
        })
        .collect())
}

fn step1(seats: &Layout) -> Layout {
    let mut output = Vec::with_capacity(seats.len());
    let mut all_neighbors = neighbors_for(seats);
    for i in 0..seats.len() {
        let mut row = Vec::with_capacity(seats[0].len());
        for j in 0..seats[i].len() {
            if let Some(neighbors) = all_neighbors.next() {
                let seat = seats[i][j];
                let neighbor_counts = neighbors
                    .iter()
                    .map(|(x, y)| seats[(*x) as usize][(*y) as usize])
                    .fold(HashMap::with_capacity(3), |mut map, cell| {
                        map.entry(cell).and_modify(|e| *e += 1).or_insert(1);
                        map
                    });
                if seat == Empty && *neighbor_counts.get(&Occupied).unwrap_or(&0) == 0 {
                    row.push(Occupied);
                } else if seat == Occupied && *neighbor_counts.get(&Occupied).unwrap_or(&0) >= 4 {
                    row.push(Empty);
                } else {
                    row.push(seat);
                }
            }
        }
        output.push(row);
    }
    output
}

fn step2(seats: &Layout) -> Layout {
    let mut output = Vec::with_capacity(seats.len());
    let mut all_neighbors = neighbors_for(seats);
    for i in 0..seats.len() as Int {
        let mut row = Vec::with_capacity(seats[i as usize].len());
        for j in 0..seats[i as usize].len() as Int {
            if let Some(neighbors) = all_neighbors.next() {
                let seat = seats[i as usize][j as usize];
                // diff between (i, j) and neighbor gives direction, just clip it
                let mut line_of_sight_neighbors = Vec::with_capacity(neighbors.len());
                for (nx, ny) in neighbors {
                    let mut x = nx;
                    let mut y = ny;
                    let dx = nx - i;
                    let dy = ny - j;
                    while seats[x as usize][y as usize] == Floor {
                        x = min(seats.len() as Int - 1, max(0, x + dx));
                        y = min(seats[i as usize].len() as Int - 1, max(0, y + dy));
                        if x == 0 || x == seats.len() as Int - 1 {
                            break;
                        }
                        if y == 0 || y == seats[i as usize].len() as Int - 1 {
                            break;
                        }
                    }
                    line_of_sight_neighbors.push((x, y));
                }
                let neighbor_counts = line_of_sight_neighbors
                    .iter()
                    .map(|(x, y)| seats[(*x) as usize][(*y) as usize])
                    .fold(HashMap::with_capacity(3), |mut map, cell| {
                        map.entry(cell).and_modify(|e| *e += 1).or_insert(1);
                        map
                    });
                if seat == Empty && *neighbor_counts.get(&Occupied).unwrap_or(&0) == 0 {
                    row.push(Occupied);
                } else if seat == Occupied && *neighbor_counts.get(&Occupied).unwrap_or(&0) >= 5 {
                    row.push(Empty);
                } else {
                    row.push(seat);
                }
            }
        }
        output.push(row);
    }
    output
}

fn iterate_by<F>(seats: &Layout, step_by: F) -> Int
where
    F: Fn(&Layout) -> Layout,
{
    let mut seats = seats.clone();
    let mut output = vec![];

    let mut done = seats == output;
    while !done {
        output = step_by(&seats);
        done = seats == output;
        seats = output;
    }
    seats
        .iter()
        .flat_map(|row| row.iter())
        .filter(|cell| **cell == Occupied)
        .count() as Int
}

struct Neighbors {
    rows: Int,
    cols: Int,
    cur_row: Int,
    cur_col: Int,
}

impl Iterator for Neighbors {
    type Item = HashSet<(Int, Int)>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_col == self.cols {
            self.cur_row += 1;
            self.cur_col = 0;
        }
        if self.cur_row == self.rows {
            return None;
        }

        let left = max(0, self.cur_col - 1);
        let cent = self.cur_col;
        let right = min(self.cur_col + 1, self.cols - 1);
        let top = max(0, self.cur_row - 1);
        let mid = self.cur_row;
        let bot = min(self.cur_row + 1, self.rows - 1);

        let mut neighbors = HashSet::from_iter(vec![
            (top, left),
            (top, cent),
            (top, right),
            (mid, left),
            (mid, right),
            (bot, left),
            (bot, cent),
            (bot, right),
        ]);
        neighbors.remove(&(self.cur_row, self.cur_col));

        self.cur_col += 1;

        Some(neighbors)
    }
}

fn neighbors_for(seats: &Layout) -> Neighbors {
    Neighbors {
        rows: seats.len() as Int,
        cols: seats[0].len() as Int,
        cur_row: 0,
        cur_col: 0,
    }
}

#[test]
fn test_neighbors_for() {
    let seats = vec![
        vec![Empty, Empty, Empty],
        vec![Empty, Empty, Empty],
        vec![Empty, Empty, Empty],
    ];
    let neighbors: Vec<_> = neighbors_for(&seats).collect();
    assert_eq!(neighbors.len(), 9);
    assert_eq!(
        neighbors[0],
        HashSet::from_iter(vec![(0, 1), (1, 1), (1, 0)])
    );
    assert_eq!(
        neighbors[1],
        HashSet::from_iter(vec![(0, 0), (0, 2), (1, 0), (1, 1), (1, 2)])
    );
    assert_eq!(
        neighbors[2],
        HashSet::from_iter(vec![(0, 1), (1, 1), (1, 2)])
    );
    assert_eq!(
        neighbors[3],
        HashSet::from_iter(vec![(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)])
    );
    assert_eq!(
        neighbors[4],
        HashSet::from_iter(vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2)
        ])
    );
    assert_eq!(
        neighbors[5],
        HashSet::from_iter(vec![(0, 1), (0, 2), (1, 1), (2, 1), (2, 2)])
    );
    assert_eq!(
        neighbors[6],
        HashSet::from_iter(vec![(1, 0), (1, 1), (2, 1)])
    );
    assert_eq!(
        neighbors[7],
        HashSet::from_iter(vec![(1, 0), (1, 1), (1, 2), (2, 0), (2, 2)])
    );
    assert_eq!(
        neighbors[8],
        HashSet::from_iter(vec![(1, 1), (1, 2), (2, 1)])
    );
}
