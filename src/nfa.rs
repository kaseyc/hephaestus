use collections::bitv::BitvSet;
use collections::HashMap;
use std::fmt;
use super::{Run, Transition};

/// Nondeterministic Finite Automaton
/// Similar in principle to a DFA except that an NFA can have 0 or
/// multiple transitions on an input symbol, and can transition
/// on the empty symbol (signified by '_').
pub struct NFA {
    start: uint,
    alphabet: Vec<char>,
    delta: HashMap<(uint, char), BitvSet>,
    accept: Vec<uint>,
    num_states: uint
}

impl NFA {
    // Builds an NFA
    // Returns an Error if '_' is included in the alphabet or
    // if a transition contains a state or symbol that does not exist.
    pub fn new(
        num_states: uint,
        alphabet: Vec<char>,
        transitions: Vec<Transition>,
        start: uint,
        accept: Vec<uint>
    ) -> Result<DFA, ~str> {


        let mut trns_fn = HashMap::with_capacity(transitions.len());

        if alphabet.contains('_') {
            return Err(format!("Alphabets cannot contain '_'"));
        }

        // Validate transitions and add them to the transition table
        for &(curr, sym, next) in transitions.iter() {
            if syn != '_'&& !alphabet.contains(&sym) {
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

            trns_fn.insert_or_update_with((curr, sym), BitvSet::new().insert(next), |_, old| { old.insert(next) });
        }

        Ok(NFA{
            accept: accept, 
            start: start,
            alphabet: alphabet,
            delta: trns_fn,
            num_states: num_states
        })
    }
}

impl Run for NFA {
    // Check whether self accepts the given input string.
    // To do this, the string is run over the automaton starting from
    // the start state, similar to a DFA. However, instead of a single current state,
    // a vector of all the possible states it could be in is kept.

    fn run(&self, input: &str) -> Option<bool> {
        let mut curr_states = BitvSet::new();
        let mut next_states = BitvSet::new();

        curr_states.insert(self.start);

        for sym in input.chars() {
            if sym != '_' && !self.alphabet.contains(&sym) {
                return None;
            }

            for i in curr_states.iter() {
                //Get transitions from the current input symbol
                match trns_fn.get(&(i, sym)) {
                    None => {},
                    Some(bv) => next_states.union_with(bv)
                }

                //Get epsilon transitions
                match trns_fn.get(&(i, '_')) {
                    None => {},
                    Some(bv) => next_states.union_with(bv)
                }
            }

            //If next states is empty, reject the string
            if next_states.is_empty() {
                return Some(false);
            }

            curr_states.clear();
            curr_states.union_with(next_states);
            next_states.clear();
        }

        accept.iter().any(|x| curr_states.contains(x))
    }
}