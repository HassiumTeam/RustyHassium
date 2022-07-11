mod ast;

use crate::lexer::{Token, TokenType};

pub use self::ast::{AstNode, BinOpType, FuncParams, UnaryOpType};

struct ParserContext {
    tokens: Vec<Token>,
    pos: u32,
}

impl ParserContext {
    fn current(&self) -> Option<Token> {
        self.tokens.get(self.pos as usize).cloned()
    }

    fn match_tok(&self, token_type: TokenType, value: Option<&str>) -> bool {
        if let Some(current) = self.current() {
            if current.token_type != token_type {
                return false;
            };

            value.map(|value| value == current.value).unwrap_or(true)
        } else {
            false
        }
    }

    fn accept_tok(&mut self, token_type: TokenType, value: Option<&str>) -> bool {
        let ret = self.match_tok(token_type, value);
        if ret {
            self.pos += 1;
        }
        return ret;
    }

    fn expect_tok(&mut self, token_type: TokenType, value: Option<&str>) -> Token {
        let current_option = self.current();
        if current_option.is_none() {
            panic!("Unexpected EOF");
        }
        let current = current_option.unwrap();
        if value.is_none() {
            if current.token_type == token_type {
                self.pos += 1;
                current
            } else {
                panic!(
                    "Unexpected {} '{}', expected {}!",
                    current.token_type, current.value, token_type
                );
            }
        } else {
            let _value = value.unwrap();
            if current.token_type == token_type && current.value == _value {
                self.pos += 1;
                current
            } else {
                panic!(
                    "Unexpected {} '{}', expected {} '{}'!",
                    current.token_type, current.value, token_type, _value
                )
            }
        }
    }
}

pub fn parse(tokens: &mut Vec<Token>) -> AstNode {
    let mut context: ParserContext = ParserContext {
        tokens: tokens.to_vec(),
        pos: 0,
    };
    let mut children: Vec<AstNode> = Vec::new();
    while !context.match_tok(TokenType::EOF, None) {
        children.push(parse_statement(&mut context));
    }

    AstNode::Block {
        children: Box::new(children),
    }
}

fn parse_statement(context: &mut ParserContext) -> AstNode {
    let statement: AstNode = if context.match_tok(TokenType::OpenBrace, None) {
        parse_block(context)
    } else if context.match_tok(TokenType::Id, Some("break")) {
        parse_break(context)
    } else if context.match_tok(TokenType::Id, Some("continue")) {
        parse_continue(context)
    } else if context.match_tok(TokenType::Id, Some("class")) {
        parse_class(context)
    } else if context.match_tok(TokenType::Id, Some("continue")) {
        parse_continue(context)
    } else if context.match_tok(TokenType::Id, Some("for")) {
        parse_for(context)
    } else if context.match_tok(TokenType::Id, Some("foreach")) {
        parse_foreach(context)
    } else if context.match_tok(TokenType::Id, Some("func")) {
        parse_func(context)
    } else if context.accept_tok(TokenType::Semicolon, None) {
        AstNode::Empty
    } else if context.match_tok(TokenType::Id, Some("if")) {
        parse_if(context)
    } else if context.match_tok(TokenType::Id, Some("import")) {
        parse_import(context)
    } else if context.match_tok(TokenType::Id, Some("raise")) {
        parse_raise(context)
    } else if context.match_tok(TokenType::Id, Some("return")) {
        parse_return(context)
    } else if context.match_tok(TokenType::Id, Some("super")) {
        parse_super(context)
    } else if context.match_tok(TokenType::Id, Some("try")) {
        parse_try_catch(context)
    } else if context.match_tok(TokenType::Id, Some("while")) {
        parse_while(context)
    } else {
        parse_expression_statement(context)
    };

    context.accept_tok(TokenType::Semicolon, None);

    return statement;
}

fn parse_block(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::OpenBrace, None);
    let mut children: Vec<AstNode> = Vec::new();
    while !context.accept_tok(TokenType::CloseBrace, None) {
        children.push(parse_statement(context));
    }
    AstNode::Block {
        children: Box::new(children),
    }
}

fn parse_break(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("break"));
    AstNode::Break
}

fn parse_class(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("class"));
    let name: String = context.expect_tok(TokenType::Id, None).value.clone();
    let extends: Option<AstNode> = if context.accept_tok(TokenType::Id, Some("extends")) {
        Some(parse_expression(context))
    } else {
        None
    };
    let body = parse_statement(context);

    AstNode::Class {
        name,
        extends: Box::new(extends),
        body: Box::new(body),
    }
}

fn parse_continue(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("continue"));
    AstNode::Continue
}

