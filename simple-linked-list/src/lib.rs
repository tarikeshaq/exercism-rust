use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Default)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut curr = &self.head;
        let mut res: usize = 0;
        while let Some(node) = curr {
            res += 1;
            curr = &node.next;
        }
        res
    }

    pub fn push(&mut self, element: T) {
        let new_head = Some(Box::new(Node {
            data: element,
            next: None,
        }));

        let old_head = std::mem::replace(&mut self.head, new_head);
        if old_head.is_some() {
            self.head.as_mut().unwrap().next = old_head;
        }
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Copy,
    {
        match std::mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                let res = node.data;
                self.head = node.next;
                Some(res)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(node) => Some(&node.data),
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut other = SimpleLinkedList::new();
        SimpleLinkedList::rev_recursive(self.head, &mut other);
        other
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn rev_recursive(head: Option<Box<Node<T>>>, other: &mut SimpleLinkedList<T>) {
        if let Some(node) = head {
            other.push(node.data);
            SimpleLinkedList::rev_recursive(node.next, other);
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut this = SimpleLinkedList::new();
        let mut iter = iter.into_iter();
        let mut element = iter.next();
        while let Some(data) = element {
            this.push(data);
            element = iter.next();
        }
        this
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T>
where
    T: Copy,
{
    fn into(self) -> Vec<T> {
        let mut res = Vec::new();
        let mut curr = &self.head;
        while let Some(node) = curr {
            res.push(node.data);
            curr = &node.next;
        }
        res.iter().rev().copied().collect()
    }
}
