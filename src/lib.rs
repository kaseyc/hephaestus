#![crate_id = "hephaestus"]
#![deny(missing_doc)]

//! Implementations of various types of automata in Rust. <br>
//! I was inspired to write this after taking CS 181 at UCLA.
extern crate collections;

pub use DFA = dfa::DFA;
pub use DFATransition = dfa::DFATransition;

/// Basic trait abstracting over all automata.  
/// Checks if an automaton accepts a given string.
pub trait Run {
    /// Returns a boolean representing if the DFA accepts the string, or None
    /// if the string contains invalid characters.
    fn run(&self, string: &str) -> Option<bool>;
}

mod dfa;