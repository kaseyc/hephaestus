extern crate hephaestus;

use hephaestus::{DFA, Run, NFA};

///////////////////////////
////  DFA Unit Tests  /////
///////////////////////////

#[test]
fn dfa_validates_transitions() {
    let alphabet = vec!('0', '1');
    let states = 1;
    let start = 0;
    let accept = vec!(0);
    let t1 = vec!((0, '0', 0)); //Too few
    let t2 = vec!((0, '0', 0), (0, '0', 0)); //Duplicate
    let t3 = vec!((0, '1', 0), (0, '1', 5)); //Invalid state

    match DFA::new(states, &alphabet, &t1, start, &accept) {
        Ok(_) => fail!(),
        Err(e) => assert_eq!(e, format!("Incorrect number of transitions"))
    }

    match DFA::new(states, &alphabet, &t2, start, &accept) {
        Ok(_) => fail!(),
        Err(e) => assert_eq!(e, format!("Duplicate transition: (0, '0') -> 0"))
    }

    match DFA::new(states, &alphabet, &t3, start, &accept) {
        Ok(_) => fail!(),
        Err(e) => assert_eq!(e, format!("In transition: (0, '1') -> 5: State `5` does not exist"))
    }
}

#[test]
fn dfa_accepts_correct_inputs() {
    let alphabet = vec!('0', '1');
    let states = 2;
    let start = 0;
    let accept = vec!(0);
    //Accepts strings of only zeros
    let transitions = vec!((0, '0', 0),
                           (0, '1', 1),
                           (1, '0', 1),
                           (1, '1', 1));

    let dfa = DFA::new(states, &alphabet, &transitions, start, &accept).unwrap();

    let valid = vec!("", "0", "00", "0000000");

    for s in valid.iter() {
        match dfa.run(*s) {
            Some(b) => assert_eq!(b, true),
            None => fail!()
        }
    }
}

#[test]
fn dfa_rejects_incorrect_inputs() {
    let invalid = vec!("1", "001", "0100000");

    let alphabet = vec!('0', '1');
    let states = 2;
    let start = 0;
    let accept = vec!(0);
    //Accepts strings of only zeros
    let transitions = vec!((0, '0', 0),
                           (0, '1', 1),
                           (1, '0', 1),
                           (1, '1', 1));

    let dfa = DFA::new(states, &alphabet, &transitions, start, &accept).unwrap();

    for s in invalid.iter() {
        match dfa.run(*s) {
            Some(b) => assert_eq!(b, false),
            None => fail!()
        }
    }
}

#[test]
fn dfa_catches_invalid_inputs() {
    let invalid = vec!("a", "00a", "0b00000");

    let alphabet = vec!('0', '1');
    let states = 2;
    let start = 0;
    let accept = vec!(0);
    //Accepts strings of only zeros
    let transitions = vec!((0, '0', 0),
                           (0, '1', 1),
                           (1, '0', 1),
                           (1, '1', 1));

    let dfa = DFA::new(states, &alphabet, &transitions, start, &accept).unwrap();

    for s in invalid.iter() {
        assert_eq!(dfa.run(*s).is_none(), true);
    }
}

#[test]
fn dfa_complement_works() {
	let accept_strings = vec!("", "00", "10", "1");
	let reject_strings = vec!("000", "101010", "111110");

	let alphabet = vec!('0', '1');
    let states = 4;
    let start = 0;
    let accept = vec!(0, 1, 2);
    //Set of strings of length <= 2
    let transitions = vec!((0, '0', 1), (0, '1', 1),
    					   (1, '0', 2), (1, '1', 2),
    					   (2, '0', 3), (2, '1', 3),
    					   (3, '0', 3), (3, '1', 3));

	let dfa = DFA::new(states, &alphabet, &transitions, start, &accept).unwrap();
	let cmp = dfa.complement();

	for s in accept_strings.iter() {
		assert_eq!(dfa.run(*s).unwrap(), true);
		assert_eq!(cmp.run(*s).unwrap(), false);
	}

	for s in reject_strings.iter() {
		assert_eq!(dfa.run(*s).unwrap(), false);
		assert_eq!(cmp.run(*s).unwrap(), true);
	}
}

#[test]
fn dfa_union_works() {
	let accept_strings = vec!("", "00", "0000", "111111", "1");
	let reject_strings = vec!("010", "10", "111110");

	let alphabet = vec!('0', '1');
    let states = 2;
    let start = 0;
    let accept = vec!(0);
    //Only zeroes
    let t1 = vec!((0, '0', 0), (0, '1', 1),
    			  (1, '0', 1), (1, '1', 1));

    let t2 = vec!((0, '0', 1), (0, '1', 0),
    			  (1, '0', 1), (1, '1', 1));

	let dfa1 = DFA::new(states, &alphabet, &t1, start, &accept).unwrap();
	let dfa2 = DFA::new(states, &alphabet, &t2, start, &accept).unwrap();
	let union = dfa1.union(&dfa2).unwrap();

	for s in accept_strings.iter() {
		assert_eq!(union.run(*s).unwrap(), true);
	}

	for s in reject_strings.iter() {
		assert_eq!(union.run(*s).unwrap(), false);
	}
}

