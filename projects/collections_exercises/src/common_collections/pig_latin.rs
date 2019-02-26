pub fn pig_latinize(s: &str) -> String {
    let word_list = s.split_whitespace();
    let vowels = String::from("aeiouAEIOU");
    let mut new_string = String::new();

    for word in word_list {
        let list_of_chars: Vec<char> = word.chars().collect();

        if vowels.contains(list_of_chars[0]) {
            let prefix = word;
            let suffix = "-hay ";
            let new_word = format!("{}{}", prefix, suffix);
            new_string += &new_word;
        } else {
            let prefix = &word[1..];
            let suffix = format!("-{}ay ", list_of_chars[0]);
            let new_word = format!("{}{}", prefix, suffix);
            new_string += &new_word;
        };
    }
    return new_string;
}
