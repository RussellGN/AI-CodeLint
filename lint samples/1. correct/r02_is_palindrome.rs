fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    chars == chars.iter().rev().cloned().collect::<Vec<_>>()
}
