
// use std::io;

// references &String allow you to refer to some value without taking ownership of it
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_width(&self) -> &u32 {
        &self.width
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub trait MyLogTrait {
    fn log_format(&self) -> String;
}

impl MyLogTrait for Rectangle {
    fn log_format(&self) -> String {
        format!("width:{}, height:{}", self.width, self.height)
    }
}

fn main() {
    // Function
    let mut s1 = String::from("hello");
    change(&mut s1);
    let s1_len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, s1_len);

    // Structs
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    println!("rect1.get_width() = {}", rect1.get_width());

    // match and enum
    println!("value in cents is {}", value_in_cents(Coin::Penny));

    // Vector
    let mut v = vec![100, 32, 57];
    v.push(6);
    for i in &mut v {
        *i += 50; // use the * dereference operator to get to the value in i
        println!("{i}");
    }

    // HashMap
    let mut scores = std::collections::HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    
    // Trait
    println!("rect1.log_format(): {}", rect1.log_format());

    // println!("Please input your guess.");
    // let mut guess = String::new(); // mutable variables
    // io::stdin()
    //     .read_line(&mut guess) // The &mut indicates that this argument is a mutable reference
    //     .expect("Failed to read line"); // handle Result
    // println!("You guessed: {guess}");
}
