// Variables and Mutability
// immutable means cannot be changed once initialized
fn main(){
    let a: u16 = 5; // default: immutable
    println!("The value of a is {}", a);
    // a = 10;
    // println!("The value of a is {}", a); // these two lines cannot run!
    let mut b: u16 = 6;
    println!("The value of b is {}", b);
    b = 10;
    println!("The value of b is {}", b); // this will run because b is mutable
}