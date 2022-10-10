use std::env;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    // let number: &u16 = &args[1].parse().unwrap();

    let operator: [u16; 5] = [1, 5, 10, 50, 100];
    let round: [u16; 6] = [5, 10, 50, 100, 500, 1000];

    for number in 1..=1000 {
        for i in operator.iter() {
            for j in round.iter() {
                if number + *i == *j {
                    resolve_number_with_exception(&number);
                };
            };
        };
        resolve_number_as_usual(&number);
    };
}

fn resolve_number_as_usual(number: &u16) {
    println!("Usual number: {}", &number);
}

fn resolve_number_with_exception(number: &u16) {
    println!("Exceptional number: {}", &number);
}
