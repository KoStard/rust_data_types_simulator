#[allow(dead_code)]
pub struct Stack {
    mem: Vec<i64>,
    stack_size: Option<usize>
}

#[allow(dead_code)]
impl Stack {
    pub fn new() -> Stack {
        Stack {
            mem: vec![],
            stack_size: None
        }
    }
    pub fn new_sized(stack_size: usize) -> Stack {
        Stack {
            mem: vec![],
            stack_size: Some(stack_size)
        }
    }
    pub fn is_empty(&self) -> bool {
        self.mem.len() == 0
    }
    pub fn has_empty_space(&self) -> bool {
        match self.stack_size {
            Some(s) => self.mem.len() < s,
            None => true
        }
    }
    pub fn push(&mut self, new_item: i64) -> &mut Stack {
        if !self.has_empty_space() {
            panic!("Trying to push with overflow.");
        } else {
            self.mem.push(new_item);
        }
        self
    }
    pub fn pop(&mut self) -> i64 {
        self.mem.pop().unwrap()
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Stack with {} elements.", self.mem.len())
    }
}

impl std::fmt::Debug for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let max_length = self.mem.iter().max().unwrap().to_string().len();
        write!(f, "^{:^width$}^\n", "", width=max_length + 2)?;
        for i in self.mem.iter().rev() {
            write!(f, "|{:^width$}|\n", i, width=max_length + 2)?;
        }
        write!(f, "{}", (0..max_length + 4).map(|_|'-').collect::<String>())
    }
}


#[allow(dead_code)]
pub fn demo() {
    let mut s = Stack::new();
    s.push(100);
    s.push(200);
    s.push(300);
    s.push(400);
    s.pop();
    println!("{}", s);
    println!("{:?}", s);

    let mut sized_stack = Stack::new_sized(3); // Stack size is 3
    sized_stack.push(1);
    sized_stack.push(2);
    sized_stack.push(3);
    println!("{}", sized_stack);
    sized_stack.push(4); // <<-- Will panic - FIXME
/*
^     ^
| 300 |
| 200 |
| 100 |
-------
 */
}
