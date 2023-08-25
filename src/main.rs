mod keymap;

use std::env;

fn main() {
    let text = env::args().nth(1).unwrap_or(String::new());
    let mut conv_text: String = String::new();

    for x in text.chars() {
        conv_text.push_str(keymap::KEYMAP.get(&x).unwrap_or(&""));
    }

    println!("{}", conv_text)
}