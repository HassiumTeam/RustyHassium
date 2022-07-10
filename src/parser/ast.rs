#[derive(Clone)]
pub enum AstNode {
    // Statement nodes
    Block {
        children: Box<Vec<AstNode>>,
    },
    Break,
    Class {
        name: String,
        extends: Box<Option<AstNode>>,
        body: Box<AstNode>,
    },
    Continue,
    Empty,
    For {
        initial: Box<AstNode>,
        condition: Box<AstNode>,
        repeated: Box<AstNode>,
        body: Box<AstNode>,
    },
    Foreach {
        var: String,
        target: Box<AstNode>,
        body: Box<AstNode>,
    },
    Func {
        name: String,
        params: FuncParams,
        return_type: Box<Option<AstNode>>,
        body: Box<AstNode>,
    },
    If {
        predicate: Box<AstNode>,
        body: Box<AstNode>,
        else_body: Box<Option<AstNode>>,
    },
    Import {
        target: Box<AstNode>,
    },
    Raise {
        value: Box<AstNode>,
    },
    Return {
        value: Box<AstNode>,
    },
    Super {
        args: Box<Vec<AstNode>>,
    },
    TryCatch {
        try_body: Box<AstNode>,
        value: Option<String>,
        catch_body: Box<AstNode>,
    },
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    ExpressionStatement {
        expression: Box<AstNode>,
    },
    // Expression nodes
    Assign {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    AttribAccess {
        target: Box<AstNode>,
        attrib: String,
    },
    BinOp {
        op: BinOpType,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    Id {
        value: String,
    },
    Invoke {
        target: Box<AstNode>,
        args: Box<Vec<AstNode>>,
    },
    Number {
        value: f64,
    },
    String {
        value: String,
    },
    Subscript {
        target: Box<AstNode>,
        key: Box<AstNode>,
    },
    UnaryOp {
        op: UnaryOpType,
        target: Box<AstNode>,
    },
}

#[derive(Clone)]
pub enum BinOpType {
    Add,
    And,
    BitwiseAnd,
    BitwiseOr,
    Divide,
    EqualTo,
    GreaterThan,
    GreaterThanOrEqual,
    LesserThan,
    LesserThanOrEqual,
    Or,
    Modulus,
    Multiply,
    Subtract,
    Xor,
}

#[derive(Clone)]
pub enum UnaryOpType {
    DecrementPre,
    Not,
    IncrementPre,
}

#[derive(Clone)]
pub struct FuncParams {
    pub names: Vec<String>,
    pub variadic: bool,
    pub types: Vec<Option<AstNode>>,
}
