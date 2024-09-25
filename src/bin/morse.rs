use itertools::Itertools;
use std::collections::HashMap;
use std::io::Write;
use std::iter::repeat;

fn main() {
    let morse_rep = HashMap::from([
        ('A', ".-"),
        ('B', "-..."),
        ('C', "-.-."),
        ('D', "-.."),
        ('E', "."),
        ('F', "..-."),
        ('G', "--."),
        ('H', "...."),
        ('I', ".."),
        ('J', ".---"),
        ('K', "-.-"),
        ('L', ".-.."),
        ('M', "--"),
        ('N', "-."),
        ('O', "---"),
        ('P', ".--."),
        ('Q', "--.-"),
        ('R', ".-."),
        ('S', "..."),
        ('T', "-"),
        ('U', "..-"),
        ('V', "...-"),
        ('W', ".--"),
        ('X', "-..-"),
        ('Y', "-.--"),
        ('Z', "--.."),
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
        (' ', "/"),
    ]);

    let mut message = String::new();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    print!("Enter text to be encoded: ");
    stdout.flush().expect("Failed to flush output buffer");

    stdin
        .read_line(&mut message)
        .expect("Failed to read message");
    let message = message.trim().to_uppercase();
    println!("Plain message: {}", message);

    let spaces = repeat(" ");
    let encoded: String = message
        .chars()
        .map(|c| morse_rep[&c])
        .interleave_shortest(spaces)
        .collect();
    let encoded = encoded.trim();

    println!("Encoded message: {}", encoded);
}
