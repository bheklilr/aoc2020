pub use regex::Regex;
pub use std::collections::HashMap;
pub use std::collections::HashSet;
pub use std::fmt::Debug;
pub use std::fmt::Display;
pub use std::fs::File;
pub use std::io::prelude::*;
pub use std::str::FromStr;

pub type R<T> = Result<T, String>;

pub fn read_file(filename: &str) -> R<String> {
    let mut file = File::open(filename).map_err(|_| "Failed to open file")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| "Failed to read file")?;

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
