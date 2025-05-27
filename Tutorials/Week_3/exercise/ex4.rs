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
 * Example 4: Slices, borrowing, string manipulation, and basic lifetime annotation
 *
 */ 

// This function takes a string slice reference and returns the length
fn string_length(s: &str) -> usize {
    s.len()
}

// This function takes two string slices and returns the longer one
// Here, we use a lifetime parameter to tie the return value's lifetime to the input lifetimes
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    // String literal (&'static str) and owned String
    // &str is a string slice. immutable, UTF-8, statically allocated (or borrowed from another string)
    // pointing to the string literal "hello", hardcoded in your program's binary.
    // Very efficient, lightweight, often used for read-only string data.
    let s1: &str = "hello";
    // heap-allocated (grow or shrink at runtime), mutable, growable string type.
    // when you need to modify a string or store user input, file data, concatenated values, etc.
    let mut s2: String = String::from("world");

    // Concatenation and string mutation
    s2.push('!');           // Add a char
    let s3 = format!("{} {}", s1, s2); // Concatenate with format!
    println!("Concatenated: {}", s3);

    // Borrowing: pass string as reference (no ownership transfer, zero copy)
    let len = string_length(&s2);
    println!("Length of s2: {}", len);

    // Slices: referencing parts of arrays or strings
    let arr = [1, 2, 3, 4, 5];
    let arr_slice = &arr[1..4]; // from index 1 up to (but not including) 4
    println!("Array slice: {:?}", arr_slice);

    let word = &s1[1..4]; // slice of "hello", gets "ell"
    println!("String slice: {}", word);

    // Using lifetime-annotated function
    let winner = longest(s1, &s2); // &s2 can convert &String to &str
    // let winner = longest(&s1, &s2); multiple references --> logically it is okay
    // let winner = longest(&s1, s2);
    println!("Longest string: {}", winner);

    // References vs ownership:
    let s4 = String::from("borrow me");
    print_str(&s4); // Borrow, s4 still valid here
    // print_str(s4); // Would move ownership if no &

    // After being borrowed, s4 can still be used
    println!("Still accessible: {}", s4);
}

// Function definition that borrows a string slice, doesn't take ownership
fn print_str(text: &str) {
    println!("Borrowed string: {}", text);
}