#[test]
fn dfa_intersection_works() {
	let accept_strings = vec!("");
	let reject_strings = vec!("010", "00", "111", "0", "1");

	let alphabet = vec!('0', '1');
    let states = 2;
    let start = 0;
    let accept = vec!(0);
    //Only zeroes
    let t1 = vec!((0, '0', 0), (0, '1', 1),
    			  (1, '0', 1), (1, '1', 1));

    let t2 = vec!((0, '0', 1), (0, '1', 0),
    			  (1, '0', 1), (1, '1', 1));

	let dfa1 = DFA::new(states, &alphabet, &t1, start, &accept).unwrap();
	let dfa2 = DFA::new(states, &alphabet, &t2, start, &accept).unwrap();
	let ints = dfa1.intersect(&dfa2).unwrap();

	for s in accept_strings.iter() {
		assert_eq!(ints.run(*s).unwrap(), true);
	}

	for s in reject_strings.iter() {
		assert_eq!(ints.run(*s).unwrap(), false);
	}
}

#[test]
fn dfa_minimization() {
	let n = 4;
	let start = 0;
	let a = vec!('a', 'b');
    let accept = vec!(0, 1, 3);
    let t = vec!((0, 'a', 1), (0, 'b', 1),
                 (1, 'a', 0), (1, 'b', 0),
                 (2, 'a', 2), (2, 'b', 0),
                 (3, 'a', 2), (3, 'b', 1));

    let dfa = DFA::new(n, &a, &t, start, &accept).unwrap().minimize().unwrap();

    let expected = "Alphabet: [a, b]\nStart State: 0\nAccept States: {0}\nTransitions:\n  (0, 'a') -> 0\n  (0, 'b') -> 0\n";

    assert_eq!(format!("{}", dfa).as_slice(), expected);
}

#[test]
fn dfa_minimum_complement_intersection() {
	let n = 2;
    let a = vec!('a', 'b');
    let accept = vec!(0, 1);
    let start = 0;
    let t = vec!((0, 'a', 1), (0, 'b', 1),
                 (1, 'a', 0), (1, 'b', 0));

    let d1 = DFA::new(n, &a, &t, start, &accept).unwrap();
    let d2 = d1.complement().intersect(&d1).unwrap().minimize().unwrap();

    let expected = "Alphabet: [a, b]\nStart State: 0\nAccept States: {}\nTransitions:\n  (0, 'a') -> 0\n  (0, 'b') -> 0\n";
    assert_eq!(format!("{}", d2).as_slice(), expected);
}


///////////////////////////
////  NFA Unit Tests  /////
///////////////////////////

#[test]
fn nfa_validates_transitions() {
    let alphabet = vec!('0', '1');
    let states = 1;
    let start = 0;
    let accept = vec!(0);
    let t = vec!((0, '1', 0), (0, '1', 5)); //Invalid state

    match DFA::new(states, &alphabet, &t, start, &accept) {
        Ok(_) => fail!(),
        Err(e) => assert_eq!(e, format!("In transition: (0, '1') -> 5: State `5` does not exist"))
    }
}

#[test]
fn nfa_accepts_proper_strings() {
	let states = 3;
	let alphabet = vec!('0', '1');
	let accept = vec!(2);
	let start = 0;
	let t = vec!((0, '0', 1), (1, '0', 2)); //Accepts the string '00'

	let nfa = NFA::new(states, &alphabet, &t, start, &accept).unwrap();

	let incorrect_strings = vec!("", "01", "10", "11", "0", "1", "001");

	match nfa.run("00") {
		Some(b) => assert_eq!(b, true),
		None => fail!()
	}

	for s in incorrect_strings.iter() {
		match nfa.run(*s) {
			Some(b) => assert_eq!(b, false),
			None => fail!()
		}
	}
}

#[test]
fn nfa_catches_invalid_inputs() {
    let alphabet = vec!('0', '1');
    let states = 1;
    let start = 0;
    let accept = vec!(0);
 
    let t = vec!((0, '1', 0));

    let nfa = NFA::new(states, &alphabet, &t, start, &accept).unwrap();

    assert_eq!(nfa.run("a").is_none(), true);
}

#[test]
fn nfa_epsilon_transitions() {
	let alphabet = vec!('0');
    let states = 2;
    let start = 0;
    let accept = vec!(1);

    let t = vec!((0, '_', 1), (1, '_', 0));

    let nfa = NFA::new(states, &alphabet, &t, start, &accept).unwrap();

    match nfa.run("") {
    	Some(b) => assert_eq!(b, true),
    	None => fail!()
    }

    match nfa.run("0") {
    	Some(b) => assert_eq!(b, false),
    	None => fail!()
    }
}