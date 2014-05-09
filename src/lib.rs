#![crate_id = "hephaestus"]
#![deny(missing_doc)]

//! Implementations of various types of automata in Rust. <br>
//! I was inspired to write this after taking CS 181 at UCLA.
extern crate collections;

pub use DFA = dfa::DFA;
pub use Transition = dfa::Transition;

mod dfa;