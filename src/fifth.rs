
pub struct UnsafeQueue<T> {
  head: Link<T>,
  tail: *mut Node<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
  elem: T,
  next: Link<T>,
}

impl<T> UnsafeQueue<T> {
  pub fn new() -> Self {
    UnsafeQueue { head: 0 as *mut _, tail: 0 as *mut _ } 
    // lmfao what the frickery did I just do
    // did I just cast 0 to a mutable pointer? 
    // the frick??? 
  }

  // Push an element to the end of a queue
  pub fn push(&mut self, elem: T) {
    // Put the box in the right place, and then grab a reference to its Node
    let raw_tail: *mut _ = Box::into_raw(Box::new(Node {
      elem: elem,
      next: 0 as *mut _, // this is the new last element of the queue
    }));
    
    if self.tail.is_null() {
      self.head = raw_tail;
    } else {
      unsafe {
        (*self.tail).next = raw_tail;
      }
    }

    self.tail = raw_tail
  }
  pub fn pop(&mut self) -> Option<T> {
    unsafe {
      if self.head.is_null() {
        None
      } else {
        let head = Box::from_raw(self.head);
        self.head = head.next;

        if self.head.is_null() {
          self.tail = std::ptr::null_mut();
        }
        Some(head.elem)
      }
    }
  }
  pub fn peak(&self) -> Option<&T> {
    unsafe {
      self.head.as_ref().map(|node|  &node.elem)
    }
  }
  pub fn peak_mut(&self) -> Option<&mut T> {
    unsafe {
      self.head.as_mut().map(|node| &mut node.elem)
    }
  }
}
impl<T> Drop for UnsafeQueue<T> {
  fn drop(&mut self) {
    while let Some(_) = self.pop() { }
  }
}

pub struct IntoIter<T>(UnsafeQueue<T>);

pub struct Iter<'a, T> {
  next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
  next: Option<&'a mut Node<T>>,
}

impl<T> UnsafeQueue<T> {
  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }

  pub fn iter(&self) -> Iter<'_, T> {
    unsafe {
      Iter { next: self.head.as_ref() }
    }
  }

  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    unsafe {
      IterMut { next: self.head.as_mut() }
    }
  }
}

impl<T> Iterator for IntoIter<T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    unsafe {
      self.next.map(|node| {
        self.next = node.next.as_ref();
        &node.elem
      })
    }
  }
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|node| {
      unsafe {
        self.next = node.next.as_mut();
      }
      &mut node.elem
    })
  }
}


#[cfg(test)]
mod test {
  use super::UnsafeQueue;
  #[test]
  fn basics() {
    let mut list = UnsafeQueue::new();

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), None);

    // Check the exhaustion case fixed the pointer right
    list.push(6);
    list.push(7);

    // Check normal removal
    assert_eq!(list.pop(), Some(6));
    assert_eq!(list.pop(), Some(7));
    assert_eq!(list.pop(), None);
  }
}
