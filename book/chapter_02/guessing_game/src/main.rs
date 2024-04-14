// Some types are not directly available and need to brought into scope.
// Those which are already available are part of the so called prelude
use rand::Rng;
use std::{cmp::Ordering, io};

// this is the entry into the program
fn main() {
    println!("Guess the number!");
    //1..=100 means from 1 to 100 inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        // Guess is made a String instance from the new() function of the String
        // struct Also all variables are usually immutable, which can be changed
        // with the 'mut' keyword
        let mut guess = String::new();

        io::stdin()
            // read_line is a method of the Stdin struct, which is returned by
            // io::stdin(), see
            // https://doc.rust-lang.org/src/std/io/stdio.rs.html#389
            // I pass '&mut guess' as the argument to read_line. This is a mutable
            // reference of String
            .read_line(&mut guess)
            // expect is a method of Result, which is returned by read_line(), see
            // https://doc.rust-lang.org/src/core/result.rs.html#1024
            .expect("Failed to read line");

        // Reusing a variable is called shadowing
        // We use the value of guess, remove leading and trailing whitespaces with trim(), then
        // convert (parse) the string into the given type u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        // Ordering is an enum. which match we can to pattern matching on every so called arm
        // in match every case MUST be handled
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // this stops the loop
                break;
            }
        }
    }
}
