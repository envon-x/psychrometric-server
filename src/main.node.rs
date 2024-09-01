/**
  Source book: Programming Rust 2nd Edition-2021.pdf 
 */

enum Option<T> {
  None,
  Some(T),
}

enum Result<T,E> {
  Ok(T),
  Err()
}
use std::fmt::Binary;

use self::BinaryTree::*;

fn main() {

  let mercury_tree = NonEmpty(Box::new(TreeNode{
    element: "Mercury",
    left: Empty,
    right: Empty,
  }));

  let jupiter_tree = NonEmpty(Box::new(TreeNode {
    element: "Jupiter",
    left: Empty,
    right: Empty,
  }));



  //larger trees can be built from smaller ones:
  let mars_tree = NonEmpty(Box::new(TreeNode {
    element: "Mars",
    left: jupiter_tree,
    right: mercury_tree,
  }));

  let uranus_tree = NonEmpty(Box::new(TreeNode {
    element: "Uranus",
    left: Empty,
    right: Empty,
  }));

  let tree = NonEmpty(Box::new(TreeNode {
    element: "Saturn",
    left: mars_tree,
    right: uranus_tree,
  }));

  let mut tree01 = BinaryTree::Empty;
  for planet in planets {
    tree.add(planet);
  }

  tree01.add("Pluton");
}

/**
Una collección ordenada de `T`'s */
enum BinaryTree<T> {
  Empty,
  NonEmpty(Box<TreeNode<T>>),
}
// Each BinaryTree value is either Empty or NonEmpty.
// If it is Empty, then it contains no DATA at all
// If NonEmpty, Then it has a  Box, a pointer to a heap-allocated TreeNode

/**
Una parte de BinaryTree */
struct TreeNode<T> {
  element: T,
  left: BinaryTree<T>,
  right: BinaryTree<T>,
}
// Each TreeNode value containes on actual element, as well as two more BinaryTree values



impl <T> BinaryTree<T> {

  /**
  page: 427 (on pdf viewer)
   */
  fn add(&mut self, value: T) {
    match *self {
        BinaryTree::Empty => {
        *self = BinaryTree::NonEmpty(Box::new(TreeNode { // If *self is not empty, we math the pattern
        element: value,
          left: BinaryTree::Empty,
          right: BinaryTree::Empty,
        }))
      }
      BinaryTree::NonEmpty(ref mut node) => {
        if value <= node.element {
          node.left.add(value);
        } else {
          node.right.add(value);
        }
      }
    }
  }
}