fn main(){
    let result = find_min_max(&[4334,356543,6456646,46654,5677547,756]);
    println!("{:?}", result);
  }
  
  fn find_min_max(numbers: &[i32]) -> Option<(i32, i32)>{
    if numbers.is_empty(){
        return None;
    }
    let max = numbers.iter().max();
    let min = numbers.iter().min();
    Some((*min.unwrap(), *max.unwrap()))
  }