fn parse_for(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("for"));
    let using_parens = context.accept_tok(TokenType::OpenParen, None);
    let initial = parse_expression(context);
    context.accept_tok(TokenType::Semicolon, None);
    let condition: AstNode = parse_expression(context);
    context.accept_tok(TokenType::Semicolon, None);
    let repeated: AstNode = parse_expression(context);
    if using_parens {
        context.expect_tok(TokenType::CloseParen, None);
    }
    let body: AstNode = parse_statement(context);

    AstNode::For {
        initial: Box::new(initial),
        condition: Box::new(condition),
        repeated: Box::new(repeated),
        body: Box::new(body),
    }
}

fn parse_foreach(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("foreach"));
    let using_parens: bool = context.accept_tok(TokenType::OpenParen, None);
    let var: String = context.expect_tok(TokenType::Id, None).value.clone();
    context.expect_tok(TokenType::Id, Some("in"));
    let target: AstNode = parse_expression(context);
    if using_parens {
        context.expect_tok(TokenType::CloseParen, None);
    }
    let body: AstNode = parse_statement(context);

    AstNode::Foreach {
        var,
        target: Box::new(target),
        body: Box::new(body),
    }
}

fn parse_func(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("func"));
    let name: String = context.expect_tok(TokenType::Id, None).value.clone();
    let params: FuncParams = parse_func_params(context);
    let return_type: Option<AstNode> = if context.accept_tok(TokenType::Colon, None) {
        Some(parse_expression(context))
    } else {
        None
    };
    let body: AstNode = parse_statement(context);

    AstNode::Func {
        name,
        params,
        return_type: Box::new(return_type),
        body: Box::new(body),
    }
}

fn parse_func_params(context: &mut ParserContext) -> FuncParams {
    context.expect_tok(TokenType::OpenParen, None);
    let mut names: Vec<String> = Vec::new();
    let mut types: Vec<Option<AstNode>> = Vec::new();
    let mut variadic: bool = false;
    while !context.accept_tok(TokenType::CloseParen, None) {
        if context.accept_tok(TokenType::Variadic, None) {
            variadic = true;
            context.expect_tok(TokenType::CloseParen, None);
            break;
        }
        names.push(context.expect_tok(TokenType::Id, None).value.clone());
        types.push(if context.accept_tok(TokenType::Colon, None) {
            Some(parse_expression(context))
        } else {
            None
        });
        // If the next token is not a comma or a cparen, fail by expecting the comma
        if !context.match_tok(TokenType::Comma, None)
            && !context.match_tok(TokenType::CloseParen, None)
        {
            context.expect_tok(TokenType::Comma, None);
        }
    }

    FuncParams {
        names,
        types,
        variadic,
    }
}

fn parse_if(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("if"));
    let using_parens: bool = context.accept_tok(TokenType::OpenParen, None);
    let predicate = parse_expression(context);
    if using_parens {
        context.expect_tok(TokenType::CloseParen, None);
    }
    let body = parse_statement(context);
    let else_body: Option<AstNode> = if context.accept_tok(TokenType::Id, Some("else")) {
        Some(parse_statement(context))
    } else {
        None
    };

    AstNode::If {
        predicate: Box::new(predicate),
        body: Box::new(body),
        else_body: Box::new(else_body),
    }
}

fn parse_import(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("import"));
    let target = parse_expression(context);
    AstNode::Import {
        target: Box::new(target),
    }
}

fn parse_raise(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("raise"));
    let value = parse_expression(context);
    AstNode::Raise {
        value: Box::new(value),
    }
}

fn parse_return(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("return"));
    let value: AstNode = parse_expression(context);
    AstNode::Return {
        value: Box::new(value),
    }
}

fn parse_super(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("super"));
    context.expect_tok(TokenType::OpenParen, None);
    let mut args: Vec<AstNode> = Vec::new();
    while !context.accept_tok(TokenType::CloseParen, None) {
        args.push(parse_expression(context));
        if !context.match_tok(TokenType::CloseParen, None) {
            context.expect_tok(TokenType::Comma, None);
        }
    }

    AstNode::Super {
        args: Box::new(args),
    }
}

fn parse_try_catch(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("try"));
    let try_body: AstNode = parse_statement(context);
    context.expect_tok(TokenType::Id, Some("catch"));
    let value: Option<String> = if context.accept_tok(TokenType::OpenParen, None) {
        Some(context.expect_tok(TokenType::Id, None).value.clone())
    } else {
        None
    };
    let catch_body: AstNode = parse_statement(context);

    AstNode::TryCatch {
        try_body: Box::new(try_body),
        value,
        catch_body: Box::new(catch_body),
    }
}

