// first.rs
// January 30, 2020
// Qui Le

// public struct for Binary Search Tree
#[derive(Debug)]
pub struct BST {
	root: Link,
}

// private Link enum
#[derive(Debug)]
enum Link {
	Empty,
	More(Box<Node>),
}

// private Node struct
#[derive(Debug)]
struct Node {
	// Data inside node
	data: i32,
	// Left child
	left: Link,
	// Right child
	right: Link,
}

impl BST {
	pub fn new() -> Self {
		BST { root: Link::Empty }
	}

	pub fn insert(&mut self, item: i32) -> bool {
		implemented!()
	}

	pub fn search(&self, item: i32) -> bool {
		implemented!()
	}
}
