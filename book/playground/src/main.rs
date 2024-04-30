use std::u8;

struct Pokemon {
    name: String,
    level: u8,
    pokemon_type: String,
    hp: u8,
}

impl Pokemon {
    fn print_pokemon(&self) {
        println!("Name: {}", self.name);
        println!("Level: {}", self.level);
        println!("Type: {}", self.pokemon_type);
        println!("HP: {}", self.hp);
    }

    // Changing name makes no sense for Pokemon but it's for learning. (:
    fn change_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn increase_level(&mut self) {
        self.level += 1;
    }

    fn decrease_hp(&mut self, drain: u8) {
        self.hp -= drain;
    }
}

fn main() {
    let mut pikachu = Pokemon {
        name: String::from("Pikachu"),
        level: 25,
        pokemon_type: String::from("Electric"),
        hp: 100,
    };
    print_pokemon(&pikachu);
    pikachu.print_pokemon();
    pikachu.change_name(String::from("Pika"));
    pikachu.print_pokemon();
    pikachu.increase_level();
    pikachu.print_pokemon();
    pikachu.decrease_hp(20);
    pikachu.print_pokemon();
}

fn print_pokemon(p: &Pokemon) {
    println!("Name: {}", p.name);
    println!("Level: {}", p.level);
    println!("Type: {}", p.pokemon_type);
}

