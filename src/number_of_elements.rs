use std::collections::HashMap;
fn main(){
    let words = ";lskdjgahfkljagfadlkfhsadlhfsdaj fsilhasfhilfaAAAAAAAAA";
    let mut count_words: HashMap<char, i32> = HashMap::new();
    let new_words = words.to_lowercase(); 

    for c in new_words.chars(){
        if c.is_whitespace(){
            continue;
        }

        let counter = count_words.entry(c).or_insert(0);
        *counter += 1;
    }

    println!("{:?}", count_words);
}