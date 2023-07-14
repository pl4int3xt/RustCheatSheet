fn main(){
    let result = palindrome("level");
    println!("{:?}", result);
}

fn palindrome(word: &str) -> bool{
    let characters: Vec<char> = word.chars().rev().collect();
    let reversed_word: String = characters.iter().collect();

    if word == reversed_word{
        true
    } else {
        false
    }
}