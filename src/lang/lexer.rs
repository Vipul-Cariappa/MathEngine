#[derive(Debug, Clone, Copy)]
pub enum Token {
    NoneToken,
    PlusToken,
    MinusToken,
    MulToken,
    DivToken,
    PowToken,
    EqualToken,
    ForToken,   // @
    CommaToken, // ,
    LeftParenToken,
    RightParenToken,
    IntegerToken(i64),
    DecimalToken(f64),
    VariableToken(char),
}

#[derive(Debug)]
pub struct LexerError {
    pub position: usize,
    pub message: String, // position where error occurred and error message
}

struct Statement {
    string: String,
    position: usize,
}

impl Iterator for Statement {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        match self.string.chars().nth(self.position) {
            Some(x) => {
                self.position += 1;
                Some((self.position - 1, x))
            }
            None => {
                self.position += 1;
                None
            }
        }
    }
}

impl Statement {
    // fn get_char(&self) -> char {
    //     self.string.chars().nth(self.position - 1).unwrap()
    // }

    // fn get_position(&self) -> usize {
    //     self.position - 1
    // }

    fn present(&self) -> Option<(usize, char)> {
        match self.string.chars().nth(self.position - 1) {
            Some(x) => Some((self.position, x)),
            None => None,
        }
    }
}

pub struct Lexer {
    statement: Statement,
    present_token: Token,
}

impl Iterator for Lexer {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((_, c)) = self.statement.present() {
            if c.is_whitespace() {
                self.statement.next();
                continue;
            } else if c.is_numeric() || c == '.' {
                match self.generate_number() {
                    Ok(x) => {
                        self.present_token = x;
                        return Some(Ok(self.present_token));
                    }
                    Err(x) => {
                        self.present_token = Token::NoneToken;
                        return Some(Err(x));
                    }
                }
            } else if c.is_alphabetic() {
                self.present_token = Token::VariableToken(c);
                self.statement.next();
                return Some(Ok(self.present_token));
            } else if c == '+' {
                self.present_token = Token::PlusToken;
                self.statement.next();
                return Some(Ok(self.present_token));
            } else if c == '-' {
                self.present_token = Token::MinusToken;
                self.statement.next();
                return Some(Ok(self.present_token));
            } else if c == '*' {
                self.present_token = Token::MulToken;
                self.statement.next();
                return Some(Ok(self.present_token));
            } else if c == '/' {
                self.present_token = Token::DivToken;
                self.statement.next();
                return Some(Ok(self.present_token));
            } else if c == '^' {
                self.present_token = Token::PowToken;
                self.statement.next();
                return Some(Ok(self.present_token));
            } else if c == '=' {
                self.present_token = Token::EqualToken;
                self.statement.next();
                return Some(Ok(self.present_token));
            } else if c == '(' {
                self.present_token = Token::LeftParenToken;
                self.statement.next();
                return Some(Ok(self.present_token));
            } else if c == ')' {
                self.present_token = Token::RightParenToken;
                self.statement.next();
                return Some(Ok(self.present_token));
            } else if c == '@' {
                self.present_token = Token::ForToken;
                self.statement.next();
                return Some(Ok(self.present_token));
            } else if c == ',' {
                self.present_token = Token::CommaToken;
                self.statement.next();
                return Some(Ok(self.present_token));
            }
        }

        self.present_token = Token::NoneToken;
        return None;
    }
}

impl Lexer {
    pub fn new(string: String) -> Self {
        let mut r = Lexer {
            statement: Statement {
                string: string,
                position: 0,
            },
            present_token: Token::NoneToken,
        };
        r.statement.next();

        return r;
    }

    pub fn present(&self) -> Token {
        return self.present_token;
    }

    fn generate_number(&mut self) -> Result<Token, LexerError> {
        let mut num_string: String = String::new();
        let mut decimal: bool = false;

        while let Some((p, c)) = self.statement.present() {
            if c.is_numeric() {
                num_string.push(c);
                self.statement.next();
            } else if c == '.' {
                if decimal == true {
                    return Err(LexerError {
                        position: p,
                        message: "Found two decimal points in single number".to_string(),
                    });
                }
                decimal = true;
                num_string.push(c);
                self.statement.next();
            } else {
                break;
            }
        }

        if decimal {
            return Ok(Token::DecimalToken(num_string.parse().unwrap()));
        }
        return Ok(Token::IntegerToken(num_string.parse().unwrap()));
    }

    // fn generate_variable(&mut self) -> Result<Token, LexerError> {
    //     let mut result_variable: String = String::new();
    //     result_variable.push(self.statement.get_char());

    //     while let Some((_, c)) = self.statement.next() {
    //         if c.is_alphabetic() {
    //             result_variable.push(c);
    //         } else {
    //             break;
    //         }
    //     }

    //     return Ok(Token::VariableToken(result_variable));
    // }
}
