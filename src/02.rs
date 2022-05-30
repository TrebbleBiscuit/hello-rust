fn main() {
    let mut name = "Sam";
    println!("My name is {}", name);
    name = "Faye";  // we can do this because we declared it mutable
    println!("Now my name is {}", name);

    // constants can not be the result of a value computed at runtime
    // but they can be a constant expression evaluated at compile time
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("There are {} seconds in three hours.", THREE_HOURS_IN_SECONDS);

    let noun = "Apple";
    println!("My noun is {}", noun);
    let noun = 33;  // we can "shadow" a variable to a different type
    println!("Now my noun is {}", noun);
    // in contrast, one cannot mutate the type of a mutable variable

    // Tuples
    let tup: (str, str, f64) = ("red", "green", "1993.42");
    // destructuring
    let (x, y, z) = tup;
    // or we could use indexes
    let primary_color = tup.0;
    let secondary_color = tup.1;
    let favorite_number = tup.2;

    let special_type = ();
    // this is a tuple without any values, a special type (the "unit type")
    // that has only one value, also written `()` (the "unit value")
    // expressions implicitly return the unit value if they don't return any other valueA

    // Arrays
    // every element of an array must have the same type
    let arr = [0, 1, 2, 3];
    // arrays have a fixed length
    // useful when you want data allocated on the stack rather than the heap (whatever tf that means???)
    // if you want variable lengths, use a vector instead
    // write the type annotation with square branckets, the type, a semicolon, then the length
    let arr2: [i32; 3] = [1, 2, 3];
    let arr3 = [1; 5];  // is like writing arr3 = [1, 1, 1, 1, 1]
    // access array elements with indexing
    let first = arr[0];
    let second = arr[1];
}
