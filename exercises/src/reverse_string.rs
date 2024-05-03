// Instructions: Your task is to reverse a given string. If the input string is "hello", the output should be "olleh".

pub fn reverse(input: &str) -> String {
    let word_reversed: String = input.chars().rev().collect();
    return word_reversed;
}
