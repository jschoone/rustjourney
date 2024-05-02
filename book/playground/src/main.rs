use std::u8;

enum PokemonType {
    Electric,
    Water,
}

struct Pokemon {
    name: String,
    level: u8,
    pokemon_type: PokemonType,
    hp: u8,
}

impl Pokemon {
    fn print_pokemon(&self) {
        println!("Name: {}", self.name);
        println!("Level: {}", self.level);
        println!("Type: {}", self.print_pokemon_type());
        println!("HP: {}", self.hp);
    }

    fn print_pokemon_type(&self) -> String {
        match &self.pokemon_type {
            PokemonType::Electric => String::from("Electric"),
            PokemonType::Water => String::from("Water"),
        }
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

    fn new(name: String, level: u8, pokemon_type: PokemonType, hp: u8) -> Self {
        Pokemon {
            name,
            level,
            pokemon_type,
            hp,
        }
    }
}

fn main() {
    let mut pikachu = Pokemon {
        name: String::from("Pikachu"),
        level: 25,
        pokemon_type: PokemonType::Electric,
        hp: 100,
    };
    let shiggy = Pokemon::new(String::from("Shiggy"), 10, PokemonType::Water, 30);
    print_pokemon(&pikachu);
    pikachu.print_pokemon();
    pikachu.change_name(String::from("Pika"));
    pikachu.print_pokemon();
    pikachu.increase_level();
    pikachu.print_pokemon();
    pikachu.decrease_hp(20);
    pikachu.print_pokemon();
    shiggy.print_pokemon();
}

fn print_pokemon(p: &Pokemon) {
    println!("Name: {}", p.name);
    println!("Level: {}", p.level);
    println!("HP: {}", p.hp)
}
