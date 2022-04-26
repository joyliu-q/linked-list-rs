struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct ListIterator<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct ListIteratorMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> Iterator for ListIteratorMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

pub struct List<T> {
    head: Link<T>,
}
impl<T> List<T> {
    // Returns an empty list.
    pub fn new() -> Self {
        List { head: None }
    }

    // Pushes an elment to the front of the list.
    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            elem: element,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Pops the first element from the list.
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // Returns a reference to the first element of the list.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    // Returns a mutable reference to the first element of the list.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    // Returns the linked list as an iterator
    pub fn into_iter(&self) -> ListIterator<'_, T> {
        // the '_ means it's hiding an elided lifetime
        ListIterator {
            next: self.head.as_deref(),
        }
    }

    // Returns the linked list as a mutable iterator
    pub fn into_iter_mut(&mut self) -> ListIteratorMut<'_, T> {
        ListIteratorMut {
            next: self.head.as_deref_mut(),
        }
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn second_list_empty() {
        let mut list = List::new() as List<i32>;
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn second_list_push_pop() {
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
    fn second_list_str() {
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

    #[test]
    fn second_list_peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn second_list_into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
