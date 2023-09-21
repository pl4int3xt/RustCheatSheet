use std::collections::HashMap;

fn main(){
    println!("{:?}", letter_count("lskdjgahfkljagfadlkfhsadlhfsdaj fsilhasfhilfaAAAAAAAAA"));
}
  
fn letter_count(s: &str) -> HashMap<char, i32>{
    let s: Vec<char> = s.chars().flat_map(|c| c.to_uppercase()).collect::<Vec<char>>();
    let mut result: HashMap<char, i32> = HashMap::new();

    for c in s.iter(){
        if c.is_whitespace(){
            continue;
        }
        let count = result.entry(*c).or_insert(0);
        *count += 1;
    }

    result
}