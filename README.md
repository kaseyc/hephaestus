Hephaestus
==========

Automata for Rust


###Rust Version
rustc 0.11.0-pre-nightly (25951b2 2014-05-30 00:31:44 -0700)

Progress
======
* Deterministic Finite Automata: Implemented  
* Non-deterministic Finite Automata: In progress  
* DFA Transformations (Complement, union, intersection): Done
* NFA to DFA compilation and NFA transformations: Up next
* Build NFA from regex: Maybe
* PDA: Yet to come

Why?
----

In Spring 2014 I took CS 181, Formal Languages and Automata at UCLA.  
I found the various automata we studied interesting, so I wanted to try
my hand at implementing them, both for fun and as a challenge to get better at Rust.
I also wanted to see what it was like trying to covert the algorithms and theorems
we studied into actual code.

The name Hephaestus
-------------------

Hephaestus was the Greek god of metals (among other things) and the builder of the automatons.
His association with metals fits nicely with Rust (Rust was named after a fungus, but whatever), and 
as the creator of the automatons, matches the aim of the library.

Thanks to [Harrison Liddiard](https://github.com/liddiard) for coming up with the name.

Examples:

```rust
extern crate hephaestus;
use hephaestus::{DFA, Run, NFA};

// DFA accepting all even length strings
let n1 = 2;
let a = vec!('a', 'b');
let accept1 = vec!(0);
let start = 0;
let t1 = vec!((0, 'a', 1), (0, 'b', 1),
					   (1, 'a', 0), (1, 'b', 0));

let dfa = DFA::new(n1, &a, &t1, start, &accept1).unwrap();
dfa.run("");       //true
dfa.run("a");      //false
dfa.run("ababba"); //true

// NFA accepting strings of the form a(b*)a(b*)a
let n2 = 4;
let accept2 = vec!(3);
let t2 = vec!((0, 'a', 1),
			  (1, 'a', 2), (1, 'b', 1),
			  (2, 'a', 3), (2, 'b', 2));

let nfa = NFA::new(n2, &a, &t2, start, &accept2).unwrap();
nfa.run("aaa");     //true
nfa.run("abbbaba"); //true
nfa.run("baba");    //false
```
