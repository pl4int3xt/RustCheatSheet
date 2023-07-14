
fn main(){
    let result = find_min_max(&[4334,356543,6456646,46654,5677547,756]);
    println!("{:?}", result);
}

fn find_min_max(numbers: &[i32]) -> Option<(i32, i32)>{
    if numbers.is_empty(){
        return None;
    }

    let mut min = numbers[0];
    let mut max = numbers[0];

    for &num in numbers.iter(){
        if num < min {
            min = num
        }

        if num > max {
            max = num
        }
    }

    Some((min, max))
}