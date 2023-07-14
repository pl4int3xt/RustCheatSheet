fn main(){
    let result = find_unique_numbers(&[3,5,2,37,5]);
    println!("{:?}", result);
}

fn find_unique_numbers(numbers: &[i32]) -> Vec<i32>{
    let mut vector: Vec<i32> = numbers.iter().copied().collect();
    vector.sort();
    vector.dedup();

    return vector;
}