fn parse_while(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("while"));
    let using_parens: bool = context.accept_tok(TokenType::OpenParen, None);
    let condition: AstNode = parse_expression(context);
    if using_parens {
        context.expect_tok(TokenType::CloseParen, None);
    }
    let body: AstNode = parse_statement(context);

    AstNode::While {
        condition: Box::new(condition),
        body: Box::new(body),
    }
}

fn parse_expression_statement(context: &mut ParserContext) -> AstNode {
    let expression = parse_expression(context);
    context.accept_tok(TokenType::Semicolon, None);
    AstNode::ExpressionStatement {
        expression: Box::new(expression),
    }
}

fn parse_expression(context: &mut ParserContext) -> AstNode {
    parse_assign(context)
}

fn parse_assign(context: &mut ParserContext) -> AstNode {
    let left: AstNode = parse_or(context);
    if context.match_tok(TokenType::Assign, None) {
        let op_str: String = context.expect_tok(TokenType::Assign, None).value.clone();
        if op_str.eq("=") {
            return AstNode::Assign {
                left: Box::new(left),
                right: Box::new(parse_assign(context)),
            };
        } else {
            let bin_op_type = match op_str.as_str() {
                "+=" => BinOpType::Add,
                "-=" => BinOpType::Subtract,
                "*=" => BinOpType::Multiply,
                "%=" => BinOpType::Modulus,
                "&=" => BinOpType::BitwiseAnd,
                "|=" => BinOpType::BitwiseOr,
                "^=" => BinOpType::Xor,
                _ => panic!("Unknown assignment op {}", op_str),
            };
            let left_clone: AstNode = left.clone();
            return AstNode::Assign {
                left: Box::new(left),
                right: Box::new(AstNode::BinOp {
                    op: bin_op_type,
                    left: Box::new(left_clone),
                    right: Box::new(parse_assign(context)),
                }),
            };
        }
    } else {
        return left;
    }
}

fn parse_or(context: &mut ParserContext) -> AstNode {
    let left: AstNode = parse_and(context);
    return if context.accept_tok(TokenType::Op, Some("||")) {
        AstNode::BinOp {
            op: BinOpType::Or,
            left: Box::new(left),
            right: Box::new(parse_or(context)),
        }
    } else {
        left
    };
}

fn parse_and(context: &mut ParserContext) -> AstNode {
    let left: AstNode = parse_bitwise_or(context);
    return if context.accept_tok(TokenType::Op, Some("&&")) {
        AstNode::BinOp {
            op: BinOpType::And,
            left: Box::new(left),
            right: Box::new(parse_or(context)),
        }
    } else {
        left
    };
}

fn parse_bitwise_or(context: &mut ParserContext) -> AstNode {
    let left: AstNode = parse_xor(context);
    return if context.accept_tok(TokenType::Op, Some("|")) {
        AstNode::BinOp {
            op: BinOpType::BitwiseOr,
            left: Box::new(left),
            right: Box::new(parse_or(context)),
        }
    } else {
        left
    };
}

fn parse_xor(context: &mut ParserContext) -> AstNode {
    let left: AstNode = parse_bitwise_and(context);
    return if context.accept_tok(TokenType::Op, Some("^")) {
        AstNode::BinOp {
            op: BinOpType::Xor,
            left: Box::new(left),
            right: Box::new(parse_or(context)),
        }
    } else {
        left
    };
}

fn parse_bitwise_and(context: &mut ParserContext) -> AstNode {
    let left: AstNode = parse_equality(context);
    return if context.accept_tok(TokenType::Op, Some("&")) {
        AstNode::BinOp {
            op: BinOpType::BitwiseAnd,
            left: Box::new(left),
            right: Box::new(parse_or(context)),
        }
    } else {
        left
    };
}

fn parse_equality(context: &mut ParserContext) -> AstNode {
    let left: AstNode = parse_comparison(context);
    return if context.accept_tok(TokenType::Op, Some("==")) {
        AstNode::BinOp {
            op: BinOpType::EqualTo,
            left: Box::new(left),
            right: Box::new(parse_comparison(context)),
        }
    } else if context.accept_tok(TokenType::Op, Some("!=")) {
        AstNode::UnaryOp {
            op: UnaryOpType::Not,
            target: Box::new(AstNode::BinOp {
                op: BinOpType::EqualTo,
                left: Box::new(left),
                right: Box::new(parse_equality(context)),
            }),
        }
    } else {
        left
    };
}

