pub fn median(numbers: &Vec<i32>) {
    numbers.sort();
    let i = numbers.len() / 2;
    numbers[i]
}

pub fn mode(numbers: Vec<i32>) {}
