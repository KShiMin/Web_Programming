/**
 * Author: Dr. Peter Yau
 * Email: PeterCY.Yau@glasgow.ac.uk
 * 
 * Copyright 2025
 * School of Computing Science, University of Glasgow
 *  
 * Disclaimer:
 * Programmers should ensure the environment and settings are correct.
 * This code does not confirm 100% accuracy. Remember to change the filename and 
 * class name to match your specific implementation.
 *
 * Example 5: Enums, pattern matching, methods, traits, and vectors of custom types
 *
 */ 

// Define an enum for messages with different types of data
enum Message {
    Quit,                    // No data
    ChangeColor(i32, i32, i32), // 3 integers (RGB)
    Echo(String),            // String data
    Move { x: i32, y: i32 }, // Named fields in a struct-like variant
}

// Implement methods for Message using 'impl'
impl Message {
    // Method that processes the message and prints what it does
    fn process(&self) {
        match self {
            Message::Quit => {
                println!("Quit message received.");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change color to red {}, green {}, blue {}", r, g, b);
            }
            Message::Echo(text) => {
                println!("Echo message: {}", text);
            }
            Message::Move { x, y } => {
                println!("Move to position: ({}, {})", x, y);
            }
        }
    }
}

// Define a trait (similar to interface in other languages)
trait Speak {
    fn speak(&self);
}

// A custom struct
struct Animal {
    name: String,
}

// Implement the Speak trait for Animal
impl Speak for Animal {
    fn speak(&self) {
        println!("{} says: Hello!", self.name);
    }
}

fn main() {
    // Using the enum with all its variants
    let msgs = vec![
        Message::Echo(String::from("hello!")),
        Message::Move { x: 2, y: 3 },
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];

    // Iterate and process each message
    for msg in &msgs {
        msg.process();
    }

    // Create a vector of custom types (Animal)
    let animals = vec![
        Animal { name: String::from("Dog") },
        Animal { name: String::from("Cat") },
    ];

    // Call trait method on each animal
    for animal in &animals {
        animal.speak();
    }
}