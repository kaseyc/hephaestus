use collections::HashMap;
use std::fmt;

/// Deterministic Finite Automata
pub struct DFA {
	accept: ~[int],
	start: int,
	alphabet: ~[char],
	delta: HashMap<(int, char), int>
}

pub type Transition = (int, char, int);

impl DFA {
	/// Creates a DFA
	pub fn new(
		num_states: uint,
		alphabet: &[char],
		transitions: &[Transition],
		start: int,
		accept: &[int]
	) -> Result<DFA, ~str> {

		let accept_states = accept.to_owned();
		let alphabet = alphabet.to_owned();
		let mut trns_fn = HashMap::with_capacity(num_states * alphabet.len());
		
		for &(curr, sym, next) in transitions.iter() {
			trns_fn.insert((curr, sym), next);
		}

		Ok(DFA{
			accept: accept_states, 
			start: start,
			alphabet: alphabet,
			delta: trns_fn
		})
	}
}

impl fmt::Show for DFA {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f.buf, "Alphabet: {}", self.alphabet));
        try!(writeln!(f.buf, "Start State: {}", self.start));
        try!(writeln!(f.buf, "Accept States: {}", self.accept));
        try!(writeln!(f.buf, "Transitions: {}", self.delta));
        Ok(())
    }
}