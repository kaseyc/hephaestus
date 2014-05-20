extern crate hephaestus;

use hephaestus::{Run,DFA};

fn main() {
    let q = 2;
    let sigma = ['0', '1'];
    let delta = [(0, '0', 1),
                 (0, '1', 1),
                 (1, '0', 0),
                 (1, '1', 0)];
    let q0 = 0;
    let f = [0];

    let x = match DFA::new(q, sigma, delta, q0, f) {
        Ok(p) => p,
        Err(e) => {
            println!("Error: {}", e);
            fail!()
        }
    };
    println!("{}", x);

    //Run some strings
    let strings = ["0", "0100", "001", "1", "0", "00", "1111111111"];

    for string in strings.iter() {
        match x.run(*string) {
            Some(s) => println!("String: \"{}\", Result: {}\n", *string, s),
            None => println!("String: \"{}\", Result: Invalid\n", *string)
        }
    }
}
