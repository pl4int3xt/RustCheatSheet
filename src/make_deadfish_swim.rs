fn main(){
    parse("iiisdosodddddiso");
}

fn parse(code: &str) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut value = 0;

    for c in code.chars() {
        match c {
            'i' => value += 1,
            'd' => value -= 1,
            's' => value *= value,
            'o' => result.push(value),
            _ => (),
        }
    }

    result
}