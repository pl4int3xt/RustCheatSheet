fn main(){
    let result = palindrome("level");
    println!("{:?}", result);
}

fn palindrome(word: &str) -> bool{
    let rev_word: String = word.chars().rev().collect();

    if word == rev_word{
        true
    } else {
        false
    }
}