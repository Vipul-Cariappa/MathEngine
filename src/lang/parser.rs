use super::lexer::{Lexer, LexerError, Token};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum Nodes {
    IntegerNode(i64),
    DecimalNode(f64),
    VariableNode(char),
    AddNode {
        lhs: Box<Nodes>,
        rhs: Box<Nodes>,
    },
    SubNode {
        lhs: Box<Nodes>,
        rhs: Box<Nodes>,
    },
    MulNode {
        lhs: Box<Nodes>,
        rhs: Box<Nodes>,
    },
    DivNode {
        numerator: Box<Nodes>,
        denominator: Box<Nodes>,
    },
    PowNode {
        base: Box<Nodes>,
        exponent: Box<Nodes>,
    },
    MinusNode(Box<Nodes>),
    Equation {
        lhs: Box<Nodes>,
        rhs: Box<Nodes>,
    },
    SolutionNode {
        eq: Box<Nodes>,
        at: Box<Nodes>,
    },
    SubstituteNode(char, Option<Box<Nodes>>), // substitute a variable to integer or decimal
}

impl Display for Nodes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Nodes::IntegerNode(i) => write!(f, "{}", i),
            Nodes::DecimalNode(i) => write!(f, "{}", i),
            Nodes::VariableNode(i) => write!(f, "{}", i),
            Nodes::AddNode { lhs, rhs } => write!(f, "({} + {})", lhs, rhs),
            Nodes::SubNode { lhs, rhs } => write!(f, "({} - {})", lhs, rhs),
            Nodes::MulNode { lhs, rhs } => write!(f, "({} * {})", lhs, rhs),
            Nodes::DivNode {
                numerator,
                denominator,
            } => write!(f, "({} / {})", numerator, denominator),
            Nodes::PowNode { base, exponent } => {
                write!(f, "({} ^ {})", base, exponent)
            }
            Nodes::Equation { lhs, rhs } => write!(f, "({} = {})", lhs, rhs),
            Nodes::MinusNode(value) => write!(f, "-({})", value),
            Nodes::SubstituteNode(c, v) => match v {
                Some(v) => write!(f, "  substitute {} with {}", c, v),
                None => write!(f, "solve for {}", c),
            },
            Nodes::SolutionNode { eq, at } => write!(f, "{} @ {}", eq, at),
        }
    }
}

impl Nodes {
    // fn eval(&self) -> ?? {

    // }
}

#[derive(Debug)]
pub enum ParserError {
    ParserError { token: Token, message: String },
    LexerError(LexerError),
}

pub struct Parser {
    // statement: String,
    tokenizer: Lexer,
}

impl Parser {
    pub fn new(statement: String) -> Self {
        Parser {
            // statement: statement.clone(),
            tokenizer: Lexer::new(statement),
        }
    }
}

impl From<LexerError> for ParserError {
    fn from(item: LexerError) -> Self {
        ParserError::LexerError(item)
    }
}

impl Parser {
    pub fn parse(&mut self) -> Result<Nodes, ParserError> {
        self.tokenizer.next();
        return self.solution();
    }

    fn solution(&mut self) -> Result<Nodes, ParserError> {
        let eq: Nodes = self.equation()?;

        if let Token::ForToken = self.tokenizer.present() {
            self.tokenizer.next();
            return Ok(Nodes::SolutionNode {
                eq: Box::new(eq),
                at: Box::new(self.substitute()?),
            });
        }

        return Ok(eq);
    }

    fn equation(&mut self) -> Result<Nodes, ParserError> {
        let eq: Nodes = self.expression()?;

        if let Token::EqualToken = self.tokenizer.present() {
            self.tokenizer.next();
            return Ok(Nodes::Equation {
                lhs: Box::new(eq),
                rhs: Box::new(self.expression()?),
            });
        }

        return Ok(eq);
    }

    fn expression(&mut self) -> Result<Nodes, ParserError> {
        let mut eq: Nodes = self.term()?;

        loop {
            if let Token::PlusToken = self.tokenizer.present() {
                self.tokenizer.next();
                eq = Nodes::AddNode {
                    lhs: Box::new(eq),
                    rhs: Box::new(self.term()?),
                };
            } else if let Token::MinusToken = self.tokenizer.present() {
                self.tokenizer.next();
                eq = Nodes::SubNode {
                    lhs: Box::new(eq),
                    rhs: Box::new(self.term()?),
                };
            } else {
                break;
            }
        }

        return Ok(eq);
    }

