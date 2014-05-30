extern crate hephaestus;

use hephaestus::{DFA, Run, NFA};

fn main() {
    // DFA accepting all even length strings
    let n1 = 2;
    let a = vec!('a', 'b');
    let accept1 = vec!(0);
    let start = 0;
    let t1 = vec!((0, 'a', 1), (0, 'b', 1),
                  (1, 'a', 0), (1, 'b', 0));

    let dfa = DFA::new(n1, &a, &t1, start, &accept1).unwrap();
    println!("{}", dfa.run("").unwrap());       //true
    println!("{}", dfa.run("a").unwrap());      //false
    println!("{}", dfa.run("ababba").unwrap()); //true

    // NFA accepting strings of the form a(b*)a(b*)a
    let n2 = 4;
    let accept2 = vec!(3);
    let t2 = vec!((0, 'a', 1),
                  (1, 'a', 2), (1, 'b', 1),
                  (2, 'a', 3), (2, 'b', 2));

    let nfa = NFA::new(n2, &a, &t2, start, &accept2).unwrap();
    println!("{}", nfa.run("aaa").unwrap());     //true
    println!("{}", nfa.run("abbbaba").unwrap()); //true
    println!("{}", nfa.run("baba").unwrap());    //false
}
