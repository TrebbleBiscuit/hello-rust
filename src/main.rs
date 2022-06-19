use std::env;
use rand::seq::SliceRandom;

fn main() {
    println!("Hello, world");
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    let valid_choices = vec!["rock", "paper", "scissors"];
    let ai_choice = valid_choices.choose(&mut rand::thread_rng()).unwrap();
    println!("{}", ai_choice);

    let choice = &args[1].trim().to_lowercase();
    match choice.as_str() {
        "rock" | "paper" | "scissors" => println!("valid choice"),
        _ => println!("invalid choice oh no oh no")
    }
}
