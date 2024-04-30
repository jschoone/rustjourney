fn main() {
    let pikachu_name = String::from("Pikachu");
    print_pokemon(pikachu_name);
    // print_pokemon(pikachu_name); // Since pikachu_name has been moved to print_pokemon this is
    // no longer available
    let pikachu_name = String::from("Pikachu");
    print_pokemon_reference(&pikachu_name);
    print_pokemon_reference(&pikachu_name); // this is possible because only the reference is
                                            // passed to the function

    print_pokemon(pikachu_name.clone());
    print_pokemon(pikachu_name.clone()); // this works also because the value will be cloned before
                                         // passing to the function
}

fn print_pokemon(s: String) {
    println!("Name: {}", s);
}

fn print_pokemon_reference(s: &String) {
    println!("Name: {}", s);
}
