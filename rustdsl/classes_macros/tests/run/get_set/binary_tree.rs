use classes_macros::classes;

classes! {
    class Node {
        #[derive(Default)]
            struct {
                value: usize,
                left: Option<CRc<Node>>,
                right: Option<CRc<Node>>,
                parent: Option<CWeak<Node>>,
            }

            pub fn new(value: usize) -> Self {
                Self { value, left: None, right: None, parent: None }
            }
        }

        class Tree {
            #[derive(Default)]
            struct {
                root: Option<CRc<Node>>,
            }

            pub fn new() -> Self {
                Self { root: None }
            }

            pub fn insert(&self, value: usize) {
                let node = Node::new(value);
                if let Some(root) = self.get_root() {
                    self.insert_node(root, node);
                } else {
                    self.set_root(node);
                }
            }

            pub fn insert_node(&self, current: CRc<Node>, new: CRc<Node>) {
                if new.get_value() < current.get_value() {
                    if let Some(left) = current.get_left() {
                        self.insert_node(left, new);
                    } else {
                        current.set_left(new.clone());
                        new.set_parent(current);
                    }
                } else {
                    if let Some(right) = current.get_right() {
                        self.insert_node(right, new);
                    } else {
                        current.set_right(new.clone());
                        new.set_parent(current);
                    }
                }
            }

            pub fn pre_order_traverse(&self) -> Vec<usize> {
                let mut result = Vec::new();
                if let Some(root) = self.get_root() {
                    self.pre_order_traverse_node(root, &mut result);
                }
                result
            }

            fn pre_order_traverse_node(&self, node: CRc<Node>, result: &mut Vec<usize>) {
                result.push(node.get_value());
                if let Some(left) = node.get_left() {
                    self.pre_order_traverse_node(left, result);
                }
                if let Some(right) = node.get_right() {
                    self.pre_order_traverse_node(right, result);
                }
            }

            pub fn in_order_traverse(&self) -> Vec<usize> {
                let mut result = Vec::new();
                if let Some(root) = self.get_root() {
                    self.in_order_traverse_node(root, &mut result);
                }
                result
            }

            fn in_order_traverse_node(&self, node: CRc<Node>, result: &mut Vec<usize>) {
                if let Some(left) = node.get_left() {
                    self.in_order_traverse_node(left, result);
                }
                result.push(node.get_value());
                if let Some(right) = node.get_right() {
                    self.in_order_traverse_node(right, result);
                }
            }

            pub fn post_order_traverse(&self) -> Vec<usize> {
                let mut result = Vec::new();
                if let Some(root) = self.get_root() {
                    self.post_order_traverse_node(root, &mut result);
                }
                result
            }

            fn post_order_traverse_node(&self, node: CRc<Node>, result: &mut Vec<usize>) {
                if let Some(left) = node.get_left() {
                    self.post_order_traverse_node(left, result);
                }
                if let Some(right) = node.get_right() {
                    self.post_order_traverse_node(right, result);
                }
                result.push(node.get_value());
            }
    }
}

#[test]
fn binary_tree() {
    let tree = Tree::new();
    let values = [5, 2, 8, 4, 7, 1, 3, 6, 9, 10];
    for value in values {
        tree.insert(value);
    }

    let pre_order = tree.pre_order_traverse();
    assert_eq!(pre_order, [5, 2, 1, 4, 3, 8, 7, 6, 9, 10]);

    let in_order = tree.in_order_traverse();
    assert_eq!(in_order, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    let post_order = tree.post_order_traverse();
    assert_eq!(post_order, [1, 3, 4, 2, 6, 7, 10, 9, 8, 5]);
}
