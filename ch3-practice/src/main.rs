fn main() {
    let temps_c = [0.0, 10.0, 50.0, 100.0];
    for t in temps_c {
        let temp_f = ctof(t);
        println!("{t} C == {temp_f} F");
    }

    let fibs = [1, 2, 3, 4, 5, 6, 7];
    for f in fibs {
        let ff = fib(f);
        println!("{f} number of fib is {ff}");
    }
}

fn ctof(temp_c: f64) -> f64 {
    return temp_c * 1.8 + 32.0;
}

fn fib(n: i32) -> i32 {
    if n < 3 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}
