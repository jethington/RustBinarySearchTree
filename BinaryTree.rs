mod BST {

  use std::cmp::PartialEq;

  pub struct Tree {
    head: Option<Box<Node>>,
  }

  impl Tree {
    pub fn new() -> Tree {
      Tree{
        head: None,
      }
    }
    
    pub fn add(&mut self, to_add: i32) -> &mut Tree {
      match self.head {
        Some(ref mut n) => n.add(to_add),
        None => self.head = Some(Box::new(Node::new(to_add))),
      }
      self
    }
    
    pub fn search(&self, target: i32) -> bool {
      match self.head {
        Some(ref n) => n.search(target),
        None => false,
      }
    }
    
    pub fn remove(&mut self, target: i32) -> &mut Tree {
      self.head.remove(target);
      self
    }
    
    pub fn print(&self) {
      match self.head {
        Some(ref n) => n.print(),
        None => println!("EMPTY"),
      }
    }
  }

  struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
  }

  impl Node {
    fn new(val: i32) -> Node {
      Node{
        val: val,
        left: None,
        right: None,
      }
    }
    
    // figure out which value in this sub-tree to promote, and then swap values with that node
    // this is called on the node to be replaced
    // so you know it has both right and left sub-nodes
    fn promote_replace(&mut self) {
      let v = self.val;   
      match self.right {
        Some(ref mut n) => {
          self.val = n.min_replace(v);
        }
        None => unreachable!(),
      }
    }
    
    // replace smallest value in this sub-tree
    fn min_replace(&mut self, replace: i32) -> i32 {
      match self.left {
        Some(ref mut n) => n.min_replace(replace),
        None => {
          let result = self.val;
          self.val = replace;
          result
        }
      }
    }
    
    fn add(&mut self, to_add: i32) {
      if to_add > self.val {
        match self.right {
          Some(ref mut n) => n.add(to_add),
          None            => self.right = Some(Box::new(Node::new(to_add))),
        }
      }
      else if to_add < self.val {
        match self.left {
          Some(ref mut n) => n.add(to_add),
          None            => self.left = Some(Box::new(Node::new(to_add))),
        }
      }
      else {
        return; // no duplicates allowed in this tree
      }
    }
    
    fn search(&self, target: i32) -> bool {
      if self.val == target {
        true
      }
      else if target > self.val {
        match self.right {
          Some(ref n) => n.search(target),
          None        => false,
        }
      }
      else {
        match self.left {
          Some(ref n) => n.search(target),
          None        => false,
        }
      }
    }
    
    fn print(&self) {
      match self.left {
        Some(ref n) => n.print(),
        None => (),
      }
      println!("{}", self.val);
      match self.right {
        Some(ref n) => n.print(),
        None => (),
      }
    }
  }

  impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
      if self.val != other.val { return false; }
      let left_equal: bool = match (&self.left, &other.left) {
        (&None, &None) => true,
        (&None, &Some(_)) => false,
        (&Some(_), &None) => false,
        (&Some(ref left_1), &Some(ref left_2)) => left_1 == left_2,
      };
      if !left_equal { return false; }
      let right_equal: bool = match (&self.right, &other.right) {
        (&None, &None) => true,
        (&None, &Some(_)) => false,
        (&Some(_), &None) => false,
        (&Some(ref left_1), &Some(ref left_2)) => left_1 == left_2,
      };
      return right_equal;
    }
  }

  impl PartialEq for Tree {
    fn eq(&self, other: &Tree) -> bool {
      match (&self.head, &other.head) {
        (&None, &None) => true,
        (&None, &Some(_)) => false,
        (&Some(_), &None) => false,
        (&Some(ref t1), &Some(ref t2)) => t1 == t2,
      }
    }
  }
  
  trait Remove {
    fn remove(&mut self, target: i32);
  }
  
  impl Remove for Option<Box<Node>> {
    fn remove(&mut self, target: i32) {
      let sub_nodes;
      match *self {
        Some(ref mut node_ref) => {
          if target < node_ref.val {
            node_ref.left.remove(target);
            return;
          }
          else if target > node_ref.val {
            node_ref.right.remove(target);
            return;
          }
          else {
            // found the target, need to remove it
            sub_nodes = match (&node_ref.left, &node_ref.right) {
              (&None, &None) => (false, false),
              (&Some(_), &None) => (true, false),
              (&None, &Some(_)) => (false, true),
              (&Some(_), &Some(_)) => (true, true),
            };
          }
        },
        None => return, // think this means it is safe even if there is no target to remove
      }
      match sub_nodes {
        (false, false) => {
          *self = None;
        }
        (true, false) => {
          *self = self.take().unwrap().left;
        }
        (false, true) => {
          *self = self.take().unwrap().right;
        }
        (true, true) => {
          self.as_mut().unwrap().promote_replace();
          self.as_mut().unwrap().right.remove(target);
        }
      } 
    }
  }
}

#[test]
fn test_equal() {
  let mut t1 = BST::Tree::new();
  let mut t2 = BST::Tree::new();
  
  // empty trees should compare equal
  assert!(t1 == t2); // note: using assert_eq! is better?
  
  t1.add(3);
  assert!(t1 != t2);
  
  t2.add(3);
  assert!(t1 == t2);
  
  t1.add(1)
    .add(0)
    .add(2);
  t2.add(1)
    .add(0)
    .add(2);
  assert!(t1 == t2);
  
  t1.add(5)
    .add(6);
  t2.add(5)
    .add(6);
  assert!(t1 == t2);
}

#[test]
fn test_remove_head() {
  let mut t = BST::Tree::new();
  
  // remove head, no children
  t.add(42);
  t.remove(42);
  let mut expected = BST::Tree::new();
  assert!(t == expected);
  
  // remove head, one child
  t.add(4)
   .add(3);
  t.remove(4);
  expected.add(3);
  assert!(t == expected);
  
  // remove head, two children
  t.add(0)
   .add(1)
   .add(-1)
   .add(5)
   .add(4)
   .add(6)
   .remove(3);

  expected = BST::Tree::new();
  expected.add(4)
          .add(0)
          .add(1)
          .add(-1)
          .add(5)
          .add(6);
  assert!(t == expected);
}

#[test]
fn test_remove() {
  // remove node with no children
  let mut t = BST::Tree::new();
  t.add(3)
   .add(1)
   .add(2)
   .add(5)
   .add(0)
   .remove(5);
  
  let mut expected = BST::Tree::new();
  expected.add(3)
          .add(1)
          .add(2)
          .add(0);
  assert!(t == expected);
  
  // remove node with two children
  t.remove(1);
  expected = BST::Tree::new();
  expected.add(3)
          .add(2)
          .add(0);
  assert!(t == expected);
  
  // remove node with one child
  t.remove(2);
  expected = BST::Tree::new();
  expected.add(3)
          .add(0);
  assert!(t == expected);
}

fn main() {
  let mut t = BST::Tree::new();
  t.add(10)
   .add(4)
   .add(13)
   .add(0);
  t.print();
  t.search(13);
  println!("test");
  t.remove(0)
   .remove(10);
  t.print();
}