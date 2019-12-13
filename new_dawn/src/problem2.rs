// problem2.rs
// Problem 2
// Sieve of Eratosthenes

/// Find all prime numbers less than `n`
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
	// Create a vector to store primes

	// Create a fixed array with size n storing 1 at first
	// indexes from 2 to n-1 represent all numbers between 2 and n (we disregard indexes 0 and 1)
	// ones and zeros inside array indicate if a number is prime or not
	
	// Iterate over a range of 2 through n
		// we multiply a prime up to n
		// we mark multiples of prime by placing 1 in array at that index 
		// if we see a multiple we skip to next number in range

	// Return the result vector

}