fn main(){
  create_phone_number(&[1,2,3,4,5,6,7,8,9,0]);
}

fn create_phone_number(numbers: &[u8]) -> String {
  let mut phone_number = String::new();

  let first_space: &[u8] = &numbers[0..3];
  let second_space: &[u8] = &numbers[3..6];
  let third_space: &[u8] = &numbers[6..];

  
  println!("{:?}", third_space);
  phone_number
}