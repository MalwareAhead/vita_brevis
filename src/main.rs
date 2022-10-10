use std::env;
use vita_brevis::checks::{requires_subtraction};

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let number: &u16 = &args[1].parse().unwrap();

    if requires_subtraction(&number) {
        resolve_subtraction(&number);
    } else {
        resolve_addition(&number);
    };
}

fn resolve_subtraction(number: &u16) {
    println!("Usual number: {}", &number);
}

fn resolve_addition(number: &u16) {
    println!("Exceptional number: {}", &number);
}