    fn term(&mut self) -> Result<Nodes, ParserError> {
        let mut eq: Nodes = self.exponent()?;

        loop {
            if let Token::MulToken = self.tokenizer.present() {
                self.tokenizer.next();
                eq = Nodes::MulNode {
                    lhs: Box::new(eq),
                    rhs: Box::new(self.exponent()?),
                };
            } else if let Token::DivToken = self.tokenizer.present() {
                self.tokenizer.next();
                eq = Nodes::DivNode {
                    numerator: Box::new(eq),
                    denominator: Box::new(self.exponent()?),
                };
            } else {
                break;
            }
        }

        return Ok(eq);
    }

    fn exponent(&mut self) -> Result<Nodes, ParserError> {
        let mut eq: Nodes = self.factor()?;

        loop {
            if let Token::PowToken = self.tokenizer.present() {
                self.tokenizer.next();
                eq = Nodes::PowNode {
                    base: Box::new(eq),
                    exponent: Box::new(self.factor()?),
                };
            } else {
                break;
            }
        }

        return Ok(eq);
    }

    fn factor(&mut self) -> Result<Nodes, ParserError> {
        match self.tokenizer.present() {
            Token::IntegerToken(i) => {
                self.tokenizer.next();
                return Ok(Nodes::IntegerNode(i));
            }
            Token::DecimalToken(i) => {
                self.tokenizer.next();
                return Ok(Nodes::DecimalNode(i));
            }
            Token::VariableToken(i) => {
                self.tokenizer.next();
                return Ok(Nodes::VariableNode(i));
            }
            Token::PlusToken => {
                self.tokenizer.next();
                return self.factor();
            }
            Token::MinusToken => {
                self.tokenizer.next();
                return Ok(Nodes::MinusNode(Box::new(self.factor()?)));
            }
            Token::LeftParenToken => {
                self.tokenizer.next();

                let eq: Nodes = self.expression()?;

                match self.tokenizer.present() {
                    Token::RightParenToken => {
                        self.tokenizer.next();
                        return Ok(eq);
                    }

                    _ => {}
                }
                return Err(ParserError::ParserError {
                    token: self.tokenizer.present(),
                    message: "Expected ')'".to_string(),
                });
            }
            _ => {}
        }

        return Err(ParserError::ParserError {
            token: self.tokenizer.present(),
            message: "Expected variable or integer or decimal token but got some thing else."
                .to_string(),
        });
    }

    fn substitute(&mut self) -> Result<Nodes, ParserError> {
        let variable: char = match self.tokenizer.present() {
            Token::VariableToken(i) => i,
            n => {
                return Err(ParserError::ParserError {
                    token: n,
                    message:
                        "Expected variable token after @ to solve for, but found something else"
                            .to_string(),
                });
            }
        };

        match self.tokenizer.next() {
            Some(x) => {
                let x: Token = x?;

                if let Token::CommaToken = x {
                } else {
                    return Err(ParserError::ParserError {
                        token: x,
                        message: "Expected end of line or comma, but found something else"
                            .to_string(),
                    });
                }
            }
            _ => {
                self.tokenizer.next();
                return Ok(Nodes::SubstituteNode(variable, None));
            }
        };

        let substitute_value: Nodes = match self.tokenizer.next() {
            Some(x) => {
                let x: Token = x?;

                if let Token::VariableToken(i) = x {
                    Nodes::VariableNode(i)
                } else if let Token::IntegerToken(i) = x {
                    Nodes::IntegerNode(i)
                } else if let Token::DecimalToken(i) = x {
                    Nodes::DecimalNode(i)
                } else {
                    return Err(ParserError::ParserError {
                        token: x,
                        message:
                            "Expected variable token after @ to solve for, but found something else"
                                .to_string(),
                    });
                }
            }
            None => {
                return Err(ParserError::ParserError {
                    token: Token::NoneToken,
                    message: "Expected variable token after @ to solve for, but found nothing"
                        .to_string(),
                });
            }
        };

        self.tokenizer.next();
        return Ok(Nodes::SubstituteNode(
            variable,
            Some(Box::new(substitute_value)),
        ));
    }
}
