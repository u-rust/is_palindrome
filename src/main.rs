use std::io;

fn main() {
    println!("Введите строку:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
    let input = input.trim();
    println!("{}: {}", input, is_palindrome(input));
}

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut start = 0;
    let mut end = chars.len() - 1;

    while start < end {
        if chars[start] != chars[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}