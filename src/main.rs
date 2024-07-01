#![allow(unused)]

mod restaurant;
mod rust_semantics;
mod more_semantics;

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::{hash_map, HashMap};
use std::f32::consts::PI;

use crate::more_semantics::error_handling;
use crate::restaurant::order_food;

fn main(){
    more_semantics::smart_pointers();
}