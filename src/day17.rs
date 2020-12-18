use crate::prelude::*;

type Int = usize;

pub fn day17() -> R<Answer<Int>> {
    let mut answer = Answer::new();
    let input = parse(&read_file("inputs/day17.txt")?);
    answer.part1(part1(&input));
    Ok(answer)
}

type XYZ = (i32, i32, i32);
type Cubes = HashMap<XYZ, bool>;

fn part1(cubes: &Cubes) -> Int {
    let space = cycle(cubes);
    let space = cycle(&space);
    let space = cycle(&space);
    let space = cycle(&space);
    let space = cycle(&space);
    let space = cycle(&space);
    space.values().filter(|b| **b).count()
}

fn cycle(cubes: &Cubes) -> Cubes {
    let mut space = HashMap::new();
    for (coords, cube) in cubes {
        let neighbors = neighbor_coords(coords);
        let mut count = 0;
        for neighbor in neighbors {
            
        }
    }
    space
}

fn neighbor_coords(coords: &XYZ) -> HashSet<XYZ> {
    let (x, y, z) = coords;
    let mut neighbors = HashSet::new();
    for a in -1..1 {
        for b in -1..1 {
            for c in -1..1 {
                neighbors.insert((x + a, y + b, z + c));
            }
        }
    }
    neighbors.remove(coords);
    neighbors
}

fn parse(text: &str) -> Cubes {
    text.lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(move |(y, c)| ((x as i32, y as i32, 0), c == '#'))
        })
        .collect()
}
