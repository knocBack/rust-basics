// Ownership, Borrowing and References

// Ownership
// ---------
// C, C++ -> Memory Management Control Issue
// Garbage Collector solved this issue, but created a new issue -> Slow Performance:
// [stopping/Resuming the program] - apps freezing because of this!

// OWNERSHIP introduced by Rust to solve safety issues and high performace at the same time.
// What is Ownership ?
// Every value has a single owner [every variable has one value, and it is its sole owner].

// Ownership Rules:
//-----------------
// 1- Each value in Rust has an owner.
// 2- There can be only one owner at a time.
// 3- When the owner goes out of scope, the value will be dropped.

// Example: Each value in Rust has a variable that is its owner
fn main(){
    let s1 = String::from("RUST"); // s1 is the owner of this string "RUST"
    let len = calculate_length(&s1); // passing reference instead of the owner itself! Ownership is not passed!
    println!("Length of '{}' is {}.", s1, len); 

    let s2 = s1; // passing the ownership of s1 to s2
    // println!("{}", s1); // gives an error now
    println!("{}", s2); // runs normally, prints "RUST"
}

// fn printLost(s: &String){
//     printfln!("{}", &s2); // s2 is not in this scope error!
// }


fn calculate_length(s: &String) -> usize{
    s.len()
}