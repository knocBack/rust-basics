fn main(){
    /*
    Arrays
    */
    println!("Hello, 🦀!");
    let numbers: [i32;5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {}", fruits[0]);
    println!("Fruits Array 2nd element: {}", fruits[1]);
    println!("Fruits Array 3rd element: {}", fruits[2]);

    /*
    Tuples
    */
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);
    // just "Alice" is a string slice in RUST

    let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("My Mix Tuple: {:?}", my_mix_tuple);
    // not needed to mention types for tuples

    /*
    Slices : dynamically sized view into contigious sequence of elements
    */
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices:&[&str] = &["Zebra", "Lion"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices:&[String] = &["Harry Potter".to_string(), "Alchemist".to_string()];
    println!("Book Slice: {:?}", book_slices);


    /*
    Strings Vs String Slices(&str)
    String [ growable, mutable, owned string type ]
    */

    // String (s) are allocated in the Heap dynamically
/*------all data types in rust are immutable by default-----*/
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold Says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (String Slice) 
    // -> immutable
    let sample_string: String = String::from("Hello, World!");
    let slice: &str = &sample_string;
    println!("Slice value: {}", slice);
    let slice: &str = &sample_string[0..5]; // 5 chars
    println!("Slice value: {}", slice);

} 