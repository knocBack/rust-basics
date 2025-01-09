// Structs
// Structs are used to name and package related values; similar to tuples.
#![allow(warnings)]

fn main() {
    // tuple
    let rect: (i32, i32) = (200, 500);

    // Struct
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool
    }

    struct User {
        active: bool,
        username: String,
        email: String, 
        sign_in_count: u64
    }

    let mut user1 = User{
        active: true,
        username: String::from("someusername1"),
        email: String::from("someusername@email.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("newemail@email.com");

    println!("User email is {}", user1.email);


    // Return a struct from a function
    fn built_user(email: String, username: String) -> User{
        User{
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }

    println!("User email: {}", built_user("thisemail@a.com".to_string(), "knocBack".to_string()).email);


    // Create instances from other instances
    let user2: User = User{
        username: String::from("tiktak"),
        ..user1
    };
    println!("User email is {}", user2.email);
    println!("Username is {}", user2.username);

    // Tuple Structs 
    struct Color(i32, i32, i32); // no names are defined, they are considered as structs, but look like tuples 
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let white = Color(255,255,255);
    println!("Tuple Struct: Color = ({}, {}, {})", black.0, black.1, black.2); // this is how to access them
    

    // Unit-Like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;

}