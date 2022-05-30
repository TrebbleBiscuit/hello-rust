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
    
    let user3 = User {
        email: String::from("dankmemes@example.com"),
        username: String::from("dankmemes"),
        ..user2 
        // the syntax `..` specifies that remaining fields not explicitly set
        // should have the same value as the fields in user2
    }
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

    // Unit-Like Structs - structs with no fields
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    // useful when you need ot implement a trait on some type but have no data
    // to store in the type itself.
}

struct User {
    active: bool,
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
