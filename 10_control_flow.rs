// Control Flow in Rust
// 1- Conditions [If ... Else]
// 2- Repeating actions. [Loops]

// If Else [ If expression ] [ Else expression ]

#![allow(warnings)] // to stop warnings


fn main(){
    println!("Hello, world!");
    let age:u16 = 18;

    if age >= 18 {
        println!("You can drive a car!");
    } else {
        println!("You can't drive a car!");
    }

    
    // Else If - Multiple conditions
    let number:u16 = 6;
    if number % 4 == 0{
        println!("number is divisible by 4");
    } else if number % 3 == 0{
        println!("number is divisible by 3");
    } else if number % 2 == 0{
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }


    // Using if in a let statement
    let condition:bool = false;
    let number:i32 = if condition {5} else {6}; // doesnt have to be in single line
    // let number:i32 = if condition {5} else {"six"} // this is wrong, because they are different types
    println!("number is: {number}");

}

