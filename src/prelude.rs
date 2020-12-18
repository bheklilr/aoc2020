pub use regex::Regex;
pub use petgraph::Graph;
pub use petgraph::graph::NodeIndex;
pub use petgraph::algo;
pub use std::collections::HashMap;
pub use std::collections::HashSet;
pub use std::iter::FromIterator;
pub use std::env;
pub use std::fmt::Debug;
pub use std::fmt::Display;
pub use std::fs::File;
pub use std::io::prelude::*;
pub use std::str::FromStr;
pub use rayon::prelude::*;

pub type R<T> = Result<T, String>;

pub fn read_file(filename: &str) -> R<String> {
    let mut file = File::open(filename).map_err(|e| format!("Failed to open file {}: {}", filename, e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;
    Ok(contents)
}

pub fn parse_file<T: FromStr>(filename: &str) -> R<Vec<T>> {
    Ok(read_file(filename)?
        .split_whitespace()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub struct Answer<T> {
    _part1: Option<T>,
    _part2: Option<T>,
}

impl<T: Copy + Default + Display> Answer<T> {
    pub fn new() -> Answer<T> {
        Answer {
            _part1: None,
            _part2: None,
        }
    }

    pub fn part1(&mut self, p1: T) {
        self._part1 = Some(p1);
    }

    pub fn part2(&mut self, p2: T) {
        self._part2 = Some(p2);
    }

    pub fn tell(&self) {
        print!("Part 1: ");
        println!("{}", self._part1.unwrap_or_default());
        print!("Part 2: ");
        println!("{}", self._part2.unwrap_or_default());
    }
}
