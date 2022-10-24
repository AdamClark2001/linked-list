struct LinkedList {
  head: Link,
}

impl LinkedList {
  fn new() -> LinkedList {
    LinkedList { 
      head: None,
    }
  }

  fn push(&mut self, element: i32) {
    let old_head = std::mem::replace(&mut self.head, None);
    let new_head = Box::new(Node {
      element,
      next: old_head,
    });

    self.head = Some(new_head);
  }

  fn reverse(&mut self) {
    // let mut cur = std::mem::replace(&mut self.head, None);
    // let mut prev = None;
    // let mut next;
    let mut cur = self.head.take();
    let mut prev = None;

    // while !cur.is_none() {
    //   //next = cur.as_ref().unwrap().next;
    //   next = std::mem::replace(&mut cur.as_ref().unwrap().next, None);
    //   cur.as_ref().unwrap().next = prev;
    //   prev = cur;
    //   cur = next;
    // }
    while let Some(mut cur_inner) = cur.take() {
      let next = cur_inner.next.take();
      cur_inner.next = prev.take();
      prev = Some(cur_inner);
      cur = next;
    }

    //self.head = prev;

    self.head = prev.take();
  }

  fn build(&mut self, length: i32) {
    for x in (1..length + 1).rev() {
      self.push(x);
    }
  }

  fn print(&mut self) {
    let mut link = &self.head;
    let mut output = String::new();

    while !link.as_ref().unwrap().next.is_none() {
      let value = &link.as_ref().unwrap().element.to_string();
      output.push_str(&value);
      output.push_str(" -> ");
      link = &link.as_ref().unwrap().next;
    }

    output.push_str(&link.as_ref().unwrap().element.to_string());

    println!("{}", output);
  }
}

struct Node {
  element: i32,
  next: Link,
}

type Link = Option<Box<Node>>;

fn main() {
  use std::io;
  use std::time::Instant;

  let mut node_count = 0;
  
  while node_count < 1 {
    println!("How many nodes in the linked list? ");

    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("failed to read line");
    let res = buffer.trim_end();
    match res.parse::<i32>() {
      Ok(int) => node_count = int,
      Err(_) => println!("Not a valid number"),
    }
  }

  let mut list = LinkedList::new();
  println!("Generating linked list with {} nodes(s)", node_count);
  list.build(node_count);
  list.print();

  println!("Reversing the linked list");
  let start = Instant::now();
  list.reverse();
  let elapsed = start.elapsed();

  list.print();

  println!("Execution time: {:.2?}ns", elapsed.as_nanos());
}