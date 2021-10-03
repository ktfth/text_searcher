use std::env;

fn main() {
    println!("Hello, world!");
    for argument in env::args() {
        println!("{}", argument);
    }
}
