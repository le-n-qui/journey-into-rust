#![cfg(test)]
// tests_student.rs


//
// Problem 1
//

// Part 1

// Import other modules
use crate::problem1::{sum, dedup, filter};
use crate::problem2::sieve;
use crate::problem3::hanoi;
use crate::problem4::{bloom, djb2, fnv, jenkins};

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

// Part 2
#[test]
fn test_dedup() {
	let v = vec![0, 0, 1, 2, 1, 0, 3, 3];
	assert_eq!(dedup(&v), vec![0,1,2,3]);
}

// Part 3

// function is testing a condition
fn is_odd(x: i32) -> bool {
    (x % 2) != 0
}

fn is_even(x: i32) -> bool {
	(x % 2) == 0
}

// tests below

#[test]
fn test_filter_odd() {
	let v = vec![1, 2, 3, 4, 5];
	assert_eq!(filter(&v, is_odd), vec![1, 3, 5]);
}

#[test]
fn test_filter_even() {
	let v = vec![0,1,2,3,4,5,6,7,8];
	assert_eq!(filter(&v, is_even), vec![0,2,4,6,8]);
}

//
// Problem 2
//

#[test]
fn test_sieve_small_n() {
	assert_eq!(sieve(7), vec![2,3,5]);
}

#[test]
fn test_sieve_big_n() {
	assert_eq!(sieve(30), vec![2,3,5,7,11,13,17,19,23,29]);
}

//
// Problem 3
//

#[test]
fn test_hanoi_1_disks() {
    let result = hanoi(1);
    assert_eq!(vec![(1, 3)], result);
    assert_eq!(1, result.len());
}

#[test]
fn test_hanoi_3_disks() {
	let result = hanoi(3);
	assert_eq!(vec![(1,3),(1,2),(3,2),(1,3),(2,1),(2,3),(1,3)], result);
	assert_eq!(7, result.len());
}

//
// Problem 4
//

#[test]
fn test_bloom_topics() {
	let topics = vec!["sports", "politics", "education", "finance", "technology", "mortgage", "love"];
	let hashes = [djb2, fnv, jenkins];
	assert_eq!(true, bloom(&topics, hashes, "sports"));
	assert_eq!(true, bloom(&topic, hashes, "investment"));
	assert_eq!(false, bloom(&topic, hashes, "geography"));
}