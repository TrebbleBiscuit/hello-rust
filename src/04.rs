fn main() {
    let hi = "Hello world!";
    println!("{}", rm_first_three_chars(hi));
    println!("{}", first_word(hi));
    println!("{}", first_word("Hellooo!"));
    let my_array = [0, 1, 2, 3];
    // use {:?} to print the result of the debug trait of a struct (in this case, the array)
    // only works if the inner elements also implement the debug trait
    // will not work if array length is more than 32
    println!("First two elements of {:?} are {:?}", my_array, first_two_elements(&my_array));
}

fn rm_first_three_chars(instr: &str) -> &str {
    let slice = &instr[2..];
    slice
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    // if there aren't any spaces just give the word back
    return s
}

fn first_two_elements(arr: &[i32]) -> &[i32] {
    // slices have a start index and a length
    // this is not an end index!
    &arr[0..2]
} 
