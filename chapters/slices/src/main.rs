fn main() {
    let multi_word_string = String::from("hello world");
    let a = first_word(&multi_word_string);
    println!("{}", a);
}


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
