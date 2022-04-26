// Cursor implementation
// Reference: https://doc.rust-lang.org/1.60.0/std/collections/linked_list/struct.CursorMut.html
use crate::sixth::{LinkedList, Link};
use std::marker::PhantomData;

impl<T> LinkedList<T> {
  pub fn cursor_mut(&mut self) -> CursorMut<'_, T> {
    CursorMut { 
      list: self, 
      cur: None, 
      index: None,
    }
  }
}

pub struct CursorMut<'a, T> {
  cur: Link<T>,
  list: &'a mut LinkedList<T>,
  index: Option<usize>,
}
impl<'a, T> CursorMut<'a, T> {
  pub fn index(&self) -> Option<usize> {
    self.index
  }
  pub fn move_next(&mut self) {
    if let Some(cur) = self.cur {
      unsafe {
        // We're on a real element, go to its next (back)
        self.cur = (*cur.as_ptr()).back;
        if self.cur.is_some() {
          *self.index.as_mut().unwrap() += 1;
        } else {
          // We just walked to the ghost, no more index
          self.index = None;
        }
      }
    } else if !self.list.is_empty() {
      // We're at the ghost, and there is a real front, so move to it!
      self.cur = self.list.front;
      self.index = Some(0)
    } else {
      // We're at the ghost, but that's the only element... do nothing.
    }
  }
  pub fn move_prev(&mut self) {
    if let Some(cur) = self.cur {
      unsafe {
        // We're on a real element, go to its previous (front)
        self.cur = (*cur.as_ptr()).front;
        if self.cur.is_some() {
          *self.index.as_mut().unwrap() -= 1;
        } else {
          // We just walked to the ghost, no more index
          self.index = None;
        }
      }
    } else if !self.list.is_empty() {
      // We're at the ghost, and there is a real back, so move to it!
      self.cur = self.list.back;
      self.index = Some(self.list.len - 1)
    } else {
      // We're at the ghost, but that's the only element... do nothing.
    }
  }
  pub fn current(&mut self) -> Option<&mut T> {
    unsafe {
      self.cur.map(|node| &mut (*node.as_ptr()).elem)
    }
  }
  pub fn peek_next(&mut self) -> Option<&mut T> {
    unsafe {
      let next = if let Some(cur) = self.cur {
        // Normal case, try to follow the cur node's back pointer
        (*cur.as_ptr()).back
      } else {
        // Ghost case, try to use the list's front pointer
        self.list.front
      };
      // Yield the element if the next node exists
      next.map(|node| &mut (*node.as_ptr()).elem)
    }
  }
  pub fn peek_prev(&mut self) -> Option<&mut T> {
    unsafe {
      let prev = if let Some(cur) = self.cur {
        // Normal case, try to follow the cur node's front pointer
        (*cur.as_ptr()).front
      } else {
        // Ghost case, try to use the list's back pointer
        self.list.back
      };
      // Yield the element if the prev node exists
      prev.map(|node| &mut (*node.as_ptr()).elem)
  }
  }
  pub fn split_before(&mut self) -> LinkedList<T> {
    if let Some(cur) = self.cur {
      // We are pointing at a real element, so the list is non-empty.
      unsafe {
        // Current state
        let old_len = self.list.len;
        let old_idx = self.index.unwrap();
        let prev = (*cur.as_ptr()).front;
        
        // What self will become
        let new_len = old_len - old_idx;
        let new_front = self.cur;
        let new_back = self.list.back;
        let new_idx = Some(0);

        // What the output will become
        let output_len = old_len - new_len;
        let output_front = self.list.front;
        let output_back = prev;

        // Break the links between cur and prev
        if let Some(prev) = prev {
          (*cur.as_ptr()).front = None;
          (*prev.as_ptr()).back = None;
        }

        // Produce the result:
        self.list.len = new_len;
        self.list.front = new_front;
        self.list.back = new_back;
        self.index = new_idx;

        LinkedList {
          front: output_front,
          back: output_back,
          len: output_len,
          _boo: PhantomData,
        }
      }
    } else {
      // We're at the ghost, just replace our list with an empty one.
      // No other state needs to be changed.
      std::mem::replace(self.list, LinkedList::new())
    }
  }
  pub fn split_after(&mut self) -> LinkedList<T> {
    if let Some(cur) = self.cur {
      // We are pointing at a real element, so the list is non-empty.
      unsafe {
        // Current state
        let old_len = self.list.len;
        let old_idx = self.index.unwrap();
        let next = (*cur.as_ptr()).back;
        
        // What self will become
        let new_len = old_idx + 1;
        let new_back = self.cur;
        let new_front = self.list.front;
        let new_idx = Some(old_idx);

        // What the output will become
        let output_len = old_len - new_len;
        let output_front = next;
        let output_back = self.list.back;

        // Break the links between cur and next
        if let Some(next) = next {
          (*cur.as_ptr()).back = None;
          (*next.as_ptr()).front = None;
        }

        // Produce the result:
        self.list.len = new_len;
        self.list.front = new_front;
        self.list.back = new_back;
          self.index = new_idx;

        LinkedList {
          front: output_front,
          back: output_back,
          len: output_len,
          _boo: PhantomData,
        }
      }
    } else {
      // We're at the ghost, just replace our list with an empty one.
      // No other state needs to be changed.
      std::mem::replace(self.list, LinkedList::new())
    }
  }
  pub fn splice_before(&mut self, mut input: LinkedList<T>) {
    unsafe {
      // We can either `take` the input's pointers or `mem::forget`
      // it. Using `take` is more responsible in case we ever do custom
      // allocators or something that also needs to be cleaned up!
      if input.is_empty() {
        // Input is empty, do nothing.
      } else if let Some(cur) = self.cur {
        // Both lists are non-empty
        let in_front = input.front.take().unwrap();
        let in_back = input.back.take().unwrap();

        if let Some(prev) = (*cur.as_ptr()).front {
          // General Case, no boundaries, just internal fixups
          (*prev.as_ptr()).back = Some(in_front);
          (*in_front.as_ptr()).front = Some(prev);
          (*cur.as_ptr()).front = Some(in_back);
          (*in_back.as_ptr()).back = Some(cur);
        } else {
          // No prev, we're appending to the front
          (*cur.as_ptr()).front = Some(in_back);
          (*in_back.as_ptr()).back = Some(cur);
          self.list.front = Some(in_front);
        }
        // Index moves forward by input length
        *self.index.as_mut().unwrap() += input.len;
      } else if let Some(back) = self.list.back {
        // We're on the ghost but non-empty, append to the back
        let in_front = input.front.take().unwrap();
        let in_back = input.back.take().unwrap();

        (*back.as_ptr()).back = Some(in_front);
        (*in_front.as_ptr()).front = Some(back);
        self.list.back = Some(in_back);
      } else {
        // We're empty, become the input, remain on the ghost
        std::mem::swap(self.list, &mut input);
      }

      self.list.len += input.len;
      // Not necessary but Polite To Do
      input.len = 0;
      // Input dropped here
    }
  }
  pub fn splice_after(&mut self, mut input: LinkedList<T>) {
    unsafe {
      // We can either `take` the input's pointers or `mem::forget`
      // it. Using `take` is more responsible in case we ever do custom
      // allocators or something that also needs to be cleaned up!
      if input.is_empty() {
        // Input is empty, do nothing.
      } else if let Some(cur) = self.cur {
        // Both lists are non-empty
        let in_front = input.front.take().unwrap();
        let in_back = input.back.take().unwrap();

        if let Some(next) = (*cur.as_ptr()).back {
          // General Case, no boundaries, just internal fixups
          (*next.as_ptr()).front = Some(in_back);
          (*in_back.as_ptr()).back = Some(next);
          (*cur.as_ptr()).back = Some(in_front);
          (*in_front.as_ptr()).front = Some(cur);
        } else {
          // No next, we're appending to the back
          (*cur.as_ptr()).back = Some(in_front);
          (*in_front.as_ptr()).front = Some(cur);
          self.list.back = Some(in_back);
        }
        // Index doesn't change
      } else if let Some(front) = self.list.front {
        // We're on the ghost but non-empty, append to the front
        let in_front = input.front.take().unwrap();
        let in_back = input.back.take().unwrap();

        (*front.as_ptr()).front = Some(in_back);
        (*in_back.as_ptr()).back = Some(front);
        self.list.front = Some(in_front);
      } else {
        // We're empty, become the input, remain on the ghost
        std::mem::swap(self.list, &mut input);
      }

      self.list.len += input.len;
      // Not necessary but Polite To Do
      input.len = 0;
      
      // Input dropped here
    }        
  }
}

