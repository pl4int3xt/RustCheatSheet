fn main(){
    let result = reversed_word("hello world");
    println!("{:?}", result);
}

fn reversed_word(word: &str) -> String{
    let answer:Vec<String> = word.split_whitespace()
      .map(|c| c.chars().rev().collect())
      .collect();

    answer.join(" ")
}