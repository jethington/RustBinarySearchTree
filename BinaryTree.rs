use std::cmp::PartialEq;

struct Tree {
  head: Option<Box<Node>>,
}

impl Tree {
  fn new() -> Tree {
    Tree{
      head: None,
    }
  }
  
  fn add(&mut self, to_add: i32) -> &mut Tree {
    match self.head {
      Some(ref mut n) => n.add(to_add),
      None => self.head = Some(Box::new(Node::new(to_add))),
    }
    self
  }
  
  fn search(&self, target: i32) -> bool {
    match self.head {
      Some(ref n) => n.search(target),
      None => false,
    }
  }
  
  fn remove(&mut self, target: i32) {
    let mut value_present = false;
    match self.head {
      Some(ref mut n) => {
        value_present = n.search(target);
      }
      None => (),
    }
    if value_present {
      self.remove_helper(target);
    }
  }
  
  // this function might be unnecessary
  fn remove_helper(&mut self, target: i32) {
    let mut sub_nodes = (false, false);
    match self.head {
      Some(ref mut n) => {
        if n.val == target {
          // need to remove the head
          sub_nodes = match (&n.left, &n.right) {
            (&None, &None) => (false, false),
            (&Some(_), &None) => (true, false),
            (&None, &Some(_)) => (false, true),
            (&Some(_), &Some(_)) => (true, true),
          };
        }
        else {
          n.remove(target);
          return;
        }
      }
      None => {
        return;
      }
    }
    match sub_nodes {
      (false, false) => {
        self.head = None;
      }
      (true, false) => {
        self.head = Some(self.head.take().unwrap().left.unwrap());       
      }
      (false, true) => {
        self.head = Some(self.head.take().unwrap().right.unwrap());
      }
      (true, true) => {
        // promote a node from the sub tree
        match self.head {
          Some(ref mut to_remove) => {
            to_remove.promote_replace();
          }
          None => {
            return; // should be unreachable
          }
        }
        self.remove_swapped(target); // now that the values are swapped, remove the correct node
      }
    }
  }
  
  fn remove_swapped(&mut self, target: i32) {
    match self.head {
      Some(ref mut n) => {
        n.remove_swapped_head(target);
      }
      None => {
        // should be unreachable
      }
    }
  }
  
  fn print(&self) {
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
  
  fn remove(&mut self, target: i32) {
    let mut sub_nodes = (false, false);
    if target > self.val {  // if target == self, go right (this happens after a new node has been promoted)
      match self.right {
        Some(ref mut n) => {
          if n.val == target { 
            // found it
            sub_nodes = match (&n.left, &n.right) {
              (&None, &None) => (false, false),
              (&Some(_), &None) => (true, false),
              (&None, &Some(_)) => (false, true),
              (&Some(_), &Some(_)) => (true, true),
            };
          }
          else { 
            n.remove(target);
            return;
          }
        }
        None => {
          return; // should be unreachable
        }
      }
      
      // self = parent of node to remove
      // self.right is present and the node to remove
      match sub_nodes {
        (false, false) => {
          self.right = None;
        }
        (true, false) => {
          self.right = self.right.take().unwrap().left;           
        }
        (false, true) => {
          self.right = self.right.take().unwrap().right;
        }
        (true, true) => {
          // promote a node from the sub tree
          match self.right {
            Some(ref mut to_remove) => {
              to_remove.promote_replace();
              to_remove.remove_swapped(target);
            }
            None => {
              return; // should be unreachable
            }
          }
        }
      }
    }
    
    // self = parent of node to remove
    // self.left is present and the node to remove
    else {
      match self.left {
        Some(ref mut n) => {
          if n.val == target { 
            // found it
            sub_nodes = match (&n.left, &n.right) {
              (&None, &None) => (false, false),
              (&Some(_), &None) => (true, false),
              (&None, &Some(_)) => (false, true),
              (&Some(_), &Some(_)) => (true, true),
            };
          }
          else { 
            n.remove(target); 
            return;
          }
        }
        None => {
          return; // should be unreachable
        } 
      }
      match sub_nodes {
        (false, false) => {
          self.left = None;
          //println!("test");
        }
        (true, false) => {
          self.left = self.left.take().unwrap().left;
        }
        (false, true) => {
          self.left = self.left.take().unwrap().right;
        }
        (true, true) => {
          // promote a node from the sub tree
          match self.left {
            Some(ref mut to_remove) => {
              to_remove.promote_replace();
              to_remove.remove_swapped(target);
              //println!("test");
            }
            None => {
              return; // should be unreachable
            }
          }
        }
      }
    }
  }
  
  fn remove_swapped(&mut self, target: i32) {
    match self.right {
      Some(ref mut n) => {
        if n.val == target { 
          // found it, don't return
        }
        else { 
          n.remove(target);
          return;
        }
      }
      None => {
        return; // should be unreachable
      }
    }
    
    // no left sub-nodes if you get here, since they would have been lower and therefore swapped in instead
    // if the node-to-remove has a right sub-node, grab it
    // if not, then this code does no harm
    self.right = self.right.take().unwrap().right;
  }
  
  fn remove_swapped_head(&mut self, target: i32) {
    match self.right {
      Some(ref mut r) => {
        if r.val == target { 
          // found it, don't return
        }
        else { 
          r.remove(target);
          return;
        }
      }
      None => {
        return; // should be unreachable
      }
    }
    self.right = self.right.take().unwrap().right;
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
      None => (),
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

#[test]
fn test_equal() {
  let mut t1 = Tree::new();
  let mut t2 = Tree::new();
  
  // empty trees should compare equal
  assert!(t1 == t2);
  
  t1.add(3);
  assert!(t1 != t2);
  
  t2.add(3);
  assert!(t1 == t2);
  
  t1.add(1);
  t1.add(0);
  t1.add(2);
  t2.add(1);
  t2.add(0);
  t2.add(2);
  assert!(t1 == t2);
  
  t1.add(5);
  t1.add(6);
  t2.add(5);
  t2.add(6);
  assert!(t1 == t2);
}

#[test]
fn test_remove() {
  let mut t = Tree::new();
  t.add(3);
  t.add(1);
  t.add(2);
  t.add(5);
  t.add(0);
  
  // remove node with no children
  t.remove(5);
  let mut expected = Tree::new();
  expected.add(3);
  expected.add(1);
  expected.add(2);
  expected.add(0);
  assert!(t == expected);
  
  // remove node with two children
  t.remove(1);
  expected = Tree::new();
  expected.add(3);
  expected.add(2);
  expected.add(0);
  assert!(t == expected);
  
  // remove node with one child
  t.remove(2);
  expected = Tree::new();
  expected.add(3);
  expected.add(0);
  assert!(t == expected);
  
  t.add(1);
  t.add(-1);
  t.add(5);
  t.add(4);
  t.add(6);
  
  // remove head
  t.remove(3);

  expected = Tree::new();
  expected.add(4);
  expected.add(0);
  expected.add(1);
  expected.add(-1);
  expected.add(5);
  expected.add(6);
  assert!(t == expected);
}

fn main() {
  let mut t = Tree::new();
  t.add(3);
  t.add(0);
  t.add(-1);
  t.add(1);
  t.add(5);
  t.add(4);
  t.add(6);
  t.remove(3);
  println!("test");
  t.print();
}