#[cfg(test)]
mod test {
  use crate::sixth::LinkedList;

  #[test]
  fn test_cursor_move_peek() {
      let mut m: LinkedList<u32> = LinkedList::new();
      m.extend([1, 2, 3, 4, 5, 6]);
      let mut cursor = m.cursor_mut();
      cursor.move_next();
      assert_eq!(cursor.current(), Some(&mut 1));
      assert_eq!(cursor.peek_next(), Some(&mut 2));
      assert_eq!(cursor.peek_prev(), None);
      assert_eq!(cursor.index(), Some(0));
      cursor.move_prev();
      assert_eq!(cursor.current(), None);
      assert_eq!(cursor.peek_next(), Some(&mut 1));
      assert_eq!(cursor.peek_prev(), Some(&mut 6));
      assert_eq!(cursor.index(), None);
      cursor.move_next();
      cursor.move_next();
      assert_eq!(cursor.current(), Some(&mut 2));
      assert_eq!(cursor.peek_next(), Some(&mut 3));
      assert_eq!(cursor.peek_prev(), Some(&mut 1));
      assert_eq!(cursor.index(), Some(1));

      let mut cursor = m.cursor_mut();
      cursor.move_prev();
      assert_eq!(cursor.current(), Some(&mut 6));
      assert_eq!(cursor.peek_next(), None);
      assert_eq!(cursor.peek_prev(), Some(&mut 5));
      assert_eq!(cursor.index(), Some(5));
      cursor.move_next();
      assert_eq!(cursor.current(), None);
      assert_eq!(cursor.peek_next(), Some(&mut 1));
      assert_eq!(cursor.peek_prev(), Some(&mut 6));
      assert_eq!(cursor.index(), None);
      cursor.move_prev();
      cursor.move_prev();
      assert_eq!(cursor.current(), Some(&mut 5));
      assert_eq!(cursor.peek_next(), Some(&mut 6));
      assert_eq!(cursor.peek_prev(), Some(&mut 4));
      assert_eq!(cursor.index(), Some(4));
  }

