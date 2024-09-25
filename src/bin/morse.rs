use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let morse_rep = HashMap::from([
        ('a', ".-"),
        ('b', "-..."),
        ('c', "-.-."),
        ('d', "-.."),
        ('e', "."),
        ('f', "..-."),
        ('g', "--."),
        ('h', "...."),
        ('i', ".."),
        ('j', ".---"),
        ('k', "-.-"),
        ('l', ".-.."),
        ('m', "--"),
        ('n', "-."),
        ('o', "---"),
        ('p', ".--."),
        ('q', "--.-"),
        ('r', ".-."),
        ('s', "..."),
        ('t', "-"),
        ('u', "..-"),
        ('v', "...-"),
        ('w', ".--"),
        ('x', "-..-"),
        ('y', "-.--"),
        ('z', "--.."),
        ('1', ".----"),
        ('2', "..---"),
        ('3', "...--"),
        ('4', "....-"),
        ('5', "....."),
        ('6', "-...."),
        ('7', "--..."),
        ('8', "---.."),
        ('9', "----."),
        ('0', "-----"),
        (',', "--..--"),
        ('?', "..--.."),
        (':', "---..."),
        ('&', ".-..."),
    ]);

    let mut message = String::new();

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    print!("Enter text to be encoded: ");
    stdout.flush().expect("Failed to flush output buffer");
    stdin
        .read_line(&mut message)
        .expect("Failed to read message");
    let message = message.trim().to_lowercase();
    println!("Plain message: {}", message);
    let encoded: String = message.chars().map(|c| morse_rep[&c]).collect();
    println!("Encoded message: {}", encoded)
}
