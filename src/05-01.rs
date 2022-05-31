fn main() {
    println!("Hello, world!");
    // if the instance is mutable, ALL of its values will be mutable
    let mut user1 = User {
        email: String::from("desu@example.com"),
        username: String::from("kawaiigirl420"),
        active: true,
        sign_in_count: 1,
    };

    user1.active = false;

    let user2 = build_user(String::from("xdddd@example.com"), String::from("hotboy45"));
    
    let msg1 = "Here's how to make debug messages";
    let msg2 = "This will print to stderr instead of stdout";
    dbg!(msg1);
    dbg!(msg2);
    // You can also print structs with them and it's like doing "{:?}""
    dbg!(&user2);  // still use &; we don't want to take ownership

    let user3 = User {
        email: String::from("dankmemes@example.com"),
        username: String::from("dankmemes"),
        ..user2 
        // the syntax `..` specifies that remaining fields not explicitly set
        // should have the same value as the fields in user2
    };
    // struct update syntax uses `=` like assignment because it moves data.
    // if we didn't specify both email and username, the corresponding String
    // from user2 would be moved into user3, and we would no longer be able
    // to use user2. bool and int32 are copied by value, so they are not moved.

    // Tuple Structs - tuples with a name
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // a fn that takes a parameter of type Color can not take one of type Point
    // even though they're both made up of three i32s. Otherwise they're just tuples.

    // unpack with the type name, like this
    let Color(_a, _b, _c) = black;
    let Point(_a, _b, _c) = origin;
    // underscores before these variables or the compiler will warn that they're unused

    // Unit-Like Structs - structs with no fields
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    // useful when you need ot implement a trait on some type but have no data
    // to store in the type itself.
}

#[derive(Debug)]  // so that we can print debug info about this struct
struct User {
    active: bool,
    // using the owned String type instead of the &str string slice type
    // because we want each instance of this struct to own all of its data
    // and for it to be valid as long as the struct as valid
    // could store references to data owned by something else but this
    // requires use of lifetimes
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    // as with any expression, we can construct a new instance of a struct as the
    // last expression in the function body to implicitly return that new instance
    User {
        // because these parameter names and struct field names are the same, we can
        // use the field init shorthand syntax to avoid typing `email: email`, etc.
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