  #[test]
    fn test_cursor_mut_insert() {
      let mut m: LinkedList<u32> = LinkedList::new();
      m.extend([1, 2, 3, 4, 5, 6]);
      let mut cursor = m.cursor_mut();
      cursor.move_next();
      cursor.splice_before(Some(7).into_iter().collect());
      cursor.splice_after(Some(8).into_iter().collect());
      assert_eq!(
        m.iter().cloned().collect::<Vec<_>>(),
        &[7, 1, 8, 2, 3, 4, 5, 6]
      );
      let mut cursor = m.cursor_mut();
      cursor.move_next();
      cursor.move_prev();
      cursor.splice_before(Some(9).into_iter().collect());
      cursor.splice_after(Some(10).into_iter().collect());
      assert_eq!(
        m.iter().cloned().collect::<Vec<_>>(),
        &[10, 7, 1, 8, 2, 3, 4, 5, 6, 9]
      );
      let mut m: LinkedList<u32> = LinkedList::new();
      m.extend([1, 8, 2, 3, 4, 5, 6]);
      let mut cursor = m.cursor_mut();
      cursor.move_next();
      let mut p: LinkedList<u32> = LinkedList::new();
      p.extend([100, 101, 102, 103]);
      let mut q: LinkedList<u32> = LinkedList::new();
      q.extend([200, 201, 202, 203]);
      cursor.splice_after(p);
      cursor.splice_before(q);
      assert_eq!(
          m.iter().cloned().collect::<Vec<_>>(),
          &[200, 201, 202, 203, 1, 100, 101, 102, 103, 8, 2, 3, 4, 5, 6]
      );
      let mut cursor = m.cursor_mut();
      cursor.move_next();
      cursor.move_prev();
      let tmp = cursor.split_before();
      assert_eq!(m.into_iter().collect::<Vec<_>>(), &[]);
      m = tmp;
      let mut cursor = m.cursor_mut();
      cursor.move_next();
      cursor.move_next();
      cursor.move_next();
      cursor.move_next();
      cursor.move_next();
      cursor.move_next();
      cursor.move_next();
      let tmp = cursor.split_after();
      assert_eq!(
        tmp.into_iter().collect::<Vec<_>>(),
        &[102, 103, 8, 2, 3, 4, 5, 6]
      );
      assert_eq!(
        m.iter().cloned().collect::<Vec<_>>(),
        &[200, 201, 202, 203, 1, 100, 101]
      );
    }
}