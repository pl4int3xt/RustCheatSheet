// fn main() {
//     let mut word = "testi";

//     let len = word.len();

//     if len % 2  == 0 {
//         word = &word[len / 2 -1..len / 2]; 
//     } else {
//         word = &word[len / 2 - 1..len / 2 + 1];
//     }
//     println!("{}",word);
// }

use std::collections::BTreeMap;
fn main(){
    let sentence = "4of Fo1r pe6ople g3ood th5e the2";
    let words: Vec<_> = sentence.split_whitespace().collect();
    let mut values: Vec<&str> = Vec::new();
    let mut key: Vec<u32> = Vec::new();
    let mut word_btree_map: BTreeMap<u32, &str> = BTreeMap::new();

    for word in words.into_iter(){
        values.push(word);
        for char in word.chars().into_iter(){
            if char.is_digit(10){
                key.push(char.to_digit(10).unwrap());
            }
        }
    }

    word_btree_map = key.into_iter().zip(values.into_iter()).collect();

    let answer: String = word_btree_map.values().cloned().collect::<Vec<_>>().join(" ");
    println!("{:?}", answer);

}