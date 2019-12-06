#![cfg(test)]

use problem1::{sum, dedup, filter};

// tests_student.rs


//
// Problem 1
//

// Part 1

// overflow on sum of very big numbers
// appropriate response ?
// fn sum needs to return an i32 value in valid cases
#[test]
#[should_panic]
fn test_sum_big_overflow() {
	let a = [100_000_000, 2_000_000_000, 50_000_000];
	sum(&a);
}

#[test]
#[should_panic]
fn test_sum_small_overflow() {
	let a = [-1_000_000_000, -1_000_000_000, -200_000_000];
	sum(&a);
}

#[test]
fn test_sum_zero_and_postive() {
	let a = [0, 10, 20, 3, 5];
	assert_eq!(sum(&a), 38);
}

#[test]
fn test_sum_zero_and_negative() {
	let a = [-2, -3, -6, -1, 0];
	assert_eq!(sum(&a), -12);
}

#[test]
fn test_sum_mix_numbers() {
	let a = [-5, 0, 5, 10, -2, 4, 0];
	assert_eq!(sum(&a), 12);
}