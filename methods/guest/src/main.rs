use risc0_zkvm::guest::env;
use std::string::String;
use serde::{Deserialize, Serialize}; // Import serialize and deserialize macros


#[derive(Debug, Serialize, Deserialize)]
struct Character {
    id: u32,
    name: String,
}

// Function to check if a character loves the enemy
fn check_character_love(characters: &[Character]) -> u32 {
    for character in characters {
        if character.id == 3 && character.name == "Belle" {
            return 1; // Found love
        }
    }
    2 // No love found
}

fn main() {
    // Read the vector of Character structs from the input
    let received_characters: Vec<Character> = env::read();

    // Call the function to check if Belle is in the vector with id 3
    let is_love = check_character_love(&received_characters);

    // Write public output to the journal
    env::commit(&is_love);
}
