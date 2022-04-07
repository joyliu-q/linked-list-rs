use std::mem;

struct Node<T> {
  elem: T,
  next: Link<T>,
};

enum Link<T> {
  Empty,
  More(Box<Node<T>>),
};

pub struct List<T> {
  head: Link<T>,
};
impl<T> List<T> {
  // Returns an empty list.
  pub fn new() -> Self {
    List { head: Link::Empty }
  }

  // Pushes an elment to the front of the list.
  pub fn push(&mut self, element: T) {
    let new_node = Box::new(Node {
      elem: element,
      next: mem::replace(&mut self.head, Link::Empty),
    });
    self.head = Link::More(new_node);
  }

  // Pops the first element from the list.
  pub fn pop(&mut self) -> Option<T> {
    let res;
    match mem::replace(&mut self.head, Link::Empty) {
      Link::Empty => res = None,
      Link::More(node) => {
        self.head = node.next;
        res = Some(node.elem);
      }
    }
    res
  }
};
impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut cur_link = mem::replace(&mut self.head, Link::Empty);
    while let Link::More(mut boxed_node) = cur_link {
      cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
    }
  }
};


#[cfg(test)]
mod tests {
    use super::List;
    
    #[test]
    fn first_list_empty() {
      let mut list = List::new() as List<i32>;
      assert_eq!(list.pop(), None);
    }

    #[test]
    fn first_list_push_pop() {
      let mut list = List::new();

      list.push(1);
      list.push(2);
      list.push(3);

      assert_eq!(list.pop(), Some(3));
      assert_eq!(list.pop(), Some(2));

      list.push(4);
      list.push(5);

      // Check normal removal
      assert_eq!(list.pop(), Some(5));
      assert_eq!(list.pop(), Some(4));
      assert_eq!(list.pop(), Some(1));

      // Check none at the end
      assert_eq!(list.pop(), None);
    }

    #[test]
    fn first_list_str() {
      let mut list = List::new();
      assert_eq!(list.pop(), None);

      list.push("!");
      list.push("world");
      list.push("hello");

      assert_eq!(list.pop(), Some("hello"));
      assert_eq!(list.pop(), Some("world"));

      list.push("test");
      list.push("another");

      // Check normal removal
      assert_eq!(list.pop(), Some("another"));
      assert_eq!(list.pop(), Some("test"));
      assert_eq!(list.pop(), Some("!"));

      // Check none at the end
      assert_eq!(list.pop(), None);
    }
}
