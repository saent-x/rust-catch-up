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

    datatypes_operators_keywords();
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