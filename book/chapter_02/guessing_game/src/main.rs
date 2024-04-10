// Some types are not directly available and need to brought into scope.
// Those which are already available are part of the so called prelude
use std::io;

// this is the entry into the program
fn main() {
    println!("Guess the number!");

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

    println!("You guessed: {guess}");
}
