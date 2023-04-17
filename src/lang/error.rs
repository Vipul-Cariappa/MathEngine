use super::{lexer::Token, parser::Nodes};
use math_engine::math::MathError;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Error {
    LexerError {
        position: usize,
        statement: String,
        message: &'static str,
    },
    ParserError {
        token: Token,
        message: &'static str,
    },
    EvalError {
        node: Nodes,
        message: &'static str,
    },
    MathError(MathError),
}

impl From<MathError> for Error {
    fn from(err: MathError) -> Self {
        Error::MathError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::LexerError {
                position,
                statement,
                message,
            } => {
                write!(
                    f,
                    "\n | {}\n   {}^\nLexer Error: {}\n",
                    statement,
                    " ".repeat(*position - 1),
                    message
                )
            }
            Error::ParserError { token, message } => {
                write!(f, "\n Parser Error: {},\n  at token {:?}\n", message, token)
            }
            Error::EvalError { node, message } => {
                write!(
                    f,
                    "\n Interpreter Error: {},\n  at node {}\n",
                    message, node
                )
            }
            Error::MathError(e) => {
                write!(f, "\n Math Error: {:?}\n", e)
            }
        }
    }
}
