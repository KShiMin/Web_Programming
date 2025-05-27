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
 * Example 2: Using basic data types, control flow, and structs
 *
 */

fn main() {
    // Integer, float, boolean, and character types
    let x: i32 = 10;      // 32-bit signed integer
    let y: f64 = 3.14;    // 64-bit floating point
    let is_active: bool = true;   // Boolean
    let grade: char = 'A';        // Character

    // Print all basic types
    println!("x: {}, y: {}, is_active: {}, grade: {}", x, y, is_active, grade);

    // Use a conditional statement (if-else)
    if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is not greater than 5");
    }

    // Define a struct (custom data type)
    struct Person {
        name: String,
        age: u8,
    }

    // Create an instance of the struct
    let user = Person {
        name: String::from("Carol"),
        age: 28,
    };

    // Access struct fields
    println!("Person: {}, age: {}", user.name, user.age);

    // Use a match statement (pattern matching)
    // https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html
    match user.age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=64 => println!("Adult"),
        _ => println!("Senior"),
    }

    // Use a simple for loop
    // This is exclusive range. The .. operator "excludes" the upper bound.
    // use ..= (inclusive range) if you want to include the upper bound (3). 
    for i in 0..3 { // Ranges are exclusive at the end (0, 1, 2)
        println!("Counting: {}", i);
    }
}