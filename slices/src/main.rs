fn main() {
    let mut the_string = String::from("Hello, this is our world!");
    let the_first_word = first_word(&the_string);
    println!(
        "The first word in \"{}\" is \"{}\"",
        the_string, the_first_word
    );
    the_string.clear();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
