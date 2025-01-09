// Constants 
// similar to immutable variables
// differences are,
// 1. not allowed to use "mut" with constants
// 2. name of the constant must be in BLOCK
// 3. type of the constant must be written

fn main(){
    println!("Hello, world!");

    const A:i32 = 10;
    println!("The value of A: {}", A);
    // const mut B:i32 = 10; // cannot use mut
    // println!("The value of B: {}", B); // throws an error

    println!("The value of PI is: {}", PI);

}

// You can declare a constant with a type annotation here! GLOABLLY
const PI: f64 = 3.141592653;