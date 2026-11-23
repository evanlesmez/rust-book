pub fn median(numbers: &[i32]) -> f64 {
    let mut nums = numbers.to_vec();
    nums.sort();
    let len = nums.len();
    if len % 2 == 1 {
        return nums[len / 2] as f64;
    } else {
        let sum = nums[(len / 2) - 1] + nums[len / 2];
        sum as f64 / 2.0
    }
}

pub fn mode(numbers: Vec<i32>) {}
