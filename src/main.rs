#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate lazy_static;

mod day18;
mod prelude;
use crate::prelude::*;
use day18::*;

fn main() -> R<()> {
    day18()?.tell();
    Ok(())
}
