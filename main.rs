#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    // print!("Start Value: ");
    //
    // let mut value =  String::new();
    //
    // io::stdin()
    //     .read_line(&mut value)
    //     .expect("failed to real line");
    //
    // let val: i32 = shadowing(10);
    //
    // println!("End value: {val}")

    string_s()
}

fn datatypes_operators_keywords(){
    let arr_1 = [1,2,3,4,5,6,7,8,9,10];
    let mut loop_idx = 0;

    // loop keyword
    loop{
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx += 1;

            continue;
        }
        if arr_1[loop_idx] == 9 {
            break;
        }

        println!("val: {}", arr_1[loop_idx]);
        loop_idx += 1;
    }

    // while
    while loop_idx < arr_1.len(){
        println!("arr: {}",  arr_1[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_1.iter() {
        println!("val: {}", val);
    }

    let tuple: (u8, String, f64) = (67, "John".to_string(), 50_000.00);

    println!("Name: {}", tuple.1);
}

fn string_s(){
    let mut str_1: String = String::new();

    str_1.push('B');
    str_1.push('-');
    str_1.push_str("word");

    let str_2: String = String::from("d k d f g w y s e f g");
    let mut v_1: Vec<char> = str_2.chars()
        .collect();

    v_1.sort();
    v_1.dedup();

    for char in v_1{
        println!("{}", char);
    }

    let str_3: &str = "Temp String";
    let mut str_4: String = str_3.to_string();

    println!("{}", str_4);

    let byte_arr_1 =  str_4.as_bytes();
    let str_5 = &str_4[0..6];

    println!("length: {}", str_5.len());

    str_4.clear();

    let str_6 = String::from("Just some nerve");
    let str_7 = String::from("... yh");

    let str_8 = str_6 + &str_7; // str_6 borrowed

    for char in str_8.bytes(){
        println!("{}", char);
    }
}

// loops variables and datatypes
fn loops_var_dt(){
    let mut counter: i64 = 0;
    const LANG: &str = "rust";

    loop {
        if counter == 100 {
            break;
        }

        println!("hello rust -> counter is {counter}");
        counter += 1;  
    }
}

fn shadowing(value: i32) -> i32{
    let x = value;

    let x = x * 5;

    {
        let sub: i32 = x - 1;
        let x: i32 = x - sub;

        println!("x value in shadow: {x}");
    }

    println!("x value outside shadow: {x}");

    x
}