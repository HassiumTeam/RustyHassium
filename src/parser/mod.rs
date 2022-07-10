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
                    current.token_type as u32, current.value, token_type as u32
                );
            }
        } else {
            let _value = value.unwrap();
            if current.token_type == token_type && current.value == _value {
                current
            } else {
                panic!(
                    "Unexpected {} '{}', expected {} '{}'!",
                    current.token_type as u32, current.value, token_type as u32, _value
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

    return AstNode {
        name: String::from("program"),
        children,
        strings: None,
        ints: None,
        params: None,
    };
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
    return AstNode {
        name: String::from("block"),
        children,
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_break(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("break"));
    return AstNode {
        name: String::from("break"),
        children: Vec::new(),
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_class(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("class"));
    let mut children: Vec<AstNode> = Vec::new();
    let name: String = context.expect_tok(TokenType::Id, None).value.clone();
    if context.accept_tok(TokenType::Id, Some("extends")) {
        children.push(parse_expression(context));
    }
    children.push(parse_statement(context));
    return AstNode {
        name: String::from("class"),
        children,
        strings: Some(vec![name]),
        ints: None,
        params: None,
    };
}

fn parse_continue(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("continue"));
    return AstNode {
        name: String::from("continue"),
        children: Vec::new(),
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_for(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("for"));
    let using_parens = context.accept_tok(TokenType::OpenParen, None);
    let initial = parse_expression(context);
    context.accept_tok(TokenType::Semicolon, None);
    let expression: AstNode = parse_expression(context);
    context.accept_tok(TokenType::Semicolon, None);
    let repeated: AstNode = parse_expression(context);
    if using_parens {
        context.expect_tok(TokenType::CloseParen, None);
    }
    let body: AstNode = parse_statement(context);

    return AstNode {
        name: String::from("for"),
        children: vec![initial, expression, repeated, body],
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_foreach(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("foreach"));
    let using_parens: bool = context.accept_tok(TokenType::OpenParen, None);
    let id: String = context.expect_tok(TokenType::Id, None).value.clone();
    context.expect_tok(TokenType::Id, Some("in"));
    let expression: AstNode = parse_expression(context);
    if using_parens {
        context.expect_tok(TokenType::CloseParen, None);
    }
    let body: AstNode = parse_statement(context);

    return AstNode {
        name: String::from("foreach"),
        children: vec![expression, body],
        strings: Some(vec![id]),
        ints: None,
        params: None,
    };
}

fn parse_func(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("func"));
    let name: String = context.expect_tok(TokenType::Id, None).value.clone();
    let params: FuncParams = parse_func_params(context);
    let body: AstNode = parse_statement(context);

    return AstNode {
        name: String::from("func"),
        children: vec![body],
        strings: Some(vec![name]),
        ints: None,
        params: Some(params),
    };
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
    let return_type: Option<AstNode> = if context.accept_tok(TokenType::Colon, None) {
        Some(parse_expression(context))
    } else {
        None
    };

    return FuncParams {
        names,
        types: Box::new(types),
        variadic,
        return_type: Box::new(return_type),
    };
}

fn parse_if(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("if"));
    let mut children: Vec<AstNode> = Vec::new();
    let using_parens: bool = context.accept_tok(TokenType::OpenParen, None);
    children.push(parse_expression(context));
    if using_parens {
        context.expect_tok(TokenType::CloseParen, None);
    }
    children.push(parse_statement(context));
    if context.accept_tok(TokenType::Id, Some("else")) {
        children.push(parse_statement(context));
    }

    return AstNode {
        name: String::from("if"),
        children: children,
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_import(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("import"));
    return AstNode {
        name: String::from("import"),
        children: vec![parse_expression(context)],
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_raise(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("raise"));
    return AstNode {
        name: String::from("raise"),
        children: vec![parse_expression(context)],
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_return(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("return"));
    return AstNode {
        name: String::from("return"),
        children: vec![parse_expression(context)],
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_super(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("super"));
    context.expect_tok(TokenType::OpenParen, None);
    let mut children: Vec<AstNode> = Vec::new();
    while !context.accept_tok(TokenType::CloseParen, None) {
        children.push(parse_expression(context));
        if !context.match_tok(TokenType::CloseParen, None) {
            context.expect_tok(TokenType::Comma, None);
        }
    }
    return AstNode {
        name: String::from("super"),
        children,
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_try_catch(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("try"));
    let _try: AstNode = parse_statement(context);
    context.expect_tok(TokenType::Id, Some("catch"));
    let strings: Option<Vec<String>> = if context.accept_tok(TokenType::OpenParen, None) {
        Some(vec![context.expect_tok(TokenType::Id, None).value.clone()])
    } else {
        None
    };
    let _catch: AstNode = parse_statement(context);
    return AstNode {
        name: String::from("try_catch"),
        children: vec![_try, _catch],
        strings,
        ints: None,
        params: None,
    };
}

fn parse_while(context: &mut ParserContext) -> AstNode {
    context.expect_tok(TokenType::Id, Some("while"));
    let using_parens: bool = context.accept_tok(TokenType::OpenParen, None);
    let condition: AstNode = parse_expression(context);
    if using_parens {
        context.expect_tok(TokenType::CloseParen, None);
    }
    let body: AstNode = parse_statement(context);
    return AstNode {
        name: String::from("while"),
        children: vec![condition, body],
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_expression_statement(context: &mut ParserContext) -> AstNode {
    return AstNode {
        name: String::from("expression_statement"),
        children: vec![parse_expression(context)],
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_expression(context: &mut ParserContext) -> AstNode {
    return todo!();
}

/*
    return AstNode::Statement {
        block: None,
        _break: None,
        _class: None,
        _continue: None,
        empty: None,
        _for: None,
        foreach: None,
        func: None,
        _if: None,
        import: None,
        raise: None,
        _return: None,
        _super: None,
        try_catch: None,
        _while: None,
        expression_statement: None,
        assign: None,
    };
*/
