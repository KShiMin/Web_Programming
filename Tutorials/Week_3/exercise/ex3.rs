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
 * Example 3: Arrays, vectors, tuples, Option, and basic error handling
 *
 */ 

fn main() {
    // Fixed-size array of integers
    let nums: [i32; 4] = [10, 20, 30, 40];

    // Access array elements by index
    println!("First number: {}", nums[0]);

    // Growable vector
    let mut fruits = vec!["Apple", "Banana"];
    fruits.push("Orange");        // Add element at the end
    println!("Fruits: {:?}", fruits); // {:?} prints debug output (programmer-friendly)

    // Iterate through vector with a for loop
    for fruit in &fruits {
        println!("Fruit: {}", fruit);
    }

    // Tuple containing mixed types
    let person: (&str, i32, bool) = ("Dave", 42, true);
    println!("Name: {}, Age: {}, Is active: {}", person.0, person.1, person.2);

    // Option type for possible absence of value
    let some_number: Option<i32> = Some(100);
    let no_number: Option<i32> = None;

    // Using match to safely unwrap Option
    match some_number {
        Some(n) => println!("We have a number: {}", n),
        None => println!("No number found"),
    }

    // Basic error handling with Result
    // This tries to parse a string into a number
    let result = "123".parse::<i32>();
    match result {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Failed to parse: {}", e),
    }

    // Example with expected error
    let result2 = "abc".parse::<i32>();
    match result2 {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Failed to parse: {}", e),
    }
}