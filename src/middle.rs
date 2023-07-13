fn main(){
    let word = "";

    if word == ""{
        println!("empty string");
    } else {
        let middle = &word[(word.len()-1)/ 2 .. (word.len() + 2) / 2];
        println!("{}", middle);
    }
}