pub struct StackList<'a, T> {
    pub data: T,
    pub prev: Option<&'a StackList<'a, T>>,
}

pub struct Iter<'a, T> {
    next: Option<&'a StackList<'a, T>>,
}

impl<'a, T> StackList<'a, T> {
    pub fn push<U>(
        prev: Option<&'a StackList<'a, T>>, 
        data: T, 
        callback: impl FnOnce(& StackList<'a, T>) -> U,
    ) -> U {
        let mut list = StackList { data, prev };
        callback(&mut list)
    }

    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter { next: Some(self) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.prev;
            &node.data
        })
    }
}


#[cfg(test)]
mod test {
    use super::StackList;

    #[test]
    fn elegance() {
        StackList::push(None, 3, |list| {
            assert_eq!(list.data, 3);
            StackList::push(Some(list), 5, |list| {
                assert_eq!(list.data, 5);
                StackList::push(Some(list), 13, |list| {
                    assert_eq!(list.data, 13);
                })
            })
        })
    }

    #[test]
    fn cell() {
        use std::cell::Cell;

        StackList::push(None, Cell::new(3), |list| {
            StackList::push(Some(list), Cell::new(5), |list| {
                StackList::push(Some(list), Cell::new(13), |list| {
                    // Multiply every value in the list by 10
                    for val in list.iter() {
                        val.set(val.get() * 10)
                    }

                    let mut vals = list.iter();
                    assert_eq!(vals.next().unwrap().get(), 130);
                    assert_eq!(vals.next().unwrap().get(), 50);
                    assert_eq!(vals.next().unwrap().get(), 30);
                    assert_eq!(vals.next(), None);
                    assert_eq!(vals.next(), None);
                })
            })
        })
    }

}
