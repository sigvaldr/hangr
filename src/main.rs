// Imports
use rand::Rng;

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

fn main() {
    println!("Hangr v{}", VERSION);

    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..=100);
    println!("Your random word is: {}", WORD_LIST[i]);
}
