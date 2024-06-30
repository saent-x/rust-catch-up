#![allow(unused)]

mod restaurant;
mod rust_semantics;
mod error_handling;

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::{hash_map, HashMap};
use std::f32::consts::PI;

use crate::error_handling::error_handling;
use crate::restaurant::order_food;

fn main(){
    error_handling()
}