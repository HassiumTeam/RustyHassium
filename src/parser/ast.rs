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

// pub struct Statement {
//     pub block: Option<Block>,
//     pub _break: Option<Break>,
//     pub _class: Option<Class>,
//     pub _continue: Option<Continue>,
//     pub empty: Option<Empty>,
//     pub _for: Option<For>,
//     pub foreach: Option<Foreach>,
//     pub func: Option<Func>,
//     pub _if: Option<If>,
//     pub import: Option<Import>,
//     pub raise: Option<Raise>,
//     pub _return: Option<Return>,
//     pub _super: Option<Super>,
//     pub try_catch: Option<TryCatch>,
//     pub _while: Option<While>,
//     pub expression_statement: Option<ExpressionStatement>,
//     pub assign: Option<Assign>,
// }

// pub struct Block {
//     pub statements: Vec<Statement>,
// }

// pub struct Break {}

// pub struct Class {
//     pub name: String,
//     pub extends: Option<Expression>,
//     pub body: Box<Statement>,
// }

// pub struct Continue {}

// pub struct Empty {}

// pub struct For {
//     pub initial: Box<Statement>,
//     pub expression: Expression,
//     pub repeated: Box<Statement>,
//     pub body: Box<Statement>,
// }

// pub struct Foreach {
//     pub var: String,
//     pub target: Expression,
//     pub body: Box<Statement>,
// }

// pub struct Func {
//     pub name: String,
//     pub params: FuncParams,
//     pub return_type: Expression,
//     pub body: Box<Statement>,
// }

// pub struct FuncArgs {
//     pub args: Vec<Expression>,
// }

// pub struct If {
//     pub predicate: Expression,
//     pub body: Box<Statement>,
// }

// pub struct Import {
//     pub module: Expression,
//     pub targets: Option<Vec<String>>,
// }

// pub struct Raise {
//     pub exception: Expression,
// }

// pub struct Return {
//     pub value: Expression,
// }

// pub struct Super {
//     pub args: FuncArgs,
// }

// pub struct TryCatch {
//     pub try_body: Box<Statement>,
//     pub catch_body: Box<Statement>,
// }

// pub struct While {
//     pub condition: Box<Statement>,
//     pub body: Box<Statement>,
// }

// pub struct ExpressionStatement {
//     expression: Expression,
// }

// pub struct Assign {
//     pub left_hand_side: Box<Expression>,
//     pub right_hand_side: Box<Expression>,
// }

// pub struct Expression {
//     pub assign: Option<Assign>,
// }
