fn twin(first_word: &str, second_word: &str) -> bool{
    let mut sorted_first: Vec<char> = first_word.chars().collect();
    sorted_first.sort();

    let mut sorted_second: Vec<char> = second_word.chars().collect();
    sorted_second.sort();

    if sorted_first == sorted_second {
        return true;
    }

    false
}