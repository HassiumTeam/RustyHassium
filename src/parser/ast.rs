pub enum AstNode {
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
}

pub struct FuncParams {
    pub names: Vec<String>,
    pub variadic: bool,
    pub types: Box<Vec<Option<AstNode>>>,
}
