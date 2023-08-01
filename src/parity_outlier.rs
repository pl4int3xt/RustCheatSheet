fn find_outlier(values: &[i32]) -> i32 {
    let even: Vec<&i32> = values.into_iter()
                    .filter(|&x| x % 2 == 0)
                    .collect();
    let odd: Vec<&i32> = values.into_iter()
                    .filter(|&x| x % 2 != 0)
                    .collect();
    
    if even.len() > odd.len(){
        return *odd[0];
    }

    return *even[0];
}
