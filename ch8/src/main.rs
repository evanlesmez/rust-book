fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let third: &i32 = &v3[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let first = &v3[0];
    // immutable borrow error v3.push(9);
    println!("The first element is {first}");

    for i in &mut v3 {
        *i += 50;
        println!("{i}");
    }
}
