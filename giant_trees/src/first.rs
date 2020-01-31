// first.rs
// January 30, 2020
// Qui Le

// public struct for Binary Search Tree
pub struct BST {
	root: Link,
}

// private Link enum
enum Link {
	Empty,
	More(Box<Node>),
}

// private Node struct
struct Node {
	// Data inside node
	data: i32,
	// Left child
	left: Link,
	// Right child
	right: Link,
}