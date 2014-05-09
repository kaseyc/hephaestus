extern crate hephaestus;

use hephaestus::DFA;

fn main() {
	let x = match DFA::new(3, ['0', '1'], [(0, '0', 1)], 0, [1,2]) {
		Ok(p) => p,
		Err(_) => fail!()
	};
	println!("{}", x);
}
