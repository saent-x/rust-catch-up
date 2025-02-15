use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
pub mod test_examples;


pub fn guessing_game_example() {
    println!("Guess Number...");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("\nEnter Number: ");
        io::stdout().flush(); // so that the line is emitted immediately

        let mut user_guess = String::new();

        io::stdin()
            .read_line(&mut user_guess) // [&mut guess] since references are immutable by default
            .expect("Failed to read line");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", user_guess);

        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct, You Win!");
                break;
            }
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

pub fn data_types() {
    let z: u32 = 23_33_000; // 23_54 literals helps for visualization
    let a: f32 = 30_000.4;

    println!("decimal {}", z + 1);
    println!("float {}", a);

    let div = -5 / 3;
    let div2 = -5.0 / 3.0;

    println!("div1 {}", div);
    println!("div2 {}", div2);

    let x: u8 = 0;
    // println!("x = {}", x-1); // error here | would return 255 in debug mode

    // tuples
    let card: (u64, &str, char) = (18772663, "JohnPaul", 'm');
    println!("card -> {} {} {}", card.0, card.1, card.2);

    // array
    let col = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let col2 = [10; 10]; // creates an array of size 10 with the entry 10, 10 times

    println!("col2 {:?}", col2);
}

pub fn control_flow() {
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
                break;
            }
            if counter == 5 {
                println!("last loop_1 = {}", counter);
                break 'loop_1;
            }
            counter_2 += 1;
        }
        counter += 1;
    }

    // while loop
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut counter = 0;
    while counter < 10 {
        println!("arr = {}", arr[counter]);
        counter += 1;
    }

    // for loop
    for el in arr {
        println!("el = {}", el);
    }
}

pub fn ownership() {
    // Ownership, Boxes, Moves
    let a = Box::new([0; 100]);
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

    let mut v = vec![1, 2, 3];
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

pub fn structs_1() {
    struct Animal {
        name: String,
        genus: String,
        is_black: bool,
    }

    let animal = Animal {
        name: String::from("Dog"),
        genus: String::from("Canis"),
        is_black: false,
    };

    let animal_2 = Animal {
        name: String::from("Wolf"),
        ..animal // using struct update syntax
    };

    // Tuple Struct
    #[derive(Debug)]
    struct Version(u32, u32, u32);

    #[derive(Debug)]
    struct Package(String, Version);

    let version = Version(1, dbg!(10 / 5), 0); // dbg! macro helps us figure out what our code is doing.
    let package = Package(String::from("flutter-engine"), version);

    println!("{:#?}", package);

    dbg!(&package);

    // Unit like structs
    struct PASSED;
    struct FAILED;

    let result = PASSED;
}

pub fn struct_2() {
    struct Animal {
        name: String,
        genus: String,
    }

    impl Animal {
        // associated functions
        fn new(name: String, genus: String) -> Self {
            Self { name, genus }
        }

        // this is a method primarily because it takes self as a parameter - its still an associated function though
        fn make_sound(&self) {
            println!(
                "{} from the family of {} makes a sound: Woof Woof!!",
                self.name, self.genus
            )
        }

        // another associated function
        fn show_struct_type() {
            println!("Type: Animal")
        }
    }

    let dog = Animal::new(String::from("Jackie"), String::from("Canis"));

    dog.make_sound();

    Animal::make_sound(&dog); // calling a method like as associated function but passing the dog obj as self.
    Animal::show_struct_type(); // calling associated function

    // -----------------------
    #[derive(Clone, Copy)]
    struct Version(u32, u32, u32);

    let version = Version(1, 0, 0);

    impl Version {
        fn get_version(&self) -> String {
            format!("{}.{}.{}", self.0, self.1, self.2)
        }

        fn own(self, another: Version) -> Version {
            Version(
                self.0.max(another.0),
                self.1.max(another.1),
                self.2.max(another.2),
            )
        }

        fn mut_fn(&mut self, another: Version) {
            *self = self.own(another)
        }
    }

    println!("version: {}", version.get_version());
}

pub fn enums_1() {
    // * Enums Implementation
    enum Shape {
        Circle,
        Square,
        Rectangle,
        Star,
    }

    let shape = Shape::Rectangle;

    // * Using Enums with match
    match shape {
        Shape::Circle => println!("A Circle"),
        Shape::Square => println!("A Square"),
        Shape::Rectangle => println!("A Rectangle"),
        Shape::Star => println!("A Star"),
    }

    // ---------------------

    enum Color {
        Red,
        Yellow,
        Blue,
    }

    fn print_color(color: Color) {
        match color {
            Color::Blue => println!("Hello Blue"),
            Color::Red => println!("Hello Red"),
            Color::Yellow => println!("Hello Yellow"),
        }
    }

    print_color(Color::Yellow);
}

pub fn enum_variants_structs_and_matching() {
    // * Enums Variants
    enum HouseType {
        Apartment,
        Flat,
        Studio,
    }

    enum RealEstate {
        House(HouseType),
        MinimumRentFee(u32),
    }

    let r_estate = RealEstate::House(HouseType::Studio);

    match r_estate {
        RealEstate::House(HouseType::Apartment) => println!("no longer available..."),
        RealEstate::House(HouseType::Studio) => println!("studios are immediately available"),
        RealEstate::House(housetype) => println!("available in a few months..."),
        _ => (),
    }

    // * Match Structs
    struct CartItem {
        product_name: String,
        product_price: u32,
        count: u32,
    }

    let cart_item = CartItem {
        product_name: "piano".to_owned(),
        product_price: 300,
        count: 5,
    };

    match cart_item {
        CartItem { count: 5, .. } => println!("you qualify of a discount..."),
        CartItem { count, .. } => println!("sorry you don't qualify for the discount"),
    }
}

pub fn advanced_matching_example() {
    enum Ticket {
        Backstage(String, u32),
        VIP(String, u32),
        Standard(u32),
    }

    let bs_ticket = Ticket::Backstage(String::from("Warden"), 8000);
    let vip_ticket = Ticket::VIP(String::from("JohnP."), 10_000);
    let sd_ticket = Ticket::Standard(2000);

    let tickets = vec![bs_ticket, vip_ticket, sd_ticket];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(holder_name, price) => println!(
                "Ticket: Backstage, Holder's Name: {}, Price: {}",
                holder_name, price
            ),
            Ticket::VIP(holder_name, price) => println!(
                "Ticket: VIP, Holder's Name: {}, Price: {}",
                holder_name, price
            ),
            Ticket::Standard(price) => println!("Ticket: Standard, Price: {}", price),
        }
    }
}

