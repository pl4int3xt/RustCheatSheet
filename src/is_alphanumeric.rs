fn main(){
    println!("{:?}",alphanumeric(""));
}

fn alphanumeric(password: &str) -> bool {
    !password.is_empty() && password.chars().all(|c| c.is_alphanumeric())
}