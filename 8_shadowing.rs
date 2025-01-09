// Shadowing in Rust
// Shadowing is not the same as marking a variable as mutable.
// changed the value, but reused the same name
// Shadowing can help in this scenario
// say you took an input from user, and named 
// let spaces = "    ";
// but you only need the length of this
// let spaces = spaces.len();
// with this new line, there's no need of useless variables like spaces_str, spaces_len, etc.

fn main(){
    let x = 5;
    let x = x + 1; // right hand x is shadowed by left hand x, and now x = 6
    println!("The value of x: {}", x);

    {
        let x = x * 2;
        println!("The value of x inside a scope: {}", x);
    }

    println!("The value of x in main function is {x}");

    let spaces = "    ";
    println!("Spaces before shadowing: '{}'", spaces);
    let spaces = spaces.len();
    println!("Spaces after shadowing: {}", spaces);

    // helps us overcome the difficulty of making absurd names like spaces_str, spaces_len, etc, 
    // can change data types as well

    // However this will not work with mutability, implies, initializing 'let' keyword is MUST
    let mut name = "Hola";
    println!("name after shadowing: {}", name);
    // name = name.len(); // Gives an error because you are not updating &str with usize
    let name = name.len();
    println!("name after shadowing: {}", name);
}