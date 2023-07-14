fn main(){
    let result = merge(&[1,2,3,4,5,6], 2, &[3,5,7,8,9], 4);
    println!("{:?}", result);
}

fn merge(num1: &[i32], m: usize, num2: &[i32], n: usize) -> Vec<i32>{
    let mut new_num1: Vec<i32> = Vec::new();
    let mut new_num2: Vec<i32> = Vec::new();

    for (index, &value) in num1.into_iter().enumerate(){
        if index < m {
            new_num1.push(value);
        }
    }

    for (index, &value) in num2.into_iter().enumerate(){
        if index < n {
            new_num2.push(value);
        }
    }

    new_num1.extend(new_num2);
    new_num1
}