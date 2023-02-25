use super::Expression;

pub trait Visitor {
    fn visit_addition(&mut self, branch: &Expression) -> i64;
    fn visit_multiplication(&mut self, branch: &Expression) -> i64;
    fn visit_number(&mut self, leaf: &Expression) -> i64;
}

pub struct EvalVisitor {}

impl Visitor for EvalVisitor {
    fn visit_addition(&mut self, branch: &Expression) -> i64 {
        match branch {
            Expression::Addition { left, right } => 1,
            _ => panic!("visit_addition got non-addition node"),
        }
    }

    fn visit_multiplication(&mut self, branch: &Expression) -> i64 {
        todo!()
    }

    fn visit_number(&mut self, leaf: &Expression) -> i64 {
        match leaf {
            Expression::IntLiteral(i) => *i,
            _ => panic!("visit_number got non-leaf node"),
        }
    }
}
