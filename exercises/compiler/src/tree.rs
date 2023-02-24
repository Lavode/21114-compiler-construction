pub struct Tree {
    root: Node,
}

impl Tree {
    /// Recursively evaluate the tree.
    ///
    /// This is the OOP-approach to the task, as per 2.b)
    pub fn eval(&self) -> i64 {
        self.root.eval()
    }
}

/// Supported arithmetic operations.
pub enum Operation {
    Addition,
    Multiplication,
}

/// Types of nodes in our tree.
///
/// As we only support binary operations, we differentiate between leaf nodes containing a numbe,
/// and branch nodes with two children and an arithmetic operation.
pub enum Node {
    Leaf(i64),
    Branch {
        left: Box<Node>,
        right: Box<Node>,
        operation: Operation,
    },
}

impl Node {
    /// Evaluate the value of the node.
    pub fn eval(&self) -> i64 {
        match self {
            Node::Leaf(i) => return *i,
            Node::Branch {
                left,
                right,
                operation,
            } => match operation {
                Operation::Addition => return left.eval() + right.eval(),
                Operation::Multiplication => return left.eval() * right.eval(),
            },
        }
    }
}

/// Evaluate the arithmetic expression encoded in the tree.
///
/// This is the procedural approach to the task, as per 2.a)
pub fn eval(tree: &Tree) -> i64 {
    eval_recursive(&tree.root)
}

// Recursive evaluation of the tree in a procedural approach.
fn eval_recursive(node: &Node) -> i64 {
    match node {
        Node::Leaf(i) => return *i,
        Node::Branch {
            left,
            right,
            operation,
        } => match operation {
            Operation::Addition => return eval_recursive(left) + eval_recursive(right),
            Operation::Multiplication => return eval_recursive(left) * eval_recursive(right),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> Tree {
        Tree {
            root: Node::Branch {
                operation: Operation::Addition,
                left: Box::new(Node::Leaf(5)),
                right: Box::new(Node::Branch {
                    operation: Operation::Multiplication,
                    left: Box::new(Node::Leaf(2)),
                    right: Box::new(Node::Leaf(10)),
                }),
            },
        }
    }

    #[test]
    fn test_procedural() {
        let tree = sample_tree();

        assert_eq!(eval(&tree), 25);
    }

    #[test]
    fn test_oop() {
        let branch_node = Node::Branch {
            operation: Operation::Multiplication,
            left: Box::new(Node::Leaf(2)),
            right: Box::new(Node::Leaf(10)),
        };
        assert_eq!(branch_node.eval(), 20);

        let leaf_node = Node::Leaf(42);
        assert_eq!(leaf_node.eval(), 42);

        let tree = sample_tree();
        assert_eq!(tree.eval(), 25);
    }
}
