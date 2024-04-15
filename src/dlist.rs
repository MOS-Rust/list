use core::ptr;
use core::mem;

pub struct DList<T> {
    head: Option<Box<Node<T>>>,
}

struct Rawlink<T> {
    p: *mut T,
}

impl<T> Copy for Rawlink<T> {}

impl<T> Clone for Rawlink<T> {
    fn clone(&self) -> Rawlink<T> {
        Rawlink { p: self.p }
    }
}

pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    prev: Rawlink<Node<T>>,
    value: T,
}

impl<T> DList<T> {
    pub fn new() -> DList<T> {
        DList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(ref node) = *current {
            count += 1;
            current = &node.next;
        }
        count
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        self.push_front_node(new_node);
    }

    pub fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        match self.head {
            None => {
                node.prev = Rawlink::none();
                self.head = Some(node);
            }
            Some(ref mut head) => {
                head.prev = Rawlink::some(&mut *node);
                node.next = Some(mem::replace(&mut self.head, None).unwrap());
                self.head = Some(node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take();
            old_head.value
        })
    }

    pub fn iter(&self) -> DListIterator<T> {
        DListIterator {
            next: &self.head,
            nelem: self.len(),
        }
    }
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            next: None,
            prev: Rawlink { p: ptr::null_mut() },
            value: value,
        }
    }
}

impl<T> Rawlink<T> {
    fn none() -> Rawlink<T> {
        Rawlink { p: ptr::null_mut() }
    }

    fn some(n: &mut T) -> Rawlink<T> {
        Rawlink { p: n }
    }

    fn resolve_immut<'a>(&self) -> Option<&'a T> {
        unsafe {
            self.p.as_ref()
        }
    }

    fn resolve<'a>(&self) -> Option<&'a mut T> {
        unsafe {
            self.p.as_mut()
        }
    }

    fn take(&mut self) -> Rawlink<T> {
        mem::replace(self, Rawlink::none())
    }
}

pub struct DListIterator<'a, T: 'a> {
    next: &'a Option<Box<Node<T>>>,
    nelem: usize,
}

impl <'a, T> Iterator for DListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.nelem == 0 {
            return None;
        }
        self.next.as_ref().map(|node| {
            self.nelem -= 1;
            self.next = &node.next;
            &node.value
        })
    }
     fn size_hint(&self) -> (usize, Option<usize>) {
        (self.nelem, Some(self.nelem))
     }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut list = DList::new();
        assert_eq!(list.len(), 0);
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_iter() {
        let mut list = DList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}