#![allow(unused)]

mod concurrency;
mod more_semantics;
mod restaurant;
mod rust_book;
mod rust_semantics;

use rand::Rng;
use std::cmp::Ordering;
use std::collections::{hash_map, HashMap};
use std::error::Error;
use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::{Add, Deref};
use std::{io, string};

use crate::more_semantics::error_handling;
use crate::restaurant::{order_food, pizza_order};
use crate::rust_book::guessing_game_example;

fn main() -> Result<(), Box<dyn Error>> {
    
    Ok(())
}
