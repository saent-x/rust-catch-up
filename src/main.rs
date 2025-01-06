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
use crate::rust_book::examples::guessing_game_example;

fn main() -> Result<(), Box<dyn Error>> {
    // Traits
    trait Animal {
        fn make_sound(&self) -> String;
        fn get_name(&self) -> String { // Default trait implementations
            "not sure of my name".to_string()
        }
    }
    
    struct Dog<'a>(&'a str);
    
    impl<'a> Animal for Dog<'a>{
        fn make_sound(&self) -> String {
            "woof woof!".to_string()
        }
    }
    
    let d = Dog("doggo");
    let name = d.get_name();
    println!("{}", name);
    
    // Traits as parameters
    fn animal_sound(animal: &impl Animal){
        println!("{}", animal.make_sound())
    }
    
    // Traits as Generics Boundary
    fn animal_name<T: Animal>(animal: &T){
        println!("{}", animal.get_name())
    }
    
    animal_sound(&d);
    animal_name::<Dog>(&d); // explicitly defined generics
    
    Ok(())
}

fn get_csv_stats(filename: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let csv_file = File::open(filename)?;
    let mut csv_reader = csv::Reader::from_reader(csv_file);

    let mut no_of_ones: u32 = 0;
    let mut no_of_zeros: u32 = 0;

    for result in csv_reader.records() {
        let record = result?;
        let attack = &record[10];

        if attack == "1" {
            no_of_ones += 1;
        } else if attack == "0" {
            no_of_zeros += 1;
        }
    }

    Ok((no_of_ones, no_of_zeros))
}
