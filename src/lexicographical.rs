fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = arr_a.iter()
        .filter(|&e| arr_b.iter().any(|&t| t.contains(e)))
        .map(|s| s.to_string())
        .collect();
    
    result.sort_unstable();
    result.dedup();
    result
}