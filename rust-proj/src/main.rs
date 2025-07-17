fn main() {
   // String Example
   let greetings: String = String::from("Hello, world!");
   println!("{}", greetings);

    // Print last character using pattern matching
    let last_char: Option<char> = greetings.chars().last();
    match last_char {
        Some(c) => println!("The last character is '{}'", c),
        None => println!("The string is empty"),
    }

    let first_word: String = get_first_word(greetings);
    println!("The first word is '{}'", first_word);
}


fn get_first_word(sentence: String) -> String {
    let mut first_word: String = String::from("");
    // print first word
    for i in sentence.chars() {
        if i == ' ' || i == ','{
            break;
        }
        first_word.push(i);
    }
    return first_word;
}
