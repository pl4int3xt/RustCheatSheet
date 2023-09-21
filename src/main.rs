fn main(){
    let result = palindrome("level");
    println!("{:?}", result);
}

fn palindrome(word: &str) -> bool{
    let characters: String = word.chars().rev().collect();

    if word == characters{
        true
    } else {
        false
    }
}