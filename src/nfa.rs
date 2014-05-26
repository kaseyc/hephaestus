use collections::bitv::BitvSet;
use collections::HashMap;
use std::fmt;
use super::{Run, Transition};

/// Nondeterministic Finite Automaton.
///
/// Similar in principle to a DFA except that an NFA can have 0 or
/// multiple transitions on an input symbol, and can transition
/// on the empty string (signified by '_').
pub struct NFA {
    start: uint,
    alphabet: Vec<char>,
    delta: HashMap<(uint, char), BitvSet>,
    accept: Vec<uint>,
    num_states: uint
}

impl NFA {
    /// Builds an NFA.
    ///
    /// Returns an Error if '_' is included in the alphabet or
    /// if a transition contains a state or symbol that does not exist.
    pub fn new(
        num_states: uint,
        alphabet: &Vec<char>,
        transitions: &Vec<Transition>,
        start: uint,
        accept: &Vec<uint>
    ) -> Result<NFA, ~str> {


        let mut trns_fn: HashMap<(uint, char), BitvSet> = HashMap::with_capacity(transitions.len());

        if alphabet.contains(&'_') {
            return Err(format!("Alphabets cannot contain '_'"));
        }

        // Validate transitions and add them to the transition table
        for &(curr, sym, next) in transitions.iter() {
            if sym != '_' && !alphabet.contains(&sym) {
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

            trns_fn.find_with_or_insert_with((curr, sym), next,
                //If the BitvSet exists, add next to it
                |_, old, new| { old.insert(new); }, 

                //If no match founf, create a new BitvSet and add it
                |_, v| {
                    let mut bv = BitvSet::new();
                    bv.insert(v);
                    bv }
            );
        }

        Ok(NFA{
            accept: accept.clone(), 
            start: start,
            alphabet: alphabet.clone(),
            delta: trns_fn,
            num_states: num_states
        })
    }
}

//In place expansion of the current states to include epsilon transitions.
//It loops to handle the epsilon transitions from newly added states.
//It terminates when no new states are added, so it will not get caught in epsilon cycles.
fn epsilons(curr: &mut BitvSet, delta: &HashMap<(uint, char), BitvSet>) {
    let mut next = BitvSet::new();
    loop {
        for i in curr.iter() {
                match delta.find(&(i, '_')) {
                    None => {},
                    Some(bv) => next.union_with(bv)
                }
            }

        //Terminate if no new states are added
        if curr.is_superset(&next) {
            break;
        }

        curr.union_with(&next);
        next.clear();
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
        epsilons(&mut curr_states, &self.delta);

        for sym in input.chars() {
            if sym != '_' && !self.alphabet.contains(&sym) {
                return None;
            }

            //Get transitions from the current input symbol
            for i in curr_states.iter() {
                match self.delta.find(&(i, sym)) {
                    None => {},
                    Some(bv) => next_states.union_with(bv)
                }
            }

            //If next states is empty, reject the string
            if next_states.is_empty() {
                return Some(false);
            }

            curr_states.clear();
            curr_states.union_with(&next_states);
            next_states.clear();

            epsilons(&mut curr_states, &self.delta);
        }

        Some(self.accept.iter().any(|x| curr_states.contains(x)))
    }
}

impl fmt::Show for NFA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Alphabet: {}\n", self.alphabet));
        try!(write!(f, "Start State: {}\n", self.start));
        try!(write!(f, "Accept States: {}\n", self.accept));
        try!(write!(f, "Transitions: \n"));

        for (&(curr, sym), next) in self.delta.iter() {
          try!(write!(f, "  ({}, '{}') -> {}\n", curr, sym, next.iter().collect::<Vec<uint>>()));
        }
        Ok(())
    }
}
