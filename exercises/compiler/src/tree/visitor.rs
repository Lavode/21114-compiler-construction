use crate::stack::Stack;

use super::Expression;

pub trait Visitor {
    fn visit_expression(&mut self, expr: &Expression);
}

pub struct EvalVisitor {
    stack: Stack<i64>,
}

impl EvalVisitor {
    pub fn new() -> EvalVisitor {
        EvalVisitor {
            stack: Stack::new(),
        }
    }

    pub fn result(&mut self) -> i64 {
        self.stack.pop().unwrap()
    }
}

impl Visitor for EvalVisitor {
    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::IntLiteral(i) => self.stack.push(*i),
            Expression::Addition { left: _, right: _ } => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();

                self.stack.push(left + right);
            }
            Expression::Subtraction { left: _, right: _ } => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();

                self.stack.push(left - right);
            }
            Expression::Multiplication { left: _, right: _ } => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();

                self.stack.push(left * right);
            }
        };
    }
}

impl Expression {
    /// Have visitor visit all parts of the expression.
    pub fn accept<T: Visitor>(&self, visitor: &mut T) {
        match self {
            Expression::IntLiteral(_) => {
                visitor.visit_expression(self);
            }
            Expression::Addition { left, right } => {
                left.accept(visitor);
                right.accept(visitor);

                visitor.visit_expression(self);
            }
            Expression::Subtraction { left, right } => {
                left.accept(visitor);
                right.accept(visitor);

                visitor.visit_expression(self);
            }
            Expression::Multiplication { left, right } => {
                left.accept(visitor);
                right.accept(visitor);

                visitor.visit_expression(self);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::Tree;

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
    fn test_visit() {
        let tree = sample_tree();
        let mut visitor = EvalVisitor::new();

        tree.root.accept(&mut visitor);
        assert_eq!(visitor.result(), 60);
    }
}
