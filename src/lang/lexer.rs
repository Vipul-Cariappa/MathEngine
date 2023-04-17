use super::error::Error;

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
    err: Error,
    err_occurred: bool,
}

impl Iterator for Lexer {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((p, c)) = self.statement.present() {
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
                        self.err_occurred = true;
                        self.err = x.clone();
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
            } else {
                self.err_occurred = true;
                self.err = Error::LexerError {
                    position: p,
                    statement: self.statement.string.clone(),
                    message: "Got unexpected character",
                };

                return Some(Err(Error::LexerError {
                    position: p,
                    statement: self.statement.string.clone(),
                    message: "Got unexpected character",
                }));
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
            err: Error::LexerError {
                position: 0,
                statement: String::new(),
                message: "",
            },
            err_occurred: false,
        };
        r.statement.next();

        return r;
    }

    pub fn present(&self) -> Result<Token, Error> {
        if self.err_occurred {
            return Err(self.err.clone());
        }
        return Ok(self.present_token);
    }

    fn generate_number(&mut self) -> Result<Token, Error> {
        let mut num_string: String = String::new();
        let mut decimal: bool = false;

        while let Some((p, c)) = self.statement.present() {
            if c.is_numeric() {
                num_string.push(c);
                self.statement.next();
            } else if c == '.' {
                if decimal == true {
                    return Err(Error::LexerError {
                        position: p,
                        statement: self.statement.string.clone(),
                        message: "Found two decimal points in single number",
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
}
