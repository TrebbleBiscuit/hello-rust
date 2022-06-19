fn main() {
    println!("Hello, world");
    let mut mygame = Game{in_progress: true, time: 0};
    mygame.progress_time(1);
    mygame.progress_time(5);
    // equivalent syntax to (&mut mygame).progress_time(5);
    // that is, rust automagically adds `&`, `&mut`, or `*` so that
    // the object matches the signature of the method
    mygame.end_game();
}

struct Game {
    in_progress: bool,
    time: i32,
}

impl Game {
    /* `&self` is a reference to the struct that implements this trait
       it is shorthand for `self: &Self`
       methods can take ownership of self and borrow self (mutably or not)
       just like any other parameter
    */
    fn progress_time(&mut self, time: i32) {
        println!("Time inches ever forward...");
        self.time += time;
    }

    fn end_game(&mut self) {
        self.in_progress = false;
        println!("Game over at t={}", self.time);
    }
}
