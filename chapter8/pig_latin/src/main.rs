use std::io;


// TODO: let user input a series of words, seperated by spaces
fn main() {
    let mut words = String::new();
    println!("Enter a series of words to be translated to pig-latin:");
    io::stdin()
        .read_line(&mut words)
        .expect("Failed to read line!");
    words = words.trim().to_string();
    let mut words_pl = Vec::new();
    for word in words.split_whitespace() {
        words_pl.push(word_to_pig_latin(&word));
    }
    println!("Old: {}", words);
    println!("New: {}", words_pl.join(" "));
}

fn word_to_pig_latin(word: &str) -> String {
    let word_pl;
    let mut chars = word.chars();
    if let Some(c) = chars.next() {
        if is_vowel(c) {
            word_pl = format!("{}-hay", word);
        } else {
            word_pl = format!("{}-{}ay", chars.as_str(), c);
        }
    } else {
        panic!("Empty string given");
    }

    word_pl
}

fn is_vowel(c: char) -> bool {
    if let Some(lower_c) = c.to_lowercase().next() {
        lower_c == 'a'
            || lower_c == 'e'
            || lower_c == 'i'
            || lower_c == 'o'
            || lower_c == 'u'
    } else {
        panic!("No lowercase for given char")
    }
}
