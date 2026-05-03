fn first_words(lines: &[&str]) -> Vec<&str> {
    lines.iter().map(|l| l.split_whitespace().next().unwrap()).collect()
}
