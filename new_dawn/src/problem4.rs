// problem4.rs
// Qui Le
// 12/30/2019
// Bloom Filter

pub fn djb2(bytes: &[u8]) -> u64 {
	let mut hash: u64 = 5381;
	for b in bytes {
		// hash * 33 + c
		hash = (hash.wrapping_shr(5) + hash) + (*b as u64);
	}
	hash
}

pub fn fnv(bytes: &[u8]) -> u64 {
	let mut hash = 0xcbf29ce484222325;
	for b in bytes {
		hash = hash ^ (*b as u64);
		hash = hash.wrapping_mul(0x100000001b3);
	}
	hash
}

pub fn jenkins(bytes: &[u8]) -> u64 {
	let mut hash = 0;
	for b in bytes {
		hash += *b as u64;
		hash += hash.wrapping_shr(10);
		hash ^= hash.wrapping_shl(6);
	}
	hash += hash.wrapping_shr(3);
	hash ^= hash.wrapping_shl(11);
	hash += hash.wrapping_shr(15);
	hash
}

/// Simulates a bloom filter by accepting an array of three hash functions, a
/// data vector, and another value to query. Returns `true` if `value` is
/// "probably" in the data vector and `false` if it is definitely not in the 
/// data vector.
pub fn bloom(data: &Vec<&str>, hashes: [fn(&u[u8]) -> u64; 3], value: &str) -> bool {
	// TODO
	// Initialize a fixed size boolean array for storage; store all zeros in it
	// Insert element from data vector
	// Hash each element using each of hash function
	// For each hashed value y = h(x), set the value at index y in array to 1
	// Test if a value x is in the data vector
	// Hash x using each of hash function
	// If values at all of the hash indices in array are 1's, return "probably true"
	// Otherwise, "definitely false" 


}
