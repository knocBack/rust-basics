fn main(){
    hello_world();
    tell_height(124);
    human_id("Noman", 69, 143.1);
    let y:i32 = add(2,4);
    println!("Value from y is : {}",y);
    println!("Value from function is : {}",add(4,6));

    let x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty // No ; but also write this as "return price * qty;"
    };
    println!("Result is: {}", x);

    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);
}

// Hoisiting - can call function anywhere in your code
fn hello_world(){
    println!("Hello, Rust 🦀!");
}

// you can insert inpur values
fn tell_height(height: i32){
    println!("My height is: {}", height);
}

// you can insert more than one value
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and my  height is {} cm.", name, age, height);
}

// functions returning values
fn add(a: i32, b:i32) -> i32 {
    a + b // could also write as "return a+b;"
}

// Expressions and Statements
// Expression: Anything that returns a value.
// Statement: Anything that does not return a value.
// Almost all statements in Rust end with ;
// let y = let x = 10; -> can't write like this
// Examples of statements:
// 1 Variable declarations: let x = 5;
// 2 Function definitions: fn foo() {}
// 3 Control flow statements: if condition { /* code */ } else { /* code */ }, while condition { /* code */ }, etc.


// Expression
//-------------------
// 5
// true & false
// add(3,4)
// if condition { value1 } else { value2 }
// ({code}) -> blocks

// variable x in main() fn


// Final examples
// BMI = weight(kg) / height(m)^2

fn calculate_bmi(weight_kgs: f64, height_mts: f64) -> f64{
    weight_kgs / (height_mts*height_mts)
}