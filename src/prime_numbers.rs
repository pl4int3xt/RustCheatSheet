fn main(){
    let result = prime_numbers(12);
    println!("{:?}", result);
}

fn prime_numbers(num: i32) -> bool{
    if num <= 1{
        return false;
    }

    for i in 2..=(num/2) {
        if num % i == 0 {
            return false;
        }
    }

    true
}