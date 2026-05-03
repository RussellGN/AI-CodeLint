fn count_negatives(nums: &[i32]) -> i32 {
    nums.iter()
        .fold(0, |acc, &x| if x < 0 { acc } else { acc + 1 })
}
