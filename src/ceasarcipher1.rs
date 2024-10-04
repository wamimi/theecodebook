// Function to encrypt plaintext using the Caesar cipher
fn caesar_cipher(plaintext: &str, shift: u8) -> String {
    plaintext
        .chars()  // Convert the input string to a character iterator
        .map(|c| {  // Apply a transformation to each character
            if c.is_ascii_lowercase() {  // Check if the character is a lowercase ASCII letter
                // Perform the shift and convert to uppercase
                // 1. Convert char to u8 and subtract 'a' to get 0-25 range
                // 2. Add the shift and use modulo 26 to wrap around the alphabet
                // 3. Add 'A' to convert back to ASCII uppercase range
                let shifted = (c as u8 - b'a' + shift) % 26 + b'A';
                shifted as char  // Convert back to char
            } else {
                c  // If not a lowercase letter, leave it unchanged
            }
        })
        .collect()  // Collect the transformed characters into a String
}

// Function to decrypt ciphertext using the Caesar cipher
fn caesar_decipher(ciphertext: &str, shift: u8) -> String {
    ciphertext
        .chars()  // Convert the input string to a character iterator
        .map(|c| {  // Apply a transformation to each character
            if c.is_ascii_uppercase() {  // Check if the character is an uppercase ASCII letter
                // Perform the reverse shift and convert to lowercase
                // 1. Convert char to u8 and subtract 'A' to get 0-25 range
                // 2. Add 26 to ensure the subtraction doesn't underflow
                // 3. Subtract the shift and use modulo 26 to wrap around the alphabet
                // 4. Add 'a' to convert back to ASCII lowercase range
                let shifted = (c as u8 - b'A' + 26 - shift) % 26 + b'a';
                shifted as char  // Convert back to char
            } else {
                c  // If not an uppercase letter, leave it unchanged
            }
        })
        .collect()  // Collect the transformed characters into a String
}

fn main() {
    // Define the plaintext message
    let plaintext = "the quick brown fox jumps over the lazy dog";
    // Define the shift value (key)
    let shift = 3;

    // Encrypt the plaintext
    let ciphertext = caesar_cipher(plaintext, shift);
    // Print the original plaintext
    println!("Plaintext:  {}", plaintext);
    // Print the encrypted ciphertext
    println!("Ciphertext: {}", ciphertext);

    // Decrypt the ciphertext
    let deciphered = caesar_decipher(&ciphertext, shift);
    // Print the decrypted text (should match the original plaintext)
    println!("Deciphered: {}", deciphered);
}