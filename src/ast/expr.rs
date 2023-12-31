use std::rc::Rc;

#[derive(Debug)]
pub enum BinaryOperator {
    LogicAnd,
    LogicOr,
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
}

#[derive(Debug)]
pub enum UnaryOperator {
    LogicNot,
}

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    Unary {
        operator: UnaryOperator,
        right: Box<Expr>,
    },
    FunctionCall {
        function: Box<Expr>,
        args: Vec<Expr>,
    },
    ArrayIndex {
        array: Box<Expr>,
        indexes: Vec<Expr>,
    },
    Identifier {
        handle: usize,
    },
    Literal(Literal),
}

#[derive(Debug)]
pub enum Literal {
    Char(char),
    String(Rc<str>),
    Integer(i64),
    Real(f64),
    Boolean(bool),
}
