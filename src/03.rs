fn main() {
    println!("Hello world!");
    some_phrase("red", "apple");
    println!("Five plus one is: {}", add_one(5));
    println!("'random' number is {}", random_number(false));
    let ten_count = count_to_ten();
    println!("Count after counting to ten was {}", ten_count);
    count_to_ten_better();
    multiple_loops();
    try_a_for_loop();
    count_to_ten_even_better();
}

// you must declare types in function signatures
fn some_phrase(adj: &str, noun: &str) {
    println!("{} {}", adj, noun)
}

// return values
fn add_one(x: i32) -> i32 {
    if x > 4 {
        println!("this sure is a number greater than 4 we're adding one to!");
    } else if x < 0 {
        println!("wow a negative number very cool");
    } else {
        println!("small positive integer? what a bold choice");
    }
    x + 1  // this expression has no semicolon because we return its value
}

fn random_number(negative: bool) -> i32 {
    // chosen by fair dice roll
    let number = if negative { -4 } else { 4 };  // these expressions must return the same type
    number
}

fn count_to_ten() -> i32 {
    let mut count = 0;
    loop {
        println!("Current count is {}", count);
        count += 1;
        if count > 10 {
            break count;  // adding an expression after break makes the loop return that expression
        }
    }
}

fn count_to_ten_better() {
    let mut count = 0;
    while count < 10 {
        count += 1;
        println!("Better count is {}", count);
    }
    println!("Done counting :)")
}

fn multiple_loops() {
    let mut count = 0;
    'counting_up: loop {  // give the loop a label so we can apply `break` or `continue` to the outer loop from the inner loop
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn try_a_for_loop() {
    let mylist = [0, 1, 2, 3];
    for element in mylist {
        println!("{} is in mylist", element)
    }
}

fn count_to_ten_even_better() {
    for count in 1..10 {
        println!("Even better count is {}", count)
    }
}
