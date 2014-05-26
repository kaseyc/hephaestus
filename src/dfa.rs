use collections::{HashSet, HashMap};
use std::fmt;
use super::{Run, Transition};


/// Deterministic Finite Automata
///
/// A DFA is comprised of a set of states and an alphabet
/// of symbols. Each state has a transition from itself to
/// some other state for each symbol in the alphabet. 
///
/// A DFA executes an input string by starting from the start state
/// and reading the string one symbol at a time. For each symbol, it
/// chages states based on the specified transitions.
/// A DFA "accepts" a string if it ends in any accept state after reading
/// the entire input.
pub struct DFA {
    start: uint,
    alphabet: Vec<char>,
    delta: HashMap<(uint, char), uint>,
    accept: Vec<uint>,
    num_states: uint
}

impl DFA {
    /// Creates a new DFA
    ///
    /// Returns an Err if there is a transition on a state or symbol that
    /// does not exist, or if there is not **exactly** one transition for each
    /// combination of state and input symbol.
    pub fn new(
        num_states: uint,
        alphabet: &Vec<char>,
        transitions: &Vec<Transition>,
        start: uint,
        accept: &Vec<uint>
    ) -> Result<DFA, ~str> {

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
                return Err(format!("In transition: ({}, '{}') -> {}: State `{}` \
                                    does not exist", curr, sym, next, curr));
            }

            if next >= num_states {
                return Err(format!("In transition: ({}, '{}') -> {}: State `{}` \
                                    does not exist", curr, sym, next, next));
            }

            if permutation.contains(&(curr, sym)) {
                trns_fn.insert((curr, sym), next);
                permutation.remove(&(curr,sym));
            }

            else {
                return Err(format!("Duplicate transition: ({}, '{}') -> {}", curr, sym, next));
            }
        }

        Ok(DFA{
            accept: accept.clone(), 
            start: start,
            alphabet: alphabet.clone(),
            delta: trns_fn,
            num_states: num_states
        })
    }

    /// Return a new DFA recognizing the union of the two inputs.  
    /// The union accepts any string that either input DFA would accept. 
    ///
    /// Returns None if the DFAs do not use the same alphabet.
    pub fn union (&self, d2: &DFA) -> Option<DFA> {
        DFA::dfa_product(self, d2, |x, y| { x || y })
    }

    /// Return a DFA representing the intersection of the inputs.  
    /// Accepts all strings accepted by both input DFAs.
    ///
    /// Returns None if the DFAs do not use the same alphabet.
    pub fn intersect(&self, d2: &DFA) -> Option<DFA> {
        DFA::dfa_product(self, d2, |x, y| { x && y })
    }

    //Take the cartesian product of 2 DFAs.
    //This is the basis for both union and intersection.
    fn dfa_product(d1: &DFA, d2: &DFA, f: |bool, bool| -> bool) -> Option<DFA> {
        //Check that the DFAs have matching alphabets
        //To do this, we need to clone and sort :(
        let mut a1 = d1.alphabet.clone();
        let mut a2 = d2.alphabet.clone();
        a1.sort();
        a2.sort();
        if a1 != a2 {
            return None
        }

        let num_states = d1.num_states * d2.num_states;
        let mut state_map = HashMap::with_capacity(num_states);
        let mut count: uint = 0;
        let mut accept: Vec<uint> = Vec::new();

        //Take the cartesian product of the states in both DFAs and map them to integers
        //Additionally, build the list of accept states
        for i in range (0, d1.num_states) {
            for j in range (0, d2.num_states) {
                state_map.insert((i, j), count);
                if f(d1.accept.contains(&i), d2.accept.contains(&j)) {
                    accept.push(count);
                }

                count += 1;
            }
        }

        let start: uint = state_map.get_copy(&(d1.start, d2.start));

        //Build the transition function
        //Assumes both d1 and d2 are valid DFAs
        let trns_size = num_states * d1.alphabet.len();
        let mut trns_fn = HashMap::with_capacity(trns_size);

        for i in range(0, d1.num_states) {
            for j in range(0, d2.num_states) {
                for sym in d1.alphabet.iter() {
                    let s1 = d1.delta.get_copy(&(i, *sym));
                    let s2 = d2.delta.get_copy(&(j, *sym));
                    let curr_s = state_map.get_copy(&(i,j));
                    let new_s = state_map.get_copy(&(s1, s2));
                    trns_fn.insert((curr_s, sym.clone()), new_s);
                }
            }
        }

        Some(DFA {accept: accept,
                  start: start,
                  delta: trns_fn,
                  alphabet: d1.alphabet.clone(),
                  num_states: num_states})
    }

    /// Returns a DFA accepting the complement of self. 
    ///
    /// It accepts all strings over self's alphabet that self rejects and vice versa.
    pub fn complement(&self) -> DFA {
        let all_states: Vec<uint> = range(0, self.num_states).collect();
        let accept: Vec<uint> = all_states.move_iter().filter(|x| !self.accept.contains(x)).collect();

        DFA { accept: accept,
              start: self.start,
              alphabet: self.alphabet.clone(),
              delta: self.delta.clone(),
              num_states: self.num_states
        }
    }

}

impl Run for DFA {
    fn run(&self, string: &str) -> Option<bool> {
        let mut curr_state = self.start;

        // Compute the transition for each char in string
        for sym in string.chars() { 
             match self.delta.find_copy(&(curr_state, sym)) {
                Some(v) => curr_state = v,
                None => return None
             }
        }

        Some(self.accept.contains(&curr_state)) 
    }
}

impl fmt::Show for DFA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Alphabet: {}\n", self.alphabet));
        try!(write!(f, "Start State: {}\n", self.start));
        try!(write!(f, "Accept States: {}\n", self.accept));
        try!(write!(f, "Transitions: \n"));

        for (&(curr, sym), next) in self.delta.iter() {
          try!(write!(f, "  ({}, '{}') -> {}\n", curr, sym, next));
        }
        Ok(())
    }
}




//Unit tests

#[cfg(test)]
mod tests {
    use super::DFA;

    #[test]
    fn dfa_product_catches_different_alphabets() {
        let a1 = vec!('0', '1');
        let a2 = vec!('0', 'a');
        let states = 1;
        let start = 0;
        let accept = vec!(0);
        //Accepts strings of only zeros
        let t1 = vec!((0, '0', 0), (0, '1', 0));
        let t2 = vec!((0, '0', 0), (0, 'a', 0));

        let dfa1 = DFA::new(states, &a1, &t1, start, &accept).unwrap();
        let dfa2 = DFA::new(states, &a2, &t2, start, &accept).unwrap();
        let res = DFA::dfa_product(&dfa1, &dfa2, |_, _| { false });

        assert_eq!(res.is_none(), true);
    }
} 
