extern crate hephaestus;

use hephaestus::{Run,NFA};

fn main() {
    let q = 2;
    let sigma = vec!('0', '1');

    let d1 = vec!((0, '0', 1),
                  (1, '1', 1));

    let d2 = vec!((0, '_', 1),
                  (1, '_', 0));

    let q0 = 0;
    
    let f: Vec<uint> = vec!(1);

    let nfa1 = match NFA::new(q, sigma.clone(), d1, q0, f.clone()) {
        Ok(x) => x,
        Err(_) => fail!()
    };

    let nfa2 = match NFA::new(q, sigma.clone(), d2, q0, f.clone()) {
        Ok(x) => x,
        Err(_) => fail!()
    };

    let strings = vec!("0", "01", "", "10", "11", "1");

    println!("NFA 1\n");
    for string in strings.iter() {
        match nfa1.run(*string) {
            Some(s) => println!("String: \"{}\", Result: {}\n", *string, s),
            None => println!("String: \"{}\", Result: Invalid\n", *string)
        }
    }

    println!("NFA 2\n");
    for string in strings.iter() {
        match nfa2.run(*string) {
            Some(s) => println!("String: \"{}\", Result: {}\n", *string, s),
            None => println!("String: \"{}\", Result: Invalid\n", *string)
        }
    }

    println!("{}", nfa2);


/*
    //Strings of all 0s
    let delta1 = vec!((0, '0', 0),
                      (0, '1', 1),
                      (1, '0', 1),
                      (1, '1', 1));

    //Strings of all 1s
    let delta2 = vec!((0, '0', 1),
                      (0, '1', 0),
                      (1, '0', 1),
                      (1, '1', 1));

    let q0 = 0;
    let f: Vec<uint> = vec!(0);

    let zeroes = match DFA::new(q, sigma.clone(), delta1, q0, f.clone()) {
        Ok(p) => p,
        Err(e) => {
            println!("Error: {}", e);
            fail!()
        }
    };

    let ones = match DFA::new(q, sigma, delta2, q0, f) {
        Ok(p) => p,
        Err(e) => {
            println!("Error: {}", e);
            fail!()
        }
    };

    let u = zeroes.union(&ones).unwrap(); //Strings of only 1s and 0s  
    let i = zeroes.intersect(&ones).unwrap(); //Empty string only
    let c = zeroes.complement(); //All with at least one 1

    println!("{}", u);

    //Run some strings
    let strings = vec!("a", "0100", "001", "1", "000000", "", "1111111111");

    println!("UNION\n");
    for string in strings.iter() {
        match u.run(*string) {
            Some(s) => println!("String: \"{}\", Result: {}\n", *string, s),
            None => println!("String: \"{}\", Result: Invalid\n", *string)
        }
    }

    println!("INTERSECTION\n");
    for string in strings.iter() {
        match i.run(*string) {
            Some(s) => println!("String: \"{}\", Result: {}\n", *string, s),
            None => println!("String: \"{}\", Result: Invalid\n", *string)
        }
    }

    println!("COMPLEMENT\n");
    for string in strings.iter() {
        match c.run(*string) {
            Some(s) => println!("String: \"{}\", Result: {}\n", *string, s),
            None => println!("String: \"{}\", Result: Invalid\n", *string)
        }
    }
    */
}
