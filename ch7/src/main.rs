fn printsep() {
    println!("===============================");
}

fn main() {
    let args = [Some(3u8), None];
    println!("Match pattern");
    for arg in args {
        match arg {
            Some(v) => println!("The arg is {}", v),
            _ => println!("No arg passed"),
        }
    }
    printsep();
    println!("If let Some(v) = Option<T> pattern");
    for arg in args {
        if let Some(v) = arg {
            println!("The arg is {}", v);
        } else {
            println!("No arg passed");
        }
    }
    printsep();
    let mut count = 0;
    println!("let Some(v) = Option<T> else pattern");
    for arg in args {
        let Some(v) = arg else {
            println!("No arg passed");
            continue;
        };
        println!("The arg is {}", v);
    }
}
