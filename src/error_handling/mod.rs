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