use std::collections::BTreeSet;

fn main(){
    let word = "AAAAABBBBBCCCDDDEEEEFFFF";
    let answer: Vec<char> = word.chars().collect();
    let unique_chars: BTreeSet<_> = answer.into_iter().collect();
    let result: String = unique_chars.into_iter().collect();

    println!("{:?}", result);
}