use crate::prelude::*;
use std::collections::VecDeque;

type Int = usize;

pub fn day15() -> R<Answer<Int>> {
    let mut answer = Answer::new();
    let input: Vec<Int> = vec![5, 1, 9, 18, 13, 8, 0];
    answer.part1(memory_game(&input, 2020));
    answer.part2(memory_game(&input, 30000000));
    Ok(answer)
}

#[derive(Debug, PartialEq, Eq)]
struct ShiftVec<T> {
    size: usize,
    items: Box<VecDeque<T>>,
}

impl<'a, T> ShiftVec<T> {
    pub fn new(size: usize) -> ShiftVec<T> {
        ShiftVec {
            size,
            items: Box::new(VecDeque::with_capacity(size)),
        }
    }

    pub fn push(&mut self, value: T) -> &Self {
        self.items.push_front(value);
        while self.items.len() > 2 {
            self.items.pop_back();
        }
        self
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.items.get(idx)
    }
}

impl<T> IntoIterator for ShiftVec<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

fn memory_game(inputs: &[Int], limit: Int) -> Int {
    let mut turns = HashMap::new();
    let mut last_turn = 0;
    for (i, num) in inputs.iter().enumerate() {
        turns.insert(*num, ShiftVec::new(2));
        let turns: &mut ShiftVec<Int> = turns.get(num).unwrap();
        turns.push(i + 1);
        last_turn = *num;
        println!("{}", last_turn);
    }

    for i in inputs.len()..limit {
        if let Some(last_two_turns) = turns.get(&last_turn) {
            if let Some(second) = last_two_turns.get(1) {
                last_turn = last_two_turns.get(0).unwrap() - second;
            } else {
                last_turn = 0;
            }
        } else {
            last_turn = 0;
        }
        println!("{}", last_turn);
        turns.insert(last_turn, ShiftVec::new(2));
        turns.get(&last_turn).unwrap().push(i + 1);
    }
    println!();
    last_turn
}

#[test]
fn test() {
    assert_eq!(memory_game(&[0, 3, 6], 10), 0);
    // assert_eq!(memory_game(&[5, 1, 9, 18, 13, 8, 0], 2020), 376);
}
