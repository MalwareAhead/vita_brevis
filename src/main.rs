use std::env;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let number: &u16 = &args[1].parse().unwrap();

    if requires_subtraction(&number) {
        resolve_subtraction(&number);
    } else {
        resolve_addition(&number);
    };
}

fn requires_subtraction(number: &u16) -> bool {
    let operator: [u16; 5] = [1, 5, 10, 50, 100];
    let round: [u16; 6] = [5, 10, 50, 100, 500, 1000];

    for i in operator.iter() {
        for j in round.iter() {
            if number + *i == *j {
                return false;
            };
        };
    };

    return true;
}

fn resolve_subtraction(number: &u16) {
    println!("Usual number: {}", &number);
}

fn resolve_addition(number: &u16) {
    println!("Exceptional number: {}", &number);
}
