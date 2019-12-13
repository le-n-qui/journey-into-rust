// problem1.rs
// Problem 1
// Vector and Slice Manipulation

/// Computes the sum of all elements in the input i32 slice named 'slice'
pub fn sum(slice: &[i32]) -> i32 {
	let mut sum = 0;
	for elem in slice {
		sum += elem;
	}
	sum
}

/// Deduplicates items in the input vector 'vs'. Produces a vector containing
/// the first instance of each distinct element of 'vs', preserving the 
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
	let mut unique_vs: Vec<i32> = Vec::new();
	
	// Iterate each element in vector vs
	for elem in vs {
		if unique_vs.is_empty() {
			unique_vs.push(*elem);
		} else {
			let mut counter = 0;
			let mut condition = false; 
			while counter < unique_vs.len() {
				if elem == &unique_vs[counter] {
					condition = true;
					break;
				}
				counter += 1;
			}
			if !condition { 
				unique_vs.push(*elem); 
			}
		}
	}

	unique_vs
}

/// Filters a vector 'vs' using a predicate 'pred' (a function from 'i32' to
/// 'bool'). Returns a new vector containing only elements that satisfy 'pred'.
pub fn filter(vs: &Vec<i32>, pred: fn(i32) -> bool) -> Vec<i32> {
	let mut new_vs: Vec<i32> = Vec::new();
	for elem in vs {
		if pred(*elem) {
			new_vs.push(*elem);
		}
	}
	new_vs
}