pub struct BinaryTree<T> where T: Clone{
    pub value: T,
    pub left: Option<Box<BinaryTree<T>>>,
    pub right: Option<Box<BinaryTree<T>>>
}

impl<T> BinaryTree<T> where T: Clone{
    pub fn new(val: T) -> BinaryTree<T> {
        BinaryTree {
            value: val,
            left: None,
            right: None,
        }
    }
    pub fn set_left(&mut self, val: T) {
        self.left = Some(Box::new(BinaryTree {
            value: val,
            left: None,
            right: None
        }));
    }
    pub fn set_right(&mut self, val: T) {
        self.right = Some(Box::new(BinaryTree {
            value: val,
            left: None,
            right: None
        }));
    }
}
