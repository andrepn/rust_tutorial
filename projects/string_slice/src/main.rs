fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn next_word<'s>(last_word: &str, s: &'s str) -> &'s str {
    let next_slice = &s[last_word.len() + 1..];
    let bytes = next_slice.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[last_word.len() + 1..i + last_word.len()];
        }
    }
    return next_slice;
}

fn main() {
    let das_string = String::from("Hello Earthling");

    let word_one = first_word(&das_string);

    let word_two = next_word(word_one, &das_string);

    println!("The first word is: {}", word_one);
    println!("The second word is: {}", word_two);
}
