use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io;
use std::collections::HashMap;
extern crate rand;
use rand::Rng;
mod ext;
extern crate regex;
use regex::Regex;
extern crate reqwest;
extern crate tokio;
use std::process::Command;


fn main() {
    fn VARIABLES () { //4
        
        let mut a = 5; //i32
        println!("{}", a);
        // All the variables are imutable by default
        a = 4; 
        println!("{}", a);
    
    }

    fn DATATYPES () { //5
        
        let a: i32 = 1; // Integer 32 bits
        let b: u32 = 0; // "u" does not support negative numbers//
        let c: f32 = 1.7; //"f32" float 32 bits
        let d: bool = false; // boolean
    
    }

    fn IF_ELSE_STATEMENTS () { //6
        
        let e = 20;
        if e < 30 {
            println!("The number is less than 30");
        }
        else if e > 30 {
            println!("The number is greater than 30");
        }
        else {
            println!("The number is equals 30");
        }
        
    }
    
    fn INFINITE_LOOP () { //7

        let mut f = 0;

        loop{
            f += 1; //Increase counter
            if f == 7 {continue;} //Turn back to the top
            if f > 10 {break;} //Stop looping
            println!("The value of 'f' is {}", f);
        }

    }
    
    fn WHILE_LOOP () { //8

        let mut n = 0;
        while n <= 50 {
            if n < 10 {println!("|---------- {} ----------|",n);}
            else {println!("|---------- {} ---------|",n);}
            n += 1;
        }

    }
    
    fn FOR_LOOP () { //9

        let numbers = 30..51;

        for i in numbers{
            println!("{}",i);
        }

        let animals = vec!["Turtle", "Bird", "Dog"]; //Defining a vector//

        for i in animals.iter(){ //iter() is used for vectors in for//
            println!("{}",i);
        }

        for (i, a) in animals.iter().enumerate(){
            println!("The animal index {} is {}",i,a);
        }

    }
    
    fn ENUM_TYPES () { //10

        enum Direction {
            Up,
            Down,
            Left,
            Right
        }

        let player_direction:Direction = Direction::Down;

        match player_direction {
            Direction::Up => println!("We are heading up!"),
            Direction::Down => println!("We are heading down!"),
            Direction::Left => println!("We are heading left!"),
            Direction::Right => println!("We are heading right!"),
        }

    }

    fn CONSTANTS () { //11

        const MAXIMUM_NUMBER: u8 = 20;

        for n in 1..MAXIMUM_NUMBER {
            println!("{}",n);
        }

    }
    
    fn TUPLES () { //12

        let tup1 = (20, "Batata",30, (1, 4 ,7)); 
        println!("{}", (tup1.3).1);

        let tup2 = ("Maionese", "3", "1.5");
        let (a,b,c) = tup2;
        println!("a is {}, b is {} and c is {}",a,b,c);
    }
    
    fn FUNCTIONS () { //13

        //print_numbers_to(10);

        fn print_numbers_to(num: u32){

            for i in 1..num+1 {
                println!("{}", i);
            }

        }

        fn is_even (num: u32) -> bool {
            return num % 2 == 0;
        }

        println!("{}", is_even(3));
    }
    
    fn CODE_BLOCKS () { //14

        let x = 10;

        {
            //ISOLATED but have acess to outside data
            
            let y = 5;

            println!("x: {} y: {}", x, y);
        }
        


    }
    
    fn SHADOWING () { //15

        let mut x = 10;

        {
            let x = 15;
        }
        let x = "string";
        println!("x is a {}", x);

        let x = true;
        println!("x is {}", x);

    }
    
    fn REFERENCES (){ //16

        let mut x = 10;

        //let xr = &x; // "&x" referencing variable "x", xr can't chance the value of x because is'nt mutable;
        //let mut xxr = &x; // referencing x, xxr can change the value of x because is mutable;
        {
            let xxxr = &mut x; // referencing x mutable;
            *xxxr += 1;
        }
        

        println!("X is {}", x);

    }
    
    fn STRUCTS () { //17

        struct Color {

            red: u8, // u8: 0 - 255
            green: u8,
            blue: u8

        }   

        let mut bg = Color {red: 255, green: 150, blue: 100};
        bg.green = 12;

        println!("{}, {}, {}", bg.red, bg.green, bg.blue);
    }
    
    fn TUPLE_STRUCTS () { //18

        struct Color (u8, u8, u8);

        let mut red = Color (255, 0, 0);
        red.0 = 230;
        println!("Red is {}, {}, {}", red.0, red.1, red.2);


    }
    
    fn PASS_BY_REFERENCE () { //19

        struct Color {

            red: u8, // u8: 0 - 255
            green: u8,
            blue: u8

        }   

        let mut blue = Color {red: 0, green: 0, blue: 255};

        fn print_color (c: &Color){
            println!("Color - R {}, G {}, B {}", c.red, c.green, c.blue);
        }

        print_color(&blue);
    }
    
    fn ARRAYS (){ //20

        //let numbers = [1, 2, 3, 4, 5];

        // let numbers: [i32; 5] = [1, 2, 3, 4, 5]; //specifiying the datatype and the array size

        //numbers [0] // == 1

        let numbers = [2; 400];

        for n in numbers.iter(){
            //println!("{}", n);
        }
        //or
        for i in 0..numbers.len(){
            println!("{}", numbers[i]);
        }

    }
    
    fn IMPLEMENTATION_KEYWORD () { //21

        struct Rectangle {
            width: u32,
            height: u32
        }

        impl Rectangle {
            fn print_description(&self){
                println!("Rectangle: {} x {}", self.width, self.height);
            }
            fn is_square (&self) -> bool{
                self.width == self.height
            }
        }

        let my_rect = Rectangle { width: 10, height: 5 };

        my_rect.print_description();

        println!("My rect is square: {}", my_rect.is_square());

    }
    
    fn STRINGS () { //22

        let mut my_string = String::from("How's it going? My name is Gui.");

        //length
        println!("My string length is: {}", my_string.len());
        //is empty?
        println!("My string is empty: {}", my_string.is_empty());

        //split whitespace
        for token in my_string.split_whitespace(){
            println!("{}", token);
        }

        println!("Does the string contain 'Gui': {}", my_string.contains("Gui"));

        //Pushing into the string
        my_string.push_str(" Welcome to the jungle!");

        println!("{}", my_string);

    }

    fn TRAITS () { //23

        struct Person {
            name: String,
            age: u8
        }   

        impl ToString for Person {
            fn to_string (&self) -> String {
                return format! ("My name is {} and I am {}.", self.name, self.age);
            }
        }
        
        let gui = Person {name: String::from("Guilherme"), age: 19};

        println!("{}", gui.to_string());
    }
    
    fn VECTORS () { //24

        //let my_vector: Vec<i32> = Vec::new();
        //or
        let mut my_vector = vec![1, 2, 3, 4];
        
        println!("{}", my_vector[2]);

        //add to vector into a new index
        my_vector.push(49);
        println!("{}", my_vector[4]);

        //remove some index
        my_vector.remove(1);
        println!("{}", my_vector[1]);

        for number in my_vector.iter(){
            print!("{} ", number);
        }
        println!(" ");

    }
    
    fn READING_FILES () { //25

        //NECESSARY THE USE OF SOME LIBRARIES
        // "use std::fs::File;"
        //AND
        // "use std::io::prelude::*;"

        let mut file = File::open("info.txt").expect("Can't open file!");

        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect("Oops! Can't read the file...");

        println!("File contents: \n\n {}", contents);

    }
    
    fn COMMAND_LINE_ARGS () { //26

        // "use std::env;"

        let args: Vec<String> = env::args().collect();

        for argument in 1..args.len(){
            println!("{}", argument);
        }

        //or

        println!("The argument in second string position (1) is: {}", args[1]);


    }
    
    fn WRITING_FILES () { //27

        // "use std::fs::File;"
        // "use std::io::prelude::*;"

        let mut file = File::create("output.txt")
            .expect("Couldn't create file");

        file.write_all(b"Testing this!")
            .expect("Can't write file");
        
    }
    
    fn DEFINING_TRAITS () { //28

        struct Person {
            name: String,
            age: u8
        }

        trait HasVoicebox {
            //Speak
            fn speak(&self);
            //Check if can speak
            fn can_speak(&self) -> bool;
        }

        impl HasVoicebox for Person {
            fn speak(&self) {
                println!("Hello, my name is {}", self.name);
            }
            fn can_speak(&self) -> bool {
                if self.age > 0 {
                    return true;
                } return false;

            }
        }

        let person = Person {
            name: String::from("Gui"),
            age: 0
        };

        println!("Can {} speak? {}", person.name, person.can_speak());

    }

    fn PATTERN_MATCHING () { //29 //SWITCH OPERATOR

        let number = 9;
        let word = "batata";

        match number {
            1 => println!("This is one!"),
            2 => println!("This is two!"),
            3 => println!("This is three!"),
            5...9 => println!("Good morning!"),
            10 | 11 => println!("This is ten or eleven."),
            _ => println!("It doesn't match!")
        }

        match word {
            "batata" => println!("You choose batata"),
            "rice"   => println!("You choose rice"),
            _        => println!("No matches found!")
        }


    }

    fn READING_USER_INPUT () { //30
        
        //use std::io;

        let mut input = String::new();

        println!("Hey mate! Say someting: ");
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("Sucess! You said: {}", input.to_uppercase());
            },
            Err(e) => println!("Oops! Something went wrong: {}", e)
        }

    }
    
    fn HASH_MAPS () { //31

        //use std::collections::HashMap;

        let mut marks = HashMap::new();

        //Add values
        marks.insert("Rust Programming", 96);
        marks.insert("Web Development", 94);
        marks.insert("UX Design", 75);
        marks.insert("Professional Computing Studies", 45);

        //Find lenght of HashMap
        println!("The HashMap lenght is: {}", marks.len());

        //Get a single value
        match marks.get("Web Development"){
            Some(mark) => println!("You got {} for Web Dev!", mark),
            None => println!("You didn't study Web Development")
        }

        //Remove a value
        marks.remove("UX Design");

        //Loop through HashMap
        for (subject, mark) in &marks {
            println!("For {} you got {}%!", subject, mark);
        }

        //Check for a value
        println!("Did you study C++? {}", marks.contains_key("C++ Programming"));



    }
    
    fn RANDOM_NUMBERS () {  //32

        //Insert "rand = 0.3" into cargo.toml -> Dependencies
        //extern crate rand;
        //use rand::Rng;

        let random_number = rand::thread_rng().gen_range(1, 11); //Random number 1 - 10
        println!("Random Number: {}", random_number);
        
        //Flip a coin
        let probability_for_true = 5; // 1 chance in 5 for true
        let random_bool = rand::thread_rng().gen_weighted_bool(probability_for_true);

        println!("Random Bool: {}", random_bool);

    }
    
    fn STRING_METHODS () { //33

        //Replace
        {
            let my_string = String::from("Rust is fantastic!");
            println! ("After replace: {}", my_string.replace("fantastic", "awesome"));
        }

        //Lines
        {
            let my_string = String::from("The weather is\nnice\noutside mate!");
            
            for line in my_string.lines(){
                println!("[ {} ]", line);
            }
        }

        //Split
        {
            let my_string = String::from("Leave+a+like+if+you+enjoyed!");
            let tokens: Vec<&str> = my_string.split("+").collect();

            println!("At index 2: {}", tokens[2]);
        }

        //Trim
        {
            let my_string = String::from("    My name is Gui  \n\r");
            println!("Before trim: {}", my_string);
            println!("After trim: {}", my_string.trim());
        }

        //Chars
        {
            let my_string = String::from("Batata bread");
            println!("My string: {}", my_string);

            //Get character at index
            match my_string.chars().nth(4) {
                Some(c) => println!("Character at index 4: {}", c),
                None => println!("No character at index 4")
            }
        }
    }
    
    fn MULTIPLE_SOURCE_FILES () { //34

        //INCLUDE THE FILE PATH WITH "mod filepath" WITHOUT ".rs" extension.
        //Ex: mod ext;

        ext::print_message();
    }

    fn REGULAR_EXPRESSIONS () { //35

    //include regex dependency in cargo.toml (regex = "0.2")
    //extern crate regex;
    
    let re = Regex::new(r"(\w{5})").unwrap(); //Five letter word
    let text = "RustL";

    match re.captures(text){
        Some (caps) => println!("Found match: {}", &caps[0]),
        None => println!("Coundn't find a match!")
    }

    }
    
    fn MODULES () { //36

        mod test {

            fn batata(){
                println!("Batata");
            }

            pub fn getBatata() {
                batata();
            }

            pub mod water {
                pub fn print_message() {
                    println!("I'm water");
                }
            }

        }
        test::getBatata();
        test::water::print_message();

    }
    
    fn OPTION_ENUM () { //37

        let name = String::from("Gui");

        println!("Character at index 4: {}", match name.chars().nth(4) {
            Some(c) => c.to_string(),
            None => "No character at index 4".to_string()
        });

        fn get_occupation(name: &str) -> Option<&str> {
            match name {
                "Gui" => Some("Bitcoinheiro"),
                "Michael" => Some("King"),
                _ => None
            }
        }

        println!("Occupation is {}", match get_occupation(&name){
            Some(o) => o,
            None => "No occupation Found" 
        });
    }
    
    fn HTTP_GET_REQUEST () { //38

        //Insert into cargo.toml / dependencies (reqwest = { version = "0.11.1", features = ["json"] })
        //Insert into cargo.toml / dependencies (tokio = { version = "1", features = ["full"] })
        //"extern crate reqwest;"
        //"extern crate tokio;"
        //"use std::collections::HashMap;"
        
        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
            let resp = reqwest::get("https://httpbin.org/ip")
                .await?
                .json::<HashMap<String, String>>()
                .await?;
            println!("{:#?}", resp);
            Ok(())
        }
        main();

    }

    fn ENUM_METHODS () { //39

        enum Day {
            Monday, Tuesday, Wednesday, Thurday, Friday, Saturday, Sunday
        }

        impl Day {
            fn is_weekday(&self) -> bool {
                match self {
                    &Day::Saturday | &Day::Sunday => return false,
                    _ => return true
                }
            }
        }

        let day = Day::Saturday;

        println!("Is day weekday: {}", day.is_weekday());

    }
    
    fn EXECUTING_CLI_COMMANDS () { //40

        //"use std::process::Command;"

        let mut cmd = Command::new("echo");
        cmd.arg("test");

        //exec
        match cmd.output(){
            Ok(o) => {
                unsafe {
                    println!("Output: {}", String::from_utf8_unchecked(o.stdout));
                }
            },
            Err(e) => {
                println!("There was an error! {}",e);
            }
        }


    }
    
    //CHAME ALGUMA DAS FUNÇÔES AQUI!
    

}
