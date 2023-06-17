pub fn encrypt(alphabet: &str, plain_text: &str, key: i32) -> String {
    let alphabet_len: i32 = alphabet.len() as i32;
    let key: i32 = (key).rem_euclid(alphabet_len);
    shift(alphabet, plain_text, key)
}

pub fn decrypt(alphabet: &str, cipher_text: &str, key: i32) -> String {
    let alphabet_len: i32 = alphabet.len() as i32;
    let key: i32 = (alphabet_len - key).rem_euclid(alphabet_len);
    shift(alphabet, cipher_text, key)
}

pub fn shift(alphabet: &str, input: &str, key: i32) -> String {
    let alphabet_len: i32 = alphabet.len() as i32;
    let mut output: Vec<char> = Vec::with_capacity(input.len());

    for char in input.chars() {
        let index: Option<usize> = alphabet.chars().position(|c| c == char);

        let char: char = if let Some(index) = index {
            let nth: usize = (index as i32 + key).rem_euclid(alphabet_len) as usize;
            alphabet.chars().nth(nth).unwrap()
        } else {
            char
        };

        output.push(char);
    }

    output.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::shift::{decrypt, encrypt};

    #[test]
    fn it_works() {
        let alphabet: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ 123456789";
        let key: i32 = 120000;
        let message: &str = "Hi Maryam";

        let encrypted: String = encrypt(alphabet, message, key);
        println!("{}", encrypted);

        let decrypted: String = decrypt(alphabet, &encrypted, key);
        println!("{}", decrypted);
    }
}