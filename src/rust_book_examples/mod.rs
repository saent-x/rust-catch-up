
mod rust_book_examples{
    use std::cmp::Ordering;
    use std::io;
    use std::io::Write;
    use rand::Rng;

    pub fn guessing_game_example(){
        println!("Guess Number...");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        loop {
            print!("\nEnter Number: ");
            io::stdout().flush(); // so that the line is emitted immediately

            let mut user_guess = String::new();

            io::stdin()
                .read_line(&mut user_guess) // [&mut guess] since references are immutable by default
                .expect("Failed to read line");

            let user_guess: u32 = match user_guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {}", user_guess);

            match user_guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("Correct, You Win!");
                    break
                },
            }
        }
    }

    pub fn const_n_shadowing() {
        // constant
        const A: u32 = 60;

        // shadowing
        let x = 20;
        let x = x + 5;

        {
            // references the first instance of x instead of the second
            let x = x * 2;
            println!("The value of x is: {}", x);
        }

        println!("The value of x is: {}", x);
    }

    pub fn data_types(){
        let z: u32 = 23_33_000; // 23_54 literals helps for visualization
        let a: f32 = 30_000.4;

        println!("decimal {}", z+1);
        println!("float {}", a);

        let div = -5/3;
        let div2 = -5.0/3.0;

        println!("div1 {}", div);
        println!("div2 {}", div2);

        let x: u8 = 0;
        // println!("x = {}", x-1); // error here | would return 255 in debug mode

        // tuples
        let card: (u64, &str, char) = (18772663, "JohnPaul", 'm');
        println!("card -> {} {} {}", card.0, card.1, card.2);

        // array
        let col = [1,2,3,4,5,6,7,8,9];
        let col2 = [10;10]; // creates an array of size 10 with the entry 10, 10 times

        println!("col2 {:?}", col2);
    }

    pub fn control_flow(){
        let a = {
            let z = 100 * 7;
            z
        };

        println!("a = {}", a);

        // control flow
        let cond = true;
        let number = if cond { 5 } else { 6 };

        let mut counter = 0;
        loop {
            if counter == 5 {
                break;
            }
            println!("{}", counter);

            counter += 1;
        }

        let mut counter = 0;

        // another use of loops
        let loop_result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 20;
            }
        };
        println!("loop_result = {}", loop_result);

        // loop labels
        let mut counter = 0;
        'loop_1: loop {
            println!("loop_1 = {}", counter);

            let mut counter_2 = 0;
            loop {
                if counter_2 == 7 {
                    break
                }
                if counter == 5 {
                    println!("last loop_1 = {}", counter);
                    break 'loop_1
                }
                counter_2 += 1;
            }
            counter += 1;
        }

        // while loop
        let arr = [1,2,3,4,5,6,7,8,9,10];

        let mut counter = 0;
        while counter < 10 {
            println!("arr = {}", arr[counter]);
            counter += 1;
        }

        // for loop
        for el in arr{
            println!("el = {}", el);
        }
    }

    pub fn ownership(){
        // Ownership, Boxes, Moves
        let a = Box::new([0;100]);
        let b = a;

        // boxes are used by Vec, String, HashMap
        fn add_suffix(mut name: String) -> String {
            name.push_str(" Jr.");
            name
        }

        let first = String::from("John");
        let first_clone = first.clone();
        let full = add_suffix(first_clone);

        println!("full: {} first: {}", full, first);

        fn greet(g1: String, g2: String) -> (String, String) {
            println!("{} {}!", g1, g2);

            (g1, g2)
        }

        let b1 = String::from("John");
        let b2 = String::from("Paul");
        let (b1_again, b2_again) = greet(b1, b2);
        let fm = format!("{} {} formatted", b1_again, b2_again);

        println!("{}", fm);
    }

    pub fn ownership_n_borrowing() {
        // using references/borrowing to avoid verbose style in previous example
        fn greet_2(g1: &String, g2: &String) {
            println!("{} {}!", g1, g2);
        }

        let c1 = String::from("Jean");
        let c2 = String::from("Paul");
        greet_2(&c1, &c2);
        let fm_2 = format!("{} {} formatted", c1, c2);

        println!("{}", fm_2);

        // ------------------

        // let mut x = Box::new(1);
        //
        // let a: i32 = *x;
        // *x += 5;
        //
        // let ref_1 = &x;
        // let b = **ref_1;
        //
        // let ref_2:&i32 = &*ref_1;
        //
        // println!("b = {}", ref_2);
        //
        // let mut v: Vec<i32> = vec![1,2,3,4,5];
        // let num: &i32 = &v[3];
        //
        //
        // v.push(6);
        // println!("num = {}", *num);


        // let mut v: Vec<i32> = vec![1, 2, 3];
        // let num: &i32 = &v[2];
        //
        // // this creates a new vector on the heap, thereby invalidating the previous one which could make num invalid
        // v.push(4);
        // println!("Third element is {}", *num);

        let mut v = vec![1,2,3];
        let num: &mut i32 = &mut v[0];

        *num = 4;
        println!("first el: {}", *num);
        println!("first el: {:?}", v);

        fn ascii_capitalize(v: &mut Vec<char>) {
            let c = &v[0];
            if c.is_ascii_lowercase() {
                let up = c.to_ascii_uppercase();
                v[0] = up;
            } else {
                println!("Already capitalized: {:?}", v);
            }
        }



        let mut z: Vec<char> = vec!['a', 'b', 'c'];
        ascii_capitalize(&mut z);

        // ---------------


        fn incr(n: &mut i32) {
            *n += 1;
        }

        let mut n = 1;
        incr(&mut n);
        println!("{n}");

        // let mut s = String::from("hello");
        // let s2 = &s;
        // let s3 = &mut s;
        // s3.push_str(" world");
        // println!("{s2}");

        let mut s: String = String::from("hello");

        let r1: &String = &s; // immutable reference
        let r2: &String = &s; // immutable reference

        println!("{} {}", r1, r2);

        let r3: &mut String = &mut s; // mutable reference
        println!("{}", r3);


        let mut point = [0, 1];
        let mut x = point[0];
        let y = &mut point[1];

        x += 1;
        *y += 1;

        println!("{} {}", point[0], point[1]);
    }

    pub fn structs_1(){
        struct Animal{
            name: String,
            genus: String,
            is_black: bool
        }
    
        let animal = Animal{
            name: String::from("Dog"),
            genus: String::from("Canis"),
            is_black: false
        };
    
        let animal_2 = Animal{
            name: String::from("Wolf"),
            ..animal // using struct update syntax
        };
    
        // Tuple Struct
        #[derive(Debug)]
        struct Version(u32, u32, u32);
    
        #[derive(Debug)]
        struct Package(String, Version);
    
        let version = Version(1, dbg!(10/5), 0); // dbg! macro helps us figure out what our code is doing.
        let package = Package(String::from("flutter-engine"), version);
    
        println!("{:#?}", package);
    
        dbg!(&package);
    
        // Unit like structs
        struct PASSED;
        struct FAILED;
    
        let result = PASSED;
    }

    pub fn struct_2(){
        
    }
}

