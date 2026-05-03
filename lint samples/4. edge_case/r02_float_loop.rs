fn count_steps(step: f64) -> u32 {
    let mut x = 0.0;
    let mut count = 0;
    while x != 1.0 {
        x += step;
        count += 1;
    }
    count
}
