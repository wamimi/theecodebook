fn shift_char(c: char, base: u8, shift: u8) -> char {
    // Convert char to u8, apply shift, and convert back to char
    let shifted = (c as u8 - base + shift) % 26 + base;
    shifted as char
}

fn caesar_cipher(plaintext: &str, shift: u8) -> String {
    plaintext
        .chars()
        .map(|c| match c {
            'a'..='z' => shift_char(c, b'a', shift).to_ascii_uppercase(),
            _ => c,  // Non-lowercase characters remain unchanged
        })
        .collect()
}

fn caesar_decipher(ciphertext: &str, shift: u8) -> String {
    ciphertext
        .chars()
        .map(|c| match c {
            'A'..='Z' => shift_char(c, b'A', 26 - shift).to_ascii_lowercase(),
            _ => c,  // Non-uppercase characters remain unchanged
        })
        .collect()
}

fn main() {
    let plaintext = "the quick brown fox jumps over the lazy dog";
    let shift = 3;

    let ciphertext = caesar_cipher(plaintext, shift);
    println!("Plaintext:  {}", plaintext);
    println!("Ciphertext: {}", ciphertext);

    let deciphered = caesar_decipher(&ciphertext, shift);
    println!("Deciphered: {}", deciphered);
}
