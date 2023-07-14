fn main(){
    let result = factorial(10);
    println!("{:?}", result);
}

fn factorial(num: i32) -> i32{
  if num == 0{
    return 0;
  }

  let mut result = 1;

  for i in 1..=num{
    result *= i;
  }

  result
}