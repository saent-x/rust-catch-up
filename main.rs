
fn main(){
    loops_var_dt()
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