fn main(){
    let mut input = vec![5, 8, 6, 3, 4];
    let mut odd_numbers: Vec<_> = input.iter().filter(|&n| n % 2 == 0).copied().collect();
    odd_numbers.sort();

    let mut index = 0;

    for i in 0..input.len(){
        if input[i] % 2 == 0{
            input[i] = odd_numbers[index];
            index += 1;
        }
    }

    println!("{:?}", input);
}