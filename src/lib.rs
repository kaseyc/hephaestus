#![crate_name = "hephaestus"]
#![deny(missing_docs)]

//! Implementations of various types of automata in Rust. <br>
//! I was inspired to write this after taking CS 181 at UCLA.
extern crate collections;

pub use dfa::DFA as DFA;
pub use nfa::NFA as NFA;

/// A 3-tuple representing a state transition.
///
/// It has the form: **(current state, symbol, next state)**
pub type Transition = (uint, char, uint);

/// Basic trait abstracting over all automata.  
/// Checks if an automaton accepts a given string.
pub trait Run {
    /// Returns a boolean representing if the automaton accepts the string, or None
    /// if the string contains characters not in the automaton's alphabet.
    fn run(&self, string: &str) -> Option<bool>;
}

mod dfa;
mod nfa;
