fn main(){
    println!("{:?}", order_weight("56 65 74 100 99 68 86 180 90"));
}
  
fn order_weight(s: &str) -> String {
    let mut numbers: Vec<&str> = s.split_whitespace().collect();
    numbers.sort();
    numbers.sort_by_key(|s| s.chars().flat_map(|c| c.to_digit(10)).sum::<u32>());
    numbers.join(" ")
}