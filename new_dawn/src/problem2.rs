// problem2.rs
// Problem 2
// Sieve of Eratosthenes

/// Find all prime numbers less than `n`
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
	
	// Create a vector to store primes
	let mut result_vec: Vec<u32> = Vec::new();
	// Create a fixed array with size n storing 1 at first
	// indexes from 2 to n-1 represent all numbers between 2 and n (we disregard indexes 0 and 1)
	// ones and zeros inside array indicate if a number is prime or not
	let mut array = vec![1; n as usize];
	// Iterate over a range of 2 through n
	for number in 2..n {
		if array[number as usize] == 1 { 
			result_vec.push(number);
			let mut count = 2;
			let mut multiple = number * count; 
			while multiple < n {
				// we mark multiples of prime by placing 0 in array at that index 
				array[multiple as usize] = 0;
				// increment count
				count += 1;
				// we multiply a prime up to n
				multiple = number * count;	
			}
		}
		// otherwise, if we see a multiple we skip to next number in range
	}
	// Return the result vector
	result_vec
}