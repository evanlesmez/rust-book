fn main() {
    let mut s = String::from("yo yaw yow");
    let w = first_word(&s);
    println!("{w}");
}

fn mutstring() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
}

fn strlenbyref() {
    let s1 = String::from("Hello");
    let len = strlen(&s1);
    println!("Length of '{s1}' is {len}");
}

fn strlen(s: &String) -> usize {
    s.len()
}

fn mutstr() {
    let mut s = String::from("Hello");

    changestr(&mut s);
    println!("{s}");
}

fn changestr(s: &mut String) {
    s.push_str(", world");
}

fn shislice() {
    let s = String::from("Alo alo");
    let full_slice = &s[..s.len()];
    let full_slice = &s[..];
    let early_slice = &s[..2];
    let late_slice = &s[3..];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
