use std::io;

fn main() {
    let mut vowels = 0;
    let mut consonants = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let ch = match input.trim().chars().next() {
            Some(ch) => ch,
            None => break,
        };

        if !ch.is_alphabetic() {
            break;
        }

        if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' ||
           ch == 'A' || ch == 'E' || ch == 'I' || ch == 'O' || ch == 'U' {
            vowels += 1;
        } else {
            consonants += 1;
        }
    }

    println!("Number of vowels: {}", vowels);
    println!("Number of consonants: {}", consonants);
}
