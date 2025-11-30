use types::StrExt;
fn main() {
    

    let text = "racecar";

    println!("Is palindrome: {}", text.is_palindrome());
    println!("Capitalized: {}", text.capitalize());
    println!("Reversed: {}", text.reverse());
    println!("Reverse words: {}", "hello world from rust".reverse_words());
    println!("Count of 'l': {}", "hello".count('l'));
}