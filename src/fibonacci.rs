fn main(){
    let result = fibonacci(10);
    println!("{:?}", result);
}

fn fibonacci(num: i32) -> i32{
  if num == 0{
    return 0;
  } else if num == 1 {
      return 1;
  }

  let mut fib = (0, 1);

  for _ in 2..=num {
    fib = (fib.1, fib.0 + fib.1);
  }

  fib.1
}