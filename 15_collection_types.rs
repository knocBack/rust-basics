// Collection Types
// Vectors - UTF8 - Hashmaps

use std::collections::HashMap; // importing HashMap

fn main(){
    // 1. Vectors
    // dynamic array
    // can hold single data type - homogeneous type of elements

    // initializing vector
    let v:Vec<i32> = Vec::new();
    let v2:Vec<i32> = vec![1,2,3]; // using macro

    let mut v3:Vec<i32> = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    v3.push(9);

    println!("{:?}", v3);

    let third: &i32 = &v3[2]; // 3rd element -> 2nd index // Direct indexing // No Ownership transfer
    println!("Third element is {}", third);

    let tenth: Option<&i32> = v3.get(9); // get 9th index index, 10th element 
    match tenth {
        Some(value) => println!("The tenth element is {}", value),
        None => println!("There is no tenth element.")
    }



    // UTF8 or also known as String in Rust
    let s = "whatever".to_string();
    let s = String::from("somewhere");

    let mut s = String::from("foo");
    s.push_str("bar"); // pushing a string
    s.push('!'); // pushing one character

    println!("the value of s = {}",s);

    // If you want to combine strings, use the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("new concatenated string s3 = {}", s3);

    // Formatting Strings
    let full_message = format!("{}-{}",s3,s2);
    println!("{full_message}");





    // Hashmaps
    // String Keys with Associated Values in Hash Maps

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Team - {}'s score is {}", team_name, score);

    println!("Team - {}'s score is {}", String::from("Yellow"), scores["Yellow"]);

    // println!("Team - {}'s score is {}", String::from("Red"), scores["Red"]); // exits from runtime


    for(key, value) in &scores{
        println!("{}: {}", key, value);
    }

    
}

