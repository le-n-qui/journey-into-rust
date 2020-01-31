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