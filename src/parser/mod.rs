mod ast;

use crate::lexer::{Token, TokenType};

use self::ast::{AstNode, FuncParams};

struct ParserContext {
    tokens: Vec<Token>,
    pos: u32,
    len: usize,
}

impl ParserContext {
    fn eof(&mut self) -> bool {
        self.pos as usize >= self.len
    }

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
        len: tokens.len(),
    };
    let mut children: Vec<AstNode> = Vec::new();
    while !context.eof() {
        children.push(parse_statement(&mut context));
    }

    AstNode::Block {
        children: Box::new(children),
    }
}

fn parse_statement(context: &mut ParserContext) -> AstNode {
    let statement: AstNode = if context.match_tok(TokenType::OpenParen, None) {
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
    return todo!();
}
