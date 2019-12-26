// problem3.rs
// Problem 3
// Towers of Hanoi

/// Solves for the sequence of moves required to move all 
/// discs from peg 1 to peg 3, using peg 2 as an intermediary
pub fn hanoi(num_discs: u32) -> Vec<(u8, u8)> {
	// Make a result vector
	let mut result: Vec<(u8, u8)> = Vec::new();
	// Make a call to private function move_disc()
	// with the given number of discs
	result.append(&mut move_disc(num_discs, 1, 3, 2));
	// Return result
	result
}

fn move_disc(num_discs: u32, source: u8, target: u8, auxiliary: u8) -> Vec<(u8, u8)> {
	// Create a vector to store the sequence of move
	let mut move_vector: Vec<(u8, u8)> = Vec::new();
	// If num_discs is bigger than zero
	if num_discs > 0 {
		// move num_discs - 1 from source to auxiliary
		move_vector.append(&mut move_disc(num_discs - 1, source, auxiliary, target));
		// Record the sequence of move for a particular disk
		move_vector.push((source, target));
		// move num_discs - 1 from auxiliary to target
		move_vector.append(&mut move_disc(num_discs - 1, auxiliary, target, source));
	}
	move_vector
}