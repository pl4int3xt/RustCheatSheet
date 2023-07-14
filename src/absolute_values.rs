fn main(){
    let result = replace_negative_with_absolute(&[3,-5,-2,37,-6,-10]);
    println!("{:?}", result);
}

fn replace_negative_with_absolute(numbers: &[i32]) -> Vec<i32>{
    let answer: Vec<i32> = numbers.iter()
      .map(|&num| num.abs())
      .collect();
    
    answer
}