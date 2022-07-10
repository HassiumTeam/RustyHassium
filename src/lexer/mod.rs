#[derive(Clone)]
pub struct Token {
    pub token_type: String,
    pub value: String,
}

struct LexerContext {
    pub code: String,
    pub pos: usize,
    pub len: usize,
    pub tokens: Vec<Token>,
}

impl LexerContext {
    fn peek(&mut self) -> u32 {
        if self.pos < self.len {
            return self.code.chars().nth(self.pos).unwrap().try_into().unwrap();
        }
        return 0;
    }

    fn peek_next(&mut self) -> u32 {
        if (self.pos + 1) < self.len {
            return self
                .code
                .chars()
                .nth(self.pos + 1)
                .unwrap()
                .try_into()
                .unwrap();
        }
        return 0;
    }

    fn read(&mut self) -> u32 {
        if self.pos < self.len {
            let ret = self.code.chars().nth(self.pos).unwrap().try_into().unwrap();
            self.pos += 1;
            return ret;
        }
        return 0;
    }

    fn add_tok(&mut self, token_type: &str, value: &str) {
        self.tokens.push(Token {
            token_type: token_type.to_string(),
            value: value.to_string(),
        });
    }
}

pub fn tokenize(code: String) -> Vec<Token> {
    let len = code.len();
    let mut context: LexerContext = LexerContext {
        code,
        pos: 0,
        len,
        tokens: Vec::new(),
    };

    while context.pos < context.len {
        whitespace(&mut context);
        let cur: char = context.peek().try_into().unwrap();
        let next: char = context.peek_next().try_into().unwrap();

        if cur.is_ascii_alphabetic() || cur == '_' {
            read_id(&mut context);
        } else if cur.is_ascii_digit() {
            read_number(&mut context);
        } else {
            match cur {
                '"' => read_str(&mut context, '"'),
                '\'' => read_str(&mut context, '\''),
                '=' => {
                    if next == '=' {
                        context.add_tok("op", "==");
                        context.read();
                    } else {
                        context.add_tok("assign", "=");
                        context.read();
                    }
                }
                '+' | '-' | '*' | '/' | '%' | '^' => {
                    if next == '=' {
                        context.add_tok("assign", &format!("{}{}", cur, next));
                        context.read();
                        context.read();
                    } else {
                        context.add_tok("op", &cur.to_string());
                        context.read();
                    }
                }
                '&' | '|' => {
                    if next == '=' {
                        context.add_tok("assign", &format!("{}{}", cur, next));
                        context.read();
                        context.read();
                    } else if next == cur {
                        context.add_tok("op", &format!("{}{}", cur, next));
                        context.read();
                        context.read();
                    } else {
                        context.add_tok("op", &cur.to_string());
                        context.read();
                    }
                }
                '!' | '>' | '<' => {
                    if next == '=' {
                        context.add_tok("op", &format!("{}{}", cur, next));
                        context.read();
                        context.read();
                    } else {
                        context.add_tok("op", &cur.to_string());
                        context.read();
                    }
                }
                '(' => {
                    context.add_tok("oparen", "(");
                    context.read();
                }
                ')' => {
                    context.add_tok("cparen", ")");
                    context.read();
                }
                ',' => {
                    context.add_tok("comma", ",");
                    context.read();
                }
                '.' => {
                    if next == '.' {
                        context.add_tok("variadic", "..");
                        context.read();
                        context.read();
                    } else {
                        context.add_tok("dot", ".");
                        context.read();
                    }
                }
                ':' => {
                    context.add_tok("colon", ":");
                    context.read();
                }
                ';' => {
                    context.add_tok("semicolon", ";");
                    context.read();
                }
                _ => panic!("Unknown char \"{}\"!", cur),
            }
        }
    }

    return context.tokens;
}

fn whitespace(context: &mut LexerContext) {
    while context.peek() != 0 && char::from_u32(context.peek()).unwrap().is_whitespace() {
        context.read();
    }
}

fn read_id(context: &mut LexerContext) {
    let mut string: String = String::new();

    while context.peek() != 0
        && (char::from_u32(context.peek())
            .unwrap()
            .is_ascii_alphanumeric()
            || context.peek() == '_' as u32)
    {
        string.push(context.read().try_into().unwrap());
    }

    context.tokens.push(Token {
        token_type: String::from("id"),
        value: string,
    });
}

fn read_number(context: &mut LexerContext) {
    let mut string: String = String::new();

    while context.peek() != 0
        && (char::from_u32(context.peek()).unwrap().is_ascii_digit()
            || context.peek() == '.' as u32)
    {
        string.push(context.read().try_into().unwrap());
    }

    context.tokens.push(Token {
        token_type: String::from("number"),
        value: string,
    });
}

fn read_str(context: &mut LexerContext, delin: char) {
    let mut string: String = String::new();
    context.read();
    while context.peek() != 0 && char::from_u32(context.peek()).unwrap() != delin {
        string.push(context.read().try_into().unwrap());
    }
    context.read();

    context.tokens.push(Token {
        token_type: String::from("string"),
        value: string,
    });
}

pub fn print_tokens(tokens: &mut Vec<Token>) {
    println!("Lexer tokens:");
    for token in tokens {
        println!("Type: {}, Value: {}", token.token_type, token.value);
    }
}
