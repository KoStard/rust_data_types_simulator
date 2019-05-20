enum Node {
    Body(i64, Box<Node>),
    Head,
}
use Node::*;

#[allow(dead_code)]
impl Node {
    pub fn next(&self) -> Option<&Node> {
        match &self {
            Body(_, next) => Some(next),
            Head => None,
        }
    }
}

// As LIFO - last-in-first-out
pub struct SinglyLinkedList {
    tail: Node, // last added node
}

#[allow(dead_code)]
impl SinglyLinkedList {
    pub fn new() -> SinglyLinkedList {
        SinglyLinkedList { tail: Node::Head }
    }
    // Getting the ownership, because I can't figure out any other solution for this yet :/
    pub fn push(mut self, value: i64) -> SinglyLinkedList {
        let old_tail = self.tail;
        self.tail = Node::Body(value, Box::new(old_tail));
        self
    }
    pub fn pop(mut self) -> Option<(SinglyLinkedList, i64)> {
        match self.tail {
            Body(value, next) => {
                self.tail = *next;
                Some((self, value))
            }
            Head => {
                panic!("Trying to remove element from empty linked list");
            }
        }
    }
    pub fn last_value(&self) -> Option<i64> {
        match &self.tail {
            Body(value, _) => Some(*value),
            Head => None,
        }
    }
    pub fn is_empty(&self) -> bool {
        match self.tail {
            Body(_, _) => false,
            Head => true,
        }
    }
    pub fn len(&self) -> usize {
        let mut res = 0;
        let mut current_node = &self.tail;
        loop {
            match current_node {
                Body(_, next) => {
                    res += 1;
                    current_node = next;
                }
                Head => {
                    return res;
                }
            }
        }
    }
}

impl std::fmt::Display for SinglyLinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut current_node = &self.tail;
        let mut lines = [String::from(""), String::from(""), String::from("")];
        loop {
            match current_node {
                Body(value, next) => {
                    let value_len = value.to_string().len();
                    lines[0].push_str(&format!(
                        "{}    ",
                        (0..value_len + 4).map(|_| '-').collect::<String>()
                    ));
                    lines[1].push_str(&format!("| {} | => ", value));
                    lines[2].push_str(&format!(
                        "{}    ",
                        (0..value_len + 4).map(|_| '-').collect::<String>()
                    ));
                    current_node = next;
                }
                Head => {
                    write!(f, "{}", lines[0])?;
                    write!(f, "-----\n")?;
                    write!(f, "{}", lines[1])?;
                    write!(f, "| x |\n")?;
                    write!(f, "{}", lines[2])?;
                    return write!(f, "-----\n");
                }
            }
        }
    }
}

pub fn demo() {
    let list = SinglyLinkedList::new();
    let list = list.push(10).push(11).push(12).push(13).push(14);
    println!("{}", list);
    /*
    ------    ------    ------    ------    ------    -----
    | 14 | => | 13 | => | 12 | => | 11 | => | 10 | => | x |
    ------    ------    ------    ------    ------    -----
     */
}
