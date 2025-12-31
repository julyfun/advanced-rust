use std::fmt::Display;
fn print<T: Display>(item: T) {
    println!("{item}");
}

fn main() {
    let s = "Hello, world!".to_string();
    print(s);
}

