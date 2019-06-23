use std::mem;

/**
 * [list] - link - node - link - node - link - node - empty link
 * Why?...
 * Internals of the enums are public - would not be able to make other interfaces private
 * Not using only one interface (enum), because it would create junk memory + first node would be on the stack - non-identical
 */

pub struct List {
    head: Link
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

#[allow(dead_code)]
impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty) // Temporary replacement
        });
        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32>{
        match mem::replace(&mut self.head, Link::Empty) { // Temporarily breaking the chain
            Link::Empty => {
                None
            },
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            },
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link { // boxed_node will get out of scope and be deallocated
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty); // Breaking whole chain
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
    }
}
