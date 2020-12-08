use crate::prelude::*;

pub fn day03() -> Option<Answer<usize>> {
    let mut answer = Answer::new();
    let map = parse_tree_map("inputs/day03.txt")?;
    answer.part1(trees_hit(&map, (3, 1))?);
    answer.part2(
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .filter_map(|slope| trees_hit(&map, *slope))
            .product(),
    );
    Some(answer)
}

fn trees_hit(map: &[Vec<bool>], slope: (usize, usize)) -> Option<usize> {
    let len = map.len();
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;
    while y < len {
        let row = map.get(y)?;
        let cell = *row.get(x % row.len())?;
        if cell {
            tree_count += 1;
        }
        x += slope.0;
        y += slope.1;
    }
    Some(tree_count)
}

fn parse_tree_map(filename: &str) -> Option<Vec<Vec<bool>>> {
    Some(
        read_file(filename)?
            .lines()
            .map(|line| line.bytes().map(|c| c == b'#').collect::<Vec<bool>>())
            .filter(|trees| !trees.is_empty())
            .collect(),
    )
}
