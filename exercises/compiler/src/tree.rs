pub mod visitor;

pub struct Tree {
    root: Expression,
}

impl Tree {
    /// Recursively evaluate the tree.
    ///
    /// This is the OOP-approach to the task, as per 2.b)
    pub fn eval(&self) -> i64 {
        self.root.eval()
    }
}

/// Types of expressions in our tree.
///
/// As we only support binary operations, we differentiate between literal expressions (leaves in
/// the tree), and arithmetic operations (branches in the tree).
#[derive(Debug)]
pub enum Expression {
    IntLiteral(i64),
    Addition {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Multiplication {
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl Expression {
    /// Evaluate the value of the expression.
    pub fn eval(&self) -> i64 {
        match self {
            Expression::IntLiteral(i) => *i,
            Expression::Addition { left, right } => left.eval() + right.eval(),
            Expression::Multiplication { left, right } => left.eval() * right.eval(),
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
fn eval_recursive(expr: &Expression) -> i64 {
    match expr {
        Expression::IntLiteral(i) => *i,
        Expression::Addition { left, right } => eval_recursive(left) + eval_recursive(right),
        Expression::Multiplication { left, right } => eval_recursive(left) * eval_recursive(right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> Tree {
        Tree {
            root: Expression::Addition {
                left: Box::new(Expression::Addition {
                    left: Box::new(Expression::IntLiteral(7)),
                    right: Box::new(Expression::Addition {
                        left: Box::new(Expression::IntLiteral(11)),
                        right: Box::new(Expression::IntLiteral(12)),
                    }),
                }),
                right: Box::new(Expression::Multiplication {
                    left: Box::new(Expression::IntLiteral(2)),
                    right: Box::new(Expression::Multiplication {
                        left: Box::new(Expression::IntLiteral(3)),
                        right: Box::new(Expression::IntLiteral(5)),
                    }),
                }),
            },
        }
    }

    #[test]
    fn test_procedural() {
        let int_literal = Expression::IntLiteral(42);
        assert_eq!(eval_recursive(&int_literal), 42);

        let mult = Expression::Multiplication {
            left: Box::new(Expression::IntLiteral(2)),
            right: Box::new(Expression::IntLiteral(10)),
        };
        assert_eq!(eval_recursive(&mult), 20);

        let tree = sample_tree();
        assert_eq!(eval(&tree), 60);
    }

    #[test]
    fn test_oop() {
        let int_literal = Expression::IntLiteral(42);
        assert_eq!(int_literal.eval(), 42);

        let mult = Expression::Multiplication {
            left: Box::new(Expression::IntLiteral(2)),
            right: Box::new(Expression::IntLiteral(10)),
        };
        assert_eq!(mult.eval(), 20);

        let tree = sample_tree();
        assert_eq!(tree.eval(), 60);
    }
}
