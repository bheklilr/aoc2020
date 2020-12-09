#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate lazy_static;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod prelude;
use crate::prelude::*;
use day01::*;
use day02::*;
use day03::*;
use day04::*;
use day05::*;
use day06::*;
use day07::*;
use day08::*;
use day09::*;
use day10::*;

fn main() -> R<()> {
    day09()?.tell();
    Ok(())
}
