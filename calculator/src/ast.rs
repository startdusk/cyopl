#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Int(i32),
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}
