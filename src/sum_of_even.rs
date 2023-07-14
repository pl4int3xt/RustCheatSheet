fn main(){
    let result = sum_of_even_numbers(&[3,5,2,37,5,6,10]);
    println!("{:?}", result);
}

fn sum_of_even_numbers(numbers: &[i32]) -> i32{
    let mut sum = 0;

    for i in numbers.iter(){
        if i % 2 == 0{
            sum += i
        }
    }

    sum
}