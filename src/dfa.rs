use std::collections::hashmap::{HashSet, HashMap};
use std::collections::bitv::BitvSet;
use std::fmt;
use std::cmp::PartialEq;
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
    accept: BitvSet,
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
    ) -> Result<DFA, String> {

        let dfa_size = num_states * alphabet.len();

        // Check that DFA has the proper number of transitions
        if transitions.len() != dfa_size {
            return Err(format!("Incorrect number of transitions"));
        }

        if start >= num_states {
            return Err(format!("Invalid start state"));
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

        let mut accept_states = BitvSet::new();
        for i in accept.iter() {
            accept_states.insert(*i);
        }

        Ok(DFA{
            accept: accept_states, 
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
        let mut accept = BitvSet::new();

        //Take the cartesian product of the states in both DFAs and map them to integers
        //Additionally, build the list of accept states
        for i in range (0, d1.num_states) {
            for j in range (0, d2.num_states) {
                state_map.insert((i, j), count);
                if f(d1.accept.contains(&i), d2.accept.contains(&j)) {
                    accept.insert(count);
                }

                count += 1;
            }
        }

        let start: uint = state_map.get_copy(&(d1.start, d2.start));

        //Build the transitions
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
        
        let mut bv = BitvSet::new();
        for i in accept.iter() {
            bv.insert(*i);
        }

        DFA { accept: bv,
              start: self.start,
              alphabet: self.alphabet.clone(),
              delta: self.delta.clone(),
              num_states: self.num_states
        }
    }

    fn reachable_states(&self) -> BitvSet {
        let mut reachable = BitvSet::new();
        reachable.insert(self.start);
        let mut new_states = BitvSet::new();
        new_states.insert(self.start);

        loop {
            let mut temp = BitvSet::new();
            for elem in new_states.iter() {
                for sym in self.alphabet.iter() {
                    temp.insert(self.delta.get_copy(&(elem, *sym)));
                }
            }

            temp.difference_with(&reachable);
            reachable.union_with(&temp);
            new_states = temp;

            if new_states.is_empty() {
                break;
            }
        }

        return reachable;
    }

    /// Returns the minimal DFA (smallest number of states) that accepts the same language.
    /// 
    /// Implements [Hopcroft's algorithm](http://en.wikipedia.org/wiki/DFA_minimization#Hopcroft.27s_algorithm).
    pub fn minimize(&self) -> Result<DFA, String> {
        //Remove unreachable states
        let reachable = self.reachable_states();

        //Minimize with Hopcroft's
        let mut partitions = vec!();
        let mut w = vec!();

        let mut non_accept = reachable.clone();
        non_accept.difference_with(&self.accept);

        let mut reachable_accept = self.accept.clone();
        reachable_accept.intersect_with(&reachable);

        partitions.push(reachable_accept.clone());
        partitions.push(non_accept);
        w.push(reachable_accept);

        //Loop until w is empty
        loop {
            let set = match w.pop() {
                Some(s) => s,
                None => break
            };

            for sym in self.alphabet.iter() {
                let mut x = BitvSet::new();
                for s in reachable.iter() {
                    match self.delta.find(&(s, *sym)) {
                        Some(v) if set.contains(v) => { x.insert(*v); },
                        _ => {}
                    }
                }

                let mut new_p = vec!();

                for y in partitions.move_iter() {
                    let mut intersection = y.clone();
                    intersection.intersect_with(&x);
                    if intersection.is_empty() {
                        new_p.push(y);
                        continue;
                    }

                    let mut difference = y.clone();
                    difference.difference_with(&x);
                    if difference.is_empty() {
                        new_p.push(y);
                        continue;
                    }

                    new_p.push(intersection.clone());
                    new_p.push(difference.clone());

                    if w.contains(&y) {
                        w.push(intersection);
                        w.push(difference);
                    }

                    else {
                        if intersection.len() <= difference.len() {
                            w.push(intersection);
                        }

                        else {
                            w.push(difference);
                        }
                    }
                }

                partitions = new_p;
            }
        }

        //Remove empty paritions
        partitions = partitions.move_iter().filter(|ref x| !x.is_empty()).collect();

        //partitions now holds all the equivalence classes
        //Construct a DFA with 1 state for each set in partitions
        //Use the index of the set as its state number
        let mut transitions = vec!();
        let mut start = 0; //This will be overwritten
        let mut accept = vec!();
        for (idx, p) in partitions.iter().enumerate() {
            //get first element of p
            let elem = match p.iter().next() {
                Some(e) => e,
                None => continue
            };
            for sym in self.alphabet.iter() {
                let new_state = self.delta.get(&(elem, *sym));
                for (new_idx, s) in partitions.iter().enumerate() {
                    if s.contains(new_state) {
                        transitions.push((idx, *sym, new_idx));
                        break;
                    }
                }
            }

            if p.contains(&self.start) {
                start = idx;
            }

            for i in self.accept.iter() {
                if p.contains(&i) {
                    accept.push(idx);
                }
            }
        }

        DFA::new(partitions.len(), &self.alphabet, &transitions, start, &accept)
    }

    /// Return true if there are no reachable accept states
    fn accepts_none(&self) -> bool {
        if self.accept.is_empty() {
            return true;
        }

        let mut reachable = self.reachable_states();
        reachable.intersect_with(&self.accept);
        return reachable.is_empty();
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

impl PartialEq for DFA {
    /// Check that (self intersect ~other) union (~self intersect other)
    /// accepts the empty language 
    fn eq(&self, other: &DFA) -> bool {
        let eq1 = match self.intersect(&other.complement()) {
            None => { return false },
            Some(dfa) => dfa
        };

        let eq2 = match other.intersect(&self.complement()) {
            None => { return false },
            Some(dfa) => dfa
        };

        match eq1.union(&eq2) {
            None => false,
            Some(dfa) => dfa.accepts_none()
        }
    }
}

impl fmt::Show for DFA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Alphabet: {}\n", self.alphabet));
        try!(write!(f, "Start State: {}\n", self.start));
        try!(write!(f, "Accept States: {}\n", self.accept));
        try!(write!(f, "Transitions:\n"));

        let mut temp = vec!();
        for (&(curr, sym), next) in self.delta.iter() {
            temp.push((curr, sym, next));
        }

        temp.sort();
        for &(curr, sym, next) in temp.iter() {
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
