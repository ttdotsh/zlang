pub struct Program(Vec<Node>);

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Expr(Expr),
    Stmt(Stmt),
}

impl From<Expr> for Node {
    fn from(value: Expr) -> Self {
        Self::Expr(value)
    }
}

impl From<Stmt> for Node {
    fn from(value: Stmt) -> Self {
        Self::Stmt(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Bool(bool),
    Int(i32),
    Str(String),
    // If {
    //     condition: Box<Expr>,
    //     then: Block,
    //     alt: Option<Block>,
    // },
    // Prefix {
    //     op: Operator,
    // },
    // Infix,
    // Postfix,
    // Call,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    Ngl(String, Expr),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Block(Vec<Node>);
