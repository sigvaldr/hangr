// Imports
use rand::Rng;
use clearscreen;
use std::io::{self, Write};

// Info
const VERSION: &str="0.0.1";

// Wordlist
const WORD_LIST: [&str; 100] = [
    "adventure", "backyard", "campfire", "diplomat", "euphoria", "freckles", "geometry", "handmade",
    "isolated", "jubilant", "keyboard", "lifetime", "magician", "necklace", "overcome", "paradise",
    "question", "recharge", "sandwich", "together", "umbrella", "vacation", "warhorse", "xenolith",
    "yardstick", "zealotry", "balconies", "cartoons", "dialogue", "elephant", "florists", "grapevine",
    "harmless", "islander", "jumpstart", "knowledge", "landmine", "magazine", "nightmare", "overtake",
    "postcard", "quilters", "radiator", "sapphire", "teaspoon", "underdog", "villager", "waterloo",
    "xylophone", "yearbook", "zippered", "abstract", "bachelor", "calendar", "dinosaur", "eleventh",
    "feedback", "giraffes", "hospital", "intranet", "junction", "keyboard", "laughter", "medicine",
    "narrator", "octangle", "portrait", "question", "rainfall", "skeleton", "triangle", "universe",
    "volcanoe", "whiplash", "xenonate", "yielding", "zoologic", "activity", "backpack", "compound",
    "deserted", "engraver", "fishbowl", "grandson", "hairline", "icebound", "jumpable", "kneecaps",
    "livening", "monogram", "nickname", "obdurate", "panicked", "quadrant", "reformer", "sunshine",
    "tethered", "unworthy", "vintages", "webisode"
];

// Hangman art
const MAN: [&str; 7] = [
    r"  +---+
  |   |
      |
      |
      |
      |
=========",
r"  +---+
  |   |
  O   |
      |
      |
      |
=========",
r"  +---+
  |   |
  O   |
  |   |
      |
      |
=========",
r"  +---+
  |   |
  O   |
 /|   |
      |
      |
=========",
r"  +---+
  |   |
  O   |
 /|\  |
      |
      |
=========",
r"  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
=========",
r"  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
========="

];

// Runtime baby!
fn main() {
    main_menu();
    game_loop(0);
}

// Utils
fn clear() {
    clearscreen::clear().unwrap();
}

fn get_word() -> String {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let i: usize = rng.gen_range(0..=100);
    let w: String = String::from(WORD_LIST[i]);
    return w;
}

fn game_loop(man_count: u8) {
    println!("{}", MAN[man_count as usize]);
    let word: String = get_word();
    println!("Your random word is: {}", word);

    let mut guess = String::new();
    print!("Guess a letter or the full word 🤔: ");
    io::stdout().flush().unwrap(); // Ensures above print statement happens immediately
    io::stdin().read_line(&mut guess).unwrap();
    let guess: String = guess.trim().to_string();
    println!("You guessed: {}", guess);
    
}

fn main_menu () {
    clear();
    println!("Hangr v{}", VERSION);
    println!("By Sigvaldr Notthrafn ©️2024")
}