pub struct AstNode {
    pub name: String,
    pub children: Vec<AstNode>,
    pub strings: Option<Vec<String>>,
    pub ints: Option<Vec<u32>>,
    pub params: Option<FuncParams>,
}

pub struct FuncParams {
    pub names: Vec<String>,
    pub variadic: bool,
    pub types: Box<Vec<Option<AstNode>>>,
    pub return_type: Box<Option<AstNode>>,
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

// pub struct FuncParams {
//     pub ids: Vec<String>,
//     pub types: Vec<Expression>,
//     pub is_variadic: bool,
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