fn parse_comparison(context: &mut ParserContext) -> AstNode {
    let left: AstNode = parse_additive(context);
    return if context.accept_tok(TokenType::Op, Some(">")) {
        AstNode::BinOp {
            op: BinOpType::GreaterThan,
            left: Box::new(left),
            right: Box::new(parse_comparison(context)),
        }
    } else if context.accept_tok(TokenType::Op, Some(">=")) {
        AstNode::BinOp {
            op: BinOpType::GreaterThanOrEqual,
            left: Box::new(left),
            right: Box::new(parse_comparison(context)),
        }
    } else if context.accept_tok(TokenType::Op, Some("<")) {
        AstNode::BinOp {
            op: BinOpType::LesserThan,
            left: Box::new(left),
            right: Box::new(parse_comparison(context)),
        }
    } else if context.accept_tok(TokenType::Op, Some("<=")) {
        AstNode::BinOp {
            op: BinOpType::LesserThanOrEqual,
            left: Box::new(left),
            right: Box::new(parse_comparison(context)),
        }
    } else {
        left
    };
}

fn parse_additive(context: &mut ParserContext) -> AstNode {
    let left: AstNode = parse_multiplicative(context);
    return if context.accept_tok(TokenType::Op, Some("+")) {
        AstNode::BinOp {
            op: BinOpType::Add,
            left: Box::new(left),
            right: Box::new(parse_additive(context)),
        }
    } else if context.accept_tok(TokenType::Op, Some("-")) {
        AstNode::BinOp {
            op: BinOpType::Subtract,
            left: Box::new(left),
            right: Box::new(parse_additive(context)),
        }
    } else {
        left
    };
}

fn parse_multiplicative(context: &mut ParserContext) -> AstNode {
    let left: AstNode = parse_unary(context);
    return if context.accept_tok(TokenType::Op, Some("/")) {
        AstNode::BinOp {
            op: BinOpType::Divide,
            left: Box::new(left),
            right: Box::new(parse_additive(context)),
        }
    } else if context.accept_tok(TokenType::Op, Some("%")) {
        AstNode::BinOp {
            op: BinOpType::Multiply,
            left: Box::new(left),
            right: Box::new(parse_multiplicative(context)),
        }
    } else if context.accept_tok(TokenType::Op, Some("*")) {
        AstNode::BinOp {
            op: BinOpType::Multiply,
            left: Box::new(left),
            right: Box::new(parse_multiplicative(context)),
        }
    } else {
        left
    };
}

fn parse_unary(context: &mut ParserContext) -> AstNode {
    return if context.accept_tok(TokenType::Op, Some("!")) {
        AstNode::UnaryOp {
            op: UnaryOpType::Not,
            target: Box::new(parse_unary(context)),
        }
    } else if context.accept_tok(TokenType::Op, Some("--")) {
        AstNode::UnaryOp {
            op: UnaryOpType::DecrementPre,
            target: Box::new(parse_unary(context)),
        }
    } else if context.accept_tok(TokenType::Op, Some("++")) {
        AstNode::UnaryOp {
            op: UnaryOpType::IncrementPre,
            target: Box::new(parse_unary(context)),
        }
    } else {
        parse_access(context, None)
    };
}

fn parse_access(context: &mut ParserContext, left: Option<AstNode>) -> AstNode {
    let _left = left.unwrap_or(parse_term(context));
    return if context.accept_tok(TokenType::OpenParen, None) {
        let mut args: Vec<AstNode> = Vec::new();
        while !context.accept_tok(TokenType::CloseParen, None) {
            args.push(parse_expression(context))
        }
        AstNode::Invoke {
            target: Box::new(_left),
            args: Box::new(args),
        }
    } else if context.accept_tok(TokenType::OpenSquare, None) {
        let key: AstNode = parse_expression(context);
        context.expect_tok(TokenType::CloseSquare, None);
        AstNode::Subscript {
            target: Box::new(_left),
            key: Box::new(key),
        }
    } else if context.accept_tok(TokenType::Dot, None) {
        let attrib = context.expect_tok(TokenType::Id, None).value.clone();
        parse_access(
            context,
            Some(AstNode::AttribAccess {
                target: Box::new(_left),
                attrib,
            }),
        )
    } else {
        _left
    };
}

fn parse_term(context: &mut ParserContext) -> AstNode {
    return if context.match_tok(TokenType::Id, None) {
        AstNode::Id {
            value: context.expect_tok(TokenType::Id, None).value.clone(),
        }
    } else if context.match_tok(TokenType::Number, None) {
        AstNode::Number {
            value: context
                .expect_tok(TokenType::Number, None)
                .value
                .parse()
                .unwrap(),
        }
    } else if context.match_tok(TokenType::String, None) {
        AstNode::String {
            value: context.expect_tok(TokenType::String, None).value.clone(),
        }
    } else if context.accept_tok(TokenType::OpenParen, None) {
        let expression = parse_expression(context);
        context.expect_tok(TokenType::CloseParen, None);
        expression
    } else {
        panic!(
            "Unexpected {} {}",
            context.current().unwrap().token_type,
            context.current().unwrap().value
        )
    };
}
