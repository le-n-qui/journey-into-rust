// first.rs
// January 30, 2020
// Qui Le

use std::cmp::Ordering::*;

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
		BST::add(&mut self.root, item)
	}

	fn add(node: &mut Link, item: i32) -> bool {
		let result; // true or false
		match node {
			Link::Empty => { 
				let new_node = Box::new(
					Node {
						data: item,
						left: Link::Empty,
						right: Link::Empty,
					}
				);
				*node = Link::More(new_node);
				result = true;
			},
			Link::More(ref mut n) => {
				match item.cmp(&n.data) {
					Equal => { result = false; },
					Less => { result = BST::add(&mut n.left, item); },
					Greater => { result = BST::add(&mut n.right, item); },
				}
			},
		} 
		result
	}

	pub fn search(&self, item: i32) -> bool {
		BST::find(& self.root, item)
	}

	fn find(node: & Link, item: i32) -> bool {
		let result; // is the search successful?
		match node {
			// if node is empty
			Link::Empty => { result = false; },
			// else look further into reference n 
			Link::More(ref n) => {
				// Compare item with data within reference n
				// cmp() returns three options of Ordering enum
				match item.cmp(&n.data) {
					Equal => { result = true; },
					Less => { result = BST::find(& n.left, item); },
					Greater => { result = BST::find(& n.right, item); },
				}
			},
		}
		result
	}
}

#[cfg(test)]
mod tests {
	// import BST struct
	use super::BST;

    #[test]
    fn insert_into_empty_tree() {
    	let mut t = BST::new();
    	
    	// Add nodes
    	assert_eq!(true, t.insert(10));
    	assert_eq!(true, t.insert(5));
    	assert_eq!(true, t.insert(15));
    	assert_eq!(true, t.insert(9));

    	// Insert number that is already in the tree
    	assert_eq!(false, t.insert(10));
    	assert_eq!(false, t.insert(15));
    }

    #[test]
    fn search_item_in_tree() {
    	let mut t = BST::new();

    	// Add new nodes
    	t.insert(10);
    	t.insert(5);
    	t.insert(15);
    	t.insert(2);
    	t.insert(8);
    	t.insert(12);
    	t.insert(20);

    	// Search item
    	assert_eq!(true, t.search(8));
    	assert_eq!(true, t.search(20));
    	assert_eq!(false, t.search(9));
    }
}