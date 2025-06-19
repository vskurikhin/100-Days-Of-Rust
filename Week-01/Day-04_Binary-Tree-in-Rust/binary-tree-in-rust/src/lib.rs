#[derive(Debug)]
pub struct Node<T: Ord> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, val: T) {
        match self.val.cmp(&val) {
            std::cmp::Ordering::Greater => {
                if let Some(ref mut node) = self.left {
                    node.insert(val);
                } else {
                    self.left = Some(Box::new(Node::new(val)));
                }
            }
            std::cmp::Ordering::Equal => todo!(),
            std::cmp::Ordering::Less => {
                if let Some(ref mut node) = self.right {
                    node.insert(val);
                } else {
                    self.right = Some(Box::new(Node::new(val)));
                }
            }
        }
    }
}