/// Introductions to using the Options types
/// * Option - Options are types that could be one of two things
/// * 1. Some data of a specified type
/// * 2. Nothing
/// * -> Used in scenarios where data may not be required or is unavailable
pub fn options_1() {
    struct Account {
        name: Option<String>,
        id: u32,
    }

    let john = Account {
        name: Some(String::from("JohnPaul")),
        id: 1,
    };

    let paul = Account { name: None, id: 2 };

    match john.name {
        Some(name) => println!("Name: {name}"),
        _ => println!("No name specified"),
    }

    // ---------------

    struct Employee {
        email: String,
        employee_id: u32,
    }

    let employee_1 = Employee {
        email: String::from("jp@mailer.com"),
        employee_id: 1,
    };

    let employee_2 = Employee {
        email: String::from("grace@mailer.com"),
        employee_id: 2,
    };

    fn get_authorized_employee(emp: &Employee, optional: Option<u32>) -> Option<String> {
        match emp {
            Employee { employee_id: 1, .. } => Some(String::from("Employee Authorized")),
            Employee { .. } => None,
        }
    }

    match get_authorized_employee(&employee_1, None) {
        Some(employee_authorization) => println!("Authorization: {employee_authorization}"),
        _ => println!("No Auhtorization"),
    }
}

/// An example of the Options type implemented with Structs Matching
pub fn options_example() {
    struct Student {
        name: String,
        locker_assigned: Option<u32>,
    }

    let student_1 = Student {
        name: String::from("john"),
        locker_assigned: None,
    };

    let student_2 = Student {
        name: String::from("paul"),
        locker_assigned: Some(56),
    };

    match &student_2 {
        Student {
            name,
            locker_assigned,
        } => match locker_assigned {
            Some(locker) => println!("{name} assigned locker {locker}"),
            _ => println!("{name} assigned no locker"),
        },
    }
}

pub fn result_datatype() -> Result<(), String> {
    /// Result Data Type
    /// A data type that contains one of two types of data
    /// 1. Successful data
    /// 2. Error data

    #[derive(Debug)]
    struct ApiResponse {
        data: String,
    }
    fn connect_to_api(api_link: &str) -> Result<ApiResponse, String> {
        let mut rng = rand::thread_rng();
        let api_result = rng.gen_bool(0.5);

        match api_result {
            true => Ok(ApiResponse {
                data: String::from("{'user': 'james bond', 'status': 'login successful'}"),
            }),
            false => Err(String::from("error: api not reacheable")),
        }
    }

    let api_response = connect_to_api("https://backend/api/login");
    match api_response {
        Ok(api_response) => println!("success: {api_response:?}"),
        Err(e) => println!("{e}"),
    }

    // alternative to match when reading from a function that returns a result
    let api_response_1: ApiResponse = connect_to_api("https://backend2/api/login")?;

    Ok(())
}

pub fn result_example() -> Result<(), String> {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    impl Person {
        fn new(name: &str, age: u32) -> Result<Self, &str> {
            match age.cmp(&21) {
                Ordering::Greater => Ok(Self {
                    name: name.to_owned(),
                    age,
                }),
                Ordering::Equal => Ok(Self {
                    name: name.to_owned(),
                    age,
                }),
                _ => Err("sorry you're below the required age of 21"),
            }
        }
    }

    let person_1 = Person::new("john", 26)?;
    println!("person_1: {}, status: eligible", &person_1.name);

    let person_2 = Person::new("paul", 20)?;
    println!("person_2: {}, status: eligible", &person_2.name);

    Ok(())
}

pub fn generics_i(){
    struct Animal<T> {
        animal_type: T,
        color: String
    }
    
    impl<T> Animal<T> {
        fn check_type(&self, a_type: T){
            
        }
    }
    // cannot have same fn name in a general and generic implementation
    // impl Animal<Dog>{
    //     fn check_type(&self, v: u32){
            
    //     }
    // }
    
    fn any_type<T>(a: T) -> T {
        a
    }
    
    fn not_just_any_type<T: PartialOrd>(b: T, c: T) -> T {
        match b > c{
            true => b,
            false => c
        }
    }
}

pub fn traits_example(){
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
}

