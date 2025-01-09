// Error Handling techniques [ 2 approaches ]

// // Approach 1 - Option
// enum Option<T>{ // Defining the generic Option type
//     Some(T), // Represents a value
//     None, // Repsents no value
// }

// Example
fn divideOption(numerator: f64, denominator: f64) -> Option<f64>{
    if denominator == 0.0{
        None
    } else {
        Some(numerator / denominator) // can't write numerator / denominator, have to enclose with Some()
    }
}


// // Aprroach 2 - Result
// enum Result<T, E> { // Define the generic Result type
//     Ok(T), // Represents a value
//     Err(E), // Represents an error
// }

fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String>{
    if denominator == 0.0{
        Err("Cannot divide by Zero!".to_string())
    } else {
        Ok(numerator / denominator)
    }
}





fn main(){

    // Using Option
    let result = divideOption(10.0, 0.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by Zero!"),
    }

    // Using Result
    match divideResult(10.0, 0.0) {
        Ok(x) => println!("Result: {}", x),
        Err(e) => println!("Error: {}",e),
    }

}