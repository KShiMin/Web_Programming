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
 * Example 1: Basic Hello World program with variables and printing
 *
 */ 

fn main() {
    // Declare a mutable variable using let mut
    let mut name = "Alice"; // 'name' is a string slice with value "Alice"
    
    // Declare an immutable variable (default)
    let age: u32 = 30; // 'u32' is an unsigned 32-bit integer
    
    // Print text to the console using the println! macro
    println!("Hello, world!"); // Simple print statement

    // Print variables using formatting
    println!("My name is {}.", name);
    println!("I am {} years old.", age);

    // Change the value of a mutable variable
    name = "Bob";
    println!("Now my name is {}.", name);

    // Call a function and use its return value
    let message = get_greeting(name);
    println!("{}", message);
}

// Define a function that takes a string slice argument and returns a String
fn get_greeting(name: &str) -> String {
    // Use the format! macro to format a String
    format!("Greetings, {}!", name)
}