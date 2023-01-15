fn pig_latinify(word: &str) -> String {
    let mut word_chars = word.chars();
    let first_char = word_chars.next().unwrap();
    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' => format!("{}-hay", word),
        _ => format!("{}-{}ay", word_chars.collect::<String>(), first_char)
    }
}

fn translate_string(sentence: &str) -> String {
    sentence.split_whitespace().map(pig_latinify).collect::<Vec<String>>().join(" ")
}

fn main() {
    println!("{}", translate_string("Hello world, this is a sentence in pig latin"));
}
