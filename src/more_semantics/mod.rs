use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

pub fn error_handling() {
    let path = "lines.txt";
    let output = File::create(path);

    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("problem creating file: {:?}", error);
        }
    };

    writeln!(output, "just kidding!\nrandom line")
        .expect("failed to write to file");

    let input = File::open(path).expect("failed to open file");
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        match line {
            Ok(content) => println!("{}", content),
            Err(error) => eprintln!("error reading line: {:?}", error),
        }
    }

    let output_2 = File::create("rand.txt");
    let output_2 = output_2.unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => match File::create("rand.txt") {
            Ok(fc) => fc,
            Err(error) => panic!("problem creating file: {:?}", error),
        },
        _other_error => panic!("problem opening file: {:?}", error),
    });
}

pub fn iterators(){
    let arr_it = [1,2,3,4,5,6,7,8,9,10];

    for val in arr_it.iter(){
        println!("{val}");
    }

    let mut iter_1 = arr_it.iter();

    println!("1st : {:?}", iter_1.next())
}

// closures
pub fn closures(){
    // like lamdas
    let can_vote = |age: i32| -> bool {
        age > 18
    };

    println!("can vote? {}", can_vote(20));

    fn math_funcs<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
        func(a,b)
    }

    let sum = |a, b| a + b;
    let prod = |a, b| a * b;

    println!("78 + 74 = {}", math_funcs(78, 74, sum));
    println!("78 * 74 = {}", math_funcs(78, 74, prod));
}