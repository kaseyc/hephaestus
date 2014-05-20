use collections::{HashSet, HashMap};
use std::fmt;
use super::Run;

/// Deterministic Finite Automata
pub struct DFA {
    start: uint,
    alphabet: ~[char],
    delta: HashMap<(uint, char), uint>,
    accept: ~[uint]
}

/// A 3-tuple representing a state transition.<br>
/// It has the form: **(current state, symbol, next state)**
pub type DFATransition = (uint, char, uint);

impl DFA {
    /// Creates a DFA
    pub fn new(
        num_states: uint,
        alphabet: &[char],
        transitions: &[DFATransition],
        start: uint,
        accept: &[uint]
    ) -> Result<DFA, ~str> {

        let accept_states = accept.to_owned();
        let alphabet = alphabet.to_owned();
        let dfa_size = num_states * alphabet.len();

        // Check that DFA has the proper number of transitions
        if transitions.len() != dfa_size {
            return Err(format!("Incorrect number of transitions"));
        }

        let mut trns_fn = HashMap::with_capacity(dfa_size);

        // We need to check that each (state, sym) transiton occurs exactly once.
        // We create a second hash initialized with the values we still need to see, and remove
        // one each time we add it to the transition function.
        // If one is missing, there is a duplicate function, as we already validated that there are only
        // states*symbols transitions given.

        let mut permutation = HashSet::with_capacity(dfa_size);
        for i in range(0, num_states) {
            for sym in alphabet.iter() {
                permutation.insert((i, sym.clone()));
            }
        }

        // Validate transitions and add them to the transition table
        for &(curr, sym, next) in transitions.iter() {
            if !alphabet.contains(&sym) {
                return Err(format!("Symbol `{}` is not in the alphabet", sym));
            }

            if curr >= num_states {
                return Err(format!("In transition: ({}, '{}') -> {}: State `{}`\
                                    does not exist", curr, sym, next, curr));
            }

            if next >= num_states {
                return Err(format!("In transition: ({}, '{}') -> {}: State `{}`\
                                    does not exist", curr, sym, next, next));
            }

            if permutation.contains(&(curr, sym)) {
                trns_fn.insert((curr, sym), next);
                permutation.remove(&(curr,sym));
            }

            else {
                return Err(format!("Duplicate or missing transitions"));
            }
        }

        Ok(DFA{
            accept: accept_states, 
            start: start,
            alphabet: alphabet,
            delta: trns_fn
        })
    }
}

impl Run for DFA {
    fn run(&self, string: &str) -> Option<bool> {
        let mut curr_state = self.start;

        // Compute the transition for each char in string
        for sym in string.chars() { 
            if !self.alphabet.contains(&sym) {
                return None;
            }

            curr_state = self.delta.get_copy(&(curr_state, sym));
        }

        Some(self.accept.contains(&curr_state)) 
    }
}

impl fmt::Show for DFA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f.buf, "Alphabet: {}", self.alphabet));
        try!(writeln!(f.buf, "Start State: {}", self.start));
        try!(writeln!(f.buf, "Accept States: {}", self.accept));
        try!(writeln!(f.buf, "Transitions: "));

        for (&(curr, sym), next) in self.delta.iter() {
            try!(writeln!(f.buf, "  ({}, '{}') -> {}", curr, sym, next));
        }
        Ok(())
    }
}