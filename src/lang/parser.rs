use super::error::Error;
use super::lexer::{Lexer, Token};
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
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
    EquationNode {
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
            Nodes::EquationNode { lhs, rhs } => write!(f, "({} = {})", lhs, rhs),
            Nodes::MinusNode(value) => write!(f, "-({})", value),
            Nodes::SubstituteNode(c, v) => match v {
                Some(v) => write!(f, "  substitute {} with {}", c, v),
                None => write!(f, "solve for {}", c),
            },
            Nodes::SolutionNode { eq, at } => write!(f, "{} @ {}", eq, at),
        }
    }
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

impl Parser {
    pub fn parse(&mut self) -> Result<Nodes, Error> {
        self.tokenizer.next();
        let ast =  self.solution();
        if let Token::NoneToken = self.tokenizer.present()? {
            ast
        } else {
            Err(Error::ParserError { token: self.tokenizer.present()?, message: "Expected end of line, but got a tokeng" })
        }
    }

    fn solution(&mut self) -> Result<Nodes, Error> {
        let eq: Nodes = self.equation()?;

        if let Token::ForToken = self.tokenizer.present()? {
            self.tokenizer.next();
            return Ok(Nodes::SolutionNode {
                eq: Box::new(eq),
                at: Box::new(self.substitute()?),
            });
        }

        return Ok(eq);
    }

    fn equation(&mut self) -> Result<Nodes, Error> {
        let eq: Nodes = self.expression()?;

        if let Token::EqualToken = self.tokenizer.present()? {
            self.tokenizer.next();
            return Ok(Nodes::EquationNode {
                lhs: Box::new(eq),
                rhs: Box::new(self.expression()?),
            });
        }

        return Ok(eq);
    }

    fn expression(&mut self) -> Result<Nodes, Error> {
        let mut eq: Nodes = self.term()?;

        loop {
            if let Token::PlusToken = self.tokenizer.present()? {
                self.tokenizer.next();
                eq = Nodes::AddNode {
                    lhs: Box::new(eq),
                    rhs: Box::new(self.term()?),
                };
            } else if let Token::MinusToken = self.tokenizer.present()? {
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

    fn term(&mut self) -> Result<Nodes, Error> {
        let mut eq: Nodes = self.exponent()?;

        loop {
            if let Token::MulToken = self.tokenizer.present()? {
                self.tokenizer.next();
                eq = Nodes::MulNode {
                    lhs: Box::new(eq),
                    rhs: Box::new(self.exponent()?),
                };
            } else if let Token::DivToken = self.tokenizer.present()? {
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

    fn exponent(&mut self) -> Result<Nodes, Error> {
        let mut eq: Nodes = self.factor()?;

        loop {
            if let Token::PowToken = self.tokenizer.present()? {
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

    fn factor(&mut self) -> Result<Nodes, Error> {
        match self.tokenizer.present()? {
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

                match self.tokenizer.present()? {
                    Token::RightParenToken => {
                        self.tokenizer.next();
                        return Ok(eq);
                    }

                    _ => {}
                }
                return Err(Error::ParserError {
                    token: self.tokenizer.present()?,
                    message: "Expected ')'",
                });
            }
            _ => {}
        }

        return Err(Error::ParserError {
            token: self.tokenizer.present()?,
            message: "Expected variable or integer or decimal token but got some thing else.",
        });
    }

    fn substitute(&mut self) -> Result<Nodes, Error> {
        let variable: char = match self.tokenizer.present()? {
            Token::VariableToken(i) => i,
            n => {
                return Err(Error::ParserError {
                    token: n,
                    message:
                        "Expected variable token after @ to solve for, but found something else",
                });
            }
        };

        match self.tokenizer.next() {
            Some(x) => {
                let x: Token = x?;

                if let Token::CommaToken = x {
                } else {
                    return Err(Error::ParserError {
                        token: x,
                        message: "Expected end of line or comma, but found something else",
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
                    return Err(Error::ParserError {
                        token: x,
                        message:
                            "Expected variable token after @ to solve for, but found something else",
                    });
                }
            }
            None => {
                return Err(Error::ParserError {
                    token: Token::NoneToken,
                    message: "Expected variable token after @ to solve for, but found nothing",
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
