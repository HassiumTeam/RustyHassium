mod ast;

use crate::lexer::Token;

use self::ast::{AstNode, FuncParams};

struct ParserContext {
    tokens: Vec<Token>,
    pos: u32,
    len: usize,
}

impl ParserContext {
    fn eof(&mut self) -> bool {
        return self.pos as usize >= self.len;
    }

    fn current(&mut self) -> Option<&Token> {
        return self.tokens.get(self.pos as usize);
    }

    fn match_tok(&mut self, token_type: &str, value: Option<&str>) -> bool {
        let current_option = self.current();
        if current_option.is_none() {
            return false;
        }
        let current = current_option.unwrap();
        if value.is_none() {
            return current.token_type == token_type;
        } else {
            return current.token_type == token_type && current.value == value.unwrap();
        }
    }

    fn accept_tok(&mut self, token_type: &str, value: Option<&str>) -> bool {
        let current_option = self.current();
        if current_option.is_none() {
            return false;
        }
        let current = current_option.unwrap();
        let ret: bool;
        if value.is_none() {
            ret = current.token_type == token_type;
        } else {
            ret = current.token_type == token_type && current.value == value.unwrap();
        }
        if ret {
            self.pos += 1;
        }
        return ret;
    }

    fn expect_tok(&mut self, token_type: &str, value: Option<&str>) -> &Token {
        let current_option = self.current();
        if current_option.is_none() {
            panic!("Unexpected EOF");
        }
        let current = current_option.unwrap();
        if value.is_none() {
            if current.token_type == token_type {
                return current;
            } else {
                panic!(
                    "Unexpected {} '{}', expected {}!",
                    current.token_type, current.value, token_type
                );
            }
        } else {
            let _value = value.unwrap();
            if current.token_type == token_type && current.value == _value {
                return current;
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

    return AstNode {
        name: String::from("program"),
        children,
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_statement(context: &mut ParserContext) -> AstNode {
    let statement: AstNode = if context.match_tok("oparen", None) {
        parse_block(context)
    } else if context.match_tok("id", Some("break")) {
        parse_break(context)
    } else if context.match_tok("id", Some("continue")) {
        parse_continue(context)
    } else if context.match_tok("id", Some("class")) {
        parse_class(context)
    } else if context.match_tok("id", Some("continue")) {
        parse_continue(context)
    } else if context.match_tok("id", Some("for")) {
        parse_for(context)
    } else if context.match_tok("id", Some("foreach")) {
        parse_foreach(context)
    } else if context.match_tok("id", Some("func")) {
        parse_func(context)
    } else if context.match_tok("id", Some("if")) {
        parse_if(context)
    } else if context.match_tok("id", Some("import")) {
        parse_import(context)
    } else if context.match_tok("id", Some("raise")) {
        parse_raise(context)
    } else if context.match_tok("id", Some("return")) {
        parse_return(context)
    } else if context.match_tok("id", Some("super")) {
        parse_super(context)
    } else if context.match_tok("id", Some("try")) {
        parse_try_catch(context)
    } else if context.match_tok("id", Some("while")) {
        parse_while(context)
    } else {
        parse_expression_statement(context)
    };

    context.accept_tok("semicolon", None);

    return statement;
}

fn parse_block(context: &mut ParserContext) -> AstNode {
    context.expect_tok("obrace", None);
    let mut children: Vec<AstNode> = Vec::new();
    while !context.accept_tok("cbrace", None) {
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
    context.expect_tok("id", Some("break"));
    return AstNode {
        name: String::from("break"),
        children: Vec::new(),
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_class(context: &mut ParserContext) -> AstNode {
    context.expect_tok("id", Some("class"));
    let mut children: Vec<AstNode> = Vec::new();
    let name: String = context.expect_tok("id", None).value.clone();
    if context.accept_tok("id", Some("extends")) {
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
    context.expect_tok("id", Some("continue"));
    return AstNode {
        name: String::from("continue"),
        children: Vec::new(),
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_for(context: &mut ParserContext) -> AstNode {
    context.expect_tok("id", Some("for"));
    let using_parens = context.accept_tok("oparen", None);
    let initial = parse_expression(context);
    context.accept_tok("semicolon", None);
    let expression: AstNode = parse_expression(context);
    context.accept_tok("semicolon", None);
    let repeated: AstNode = parse_expression(context);
    if using_parens {
        context.expect_tok("cparen", None);
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
    context.expect_tok("id", Some("foreach"));
    let using_parens: bool = context.accept_tok("oparen", None);
    let id: String = context.expect_tok("id", None).value.clone();
    context.expect_tok("id", Some("in"));
    let expression: AstNode = parse_expression(context);
    if using_parens {
        context.expect_tok("cparen", None);
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
    context.expect_tok("id", Some("func"));
    let name: String = context.expect_tok("id", None).value.clone();
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
    context.expect_tok("oparen", None);
    let mut names: Vec<String> = Vec::new();
    let mut types: Vec<Option<AstNode>> = Vec::new();
    let mut variadic: bool = false;
    while !context.accept_tok("cparen", None) {
        if context.accept_tok("variadic", None) {
            variadic = true;
            context.expect_tok("cparen", None);
            break;
        }
        names.push(context.expect_tok("id", None).value.clone());
        types.push(if context.accept_tok("colon", None) {
            Some(parse_expression(context))
        } else {
            None
        });
        // If the next token is not a comma or a cparen, fail by expecting the comma
        if !context.match_tok("comma", None) && !context.match_tok("cparen", None) {
            context.expect_tok("comma", None);
        }
    }
    let return_type: Option<AstNode> = if context.accept_tok("colon", None) {
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
    context.expect_tok("id", Some("if"));
    let mut children: Vec<AstNode> = Vec::new();
    let using_parens: bool = context.accept_tok("oparen", None);
    children.push(parse_expression(context));
    if using_parens {
        context.expect_tok("cparen", None);
    }
    children.push(parse_statement(context));
    if context.accept_tok("id", Some("else")) {
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
    context.expect_tok("id", Some("import"));
    return AstNode {
        name: String::from("import"),
        children: vec![parse_expression(context)],
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_raise(context: &mut ParserContext) -> AstNode {
    context.expect_tok("id", Some("raise"));
    return AstNode {
        name: String::from("raise"),
        children: vec![parse_expression(context)],
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_return(context: &mut ParserContext) -> AstNode {
    context.expect_tok("id", Some("return"));
    return AstNode {
        name: String::from("return"),
        children: vec![parse_expression(context)],
        strings: None,
        ints: None,
        params: None,
    };
}

fn parse_super(context: &mut ParserContext) -> AstNode {
    context.expect_tok("id", Some("super"));
    context.expect_tok("oparen", None);
    let mut children: Vec<AstNode> = Vec::new();
    while !context.accept_tok("cparen", None) {
        children.push(parse_expression(context));
        if !context.match_tok("cparen", None) {
            context.expect_tok("comma", None);
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
    context.expect_tok("id", Some("try"));
    let _try: AstNode = parse_statement(context);
    context.expect_tok("id", Some("catch"));
    let strings: Option<Vec<String>> = if context.accept_tok("oparen", None) {
        Some(vec![context.expect_tok("id", None).value.clone()])
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
    context.expect_tok("id", Some("while"));
    let using_parens: bool = context.accept_tok("oparen", None);
    let condition: AstNode = parse_expression(context);
    if using_parens {
        context.expect_tok("cparen", None);
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
