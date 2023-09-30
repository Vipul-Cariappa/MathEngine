use super::error::Error;
use super::parser::{Nodes, Parser};
use math_engine::equation::{Equation, PartEquation};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum EvalResult {
    Equation(Equation),
    PartEquation(PartEquation),
}

impl Display for EvalResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalResult::Equation(e) => write!(f, "{}", e),
            EvalResult::PartEquation(e) => write!(f, "{}", e),
        }
    }
}

fn eval(node: Nodes) -> Result<EvalResult, Error> {
    match node {
        Nodes::IntegerNode(i) => Ok(EvalResult::PartEquation(PartEquation::from(i))),
        Nodes::DecimalNode(i) => Ok(EvalResult::PartEquation(PartEquation::from(i))),
        Nodes::VariableNode(i) => Ok(EvalResult::PartEquation(PartEquation::from(i))),
        Nodes::AddNode { lhs, rhs } => {
            let lhs: PartEquation = {
                match eval(*lhs.clone())? {
                    EvalResult::Equation(_) => {
                        return Err(Error::EvalError {
                            node: *lhs,
                            message: "Got Equation where PartEquation was expected",
                        });
                    }
                    EvalResult::PartEquation(e) => e,
                }
            };
            let rhs: PartEquation = {
                match eval(*rhs.clone())? {
                    EvalResult::Equation(_) => {
                        return Err(Error::EvalError {
                            node: *rhs,
                            message: "Got Equation where PartEquation was expected",
                        });
                    }
                    EvalResult::PartEquation(e) => e,
                }
            };

            Ok(EvalResult::PartEquation(lhs + rhs))
        }
        Nodes::SubNode { lhs, rhs } => {
            let lhs: PartEquation = {
                match eval(*lhs.clone())? {
                    EvalResult::Equation(_) => {
                        return Err(Error::EvalError {
                            node: *lhs,
                            message: "Got Equation where PartEquation was expected",
                        });
                    }
                    EvalResult::PartEquation(e) => e,
                }
            };
            let rhs: PartEquation = {
                match eval(*rhs.clone())? {
                    EvalResult::Equation(_) => {
                        return Err(Error::EvalError {
                            node: *rhs,
                            message: "Got Equation where PartEquation was expected",
                        });
                    }
                    EvalResult::PartEquation(e) => e,
                }
            };

            Ok(EvalResult::PartEquation(lhs - rhs))
        }
        Nodes::MulNode { lhs, rhs } => {
            let lhs: PartEquation = {
                match eval(*lhs.clone())? {
                    EvalResult::Equation(_) => {
                        return Err(Error::EvalError {
                            node: *lhs,
                            message: "Got Equation where PartEquation was expected",
                        });
                    }
                    EvalResult::PartEquation(e) => e,
                }
            };
            let rhs: PartEquation = {
                match eval(*rhs.clone())? {
                    EvalResult::Equation(_) => {
                        return Err(Error::EvalError {
                            node: *rhs,
                            message: "Got Equation where PartEquation was expected",
                        });
                    }
                    EvalResult::PartEquation(e) => e,
                }
            };

            Ok(EvalResult::PartEquation(lhs * rhs))
        }
        Nodes::DivNode {
            numerator,
            denominator,
        } => {
            let numerator: PartEquation = {
                match eval(*numerator.clone())? {
                    EvalResult::Equation(_) => {
                        return Err(Error::EvalError {
                            node: *numerator,
                            message: "Got Equation where PartEquation was expected",
                        });
                    }
                    EvalResult::PartEquation(e) => e,
                }
            };
            let denominator: PartEquation = {
                match eval(*denominator.clone())? {
                    EvalResult::Equation(_) => {
                        return Err(Error::EvalError {
                            node: *denominator,
                            message: "Got Equation where PartEquation was expected",
                        });
                    }
                    EvalResult::PartEquation(e) => e,
                }
            };

            Ok(EvalResult::PartEquation(numerator / denominator))
        }
        Nodes::PowNode { base, exponent } => {
            let base: PartEquation = {
                match eval(*base.clone())? {
                    EvalResult::Equation(_) => {
                        return Err(Error::EvalError {
                            node: *base,
                            message: "Got Equation where PartEquation was expected",
                        });
                    }
                    EvalResult::PartEquation(e) => e,
                }
            };
            let exponent: PartEquation = {
                match eval(*exponent.clone())? {
                    EvalResult::Equation(_) => {
                        return Err(Error::EvalError {
                            node: *exponent,
                            message: "Got Equation where PartEquation was expected",
                        });
                    }
                    EvalResult::PartEquation(e) => e,
                }
            };

            Ok(EvalResult::PartEquation(base.pow(&exponent)))
        }
        Nodes::MinusNode(i) => {
            let v: PartEquation = {
                match eval(*i.clone())? {
                    EvalResult::Equation(_) => {
                        return Err(Error::EvalError {
                            node: *i,
                            message: "Got Equation where PartEquation was expected",
                        });
                    }
                    EvalResult::PartEquation(e) => e,
                }
            };

            Ok(EvalResult::PartEquation(-v))
        }
        Nodes::EquationNode { lhs, rhs } => {
            let lhs: PartEquation = {
                match eval(*lhs.clone())? {
                    EvalResult::Equation(_) => {
                        return Err(Error::EvalError {
                            node: *lhs,
                            message: "Got Equation where PartEquation was expected",
                        });
                    }
                    EvalResult::PartEquation(e) => e,
                }
            };
            let rhs: PartEquation = {
                match eval(*rhs.clone())? {
                    EvalResult::Equation(_) => {
                        return Err(Error::EvalError {
                            node: *rhs,
                            message: "Got Equation where PartEquation was expected",
                        });
                    }
                    EvalResult::PartEquation(e) => e,
                }
            };

            Ok(EvalResult::Equation(Equation::new(&lhs, &rhs)))
        }
        Nodes::SolutionNode { eq, at } => {
            let eq = eval(*eq.clone())?;

            if let Nodes::SubstituteNode(variable, value) = *at {
                match value {
                    Some(v) => match eq {
                        EvalResult::PartEquation(e) => match *v {
                            Nodes::IntegerNode(i) => {
                                Ok(EvalResult::PartEquation(e.substitute(variable, &PartEquation::from(i))))
                            }
                            Nodes::DecimalNode(i) => {
                                Ok(EvalResult::PartEquation(e.substitute(variable, &PartEquation::from(i))))
                            }
                            _ => Err(Error::EvalError {
                                node: *v,
                                message: "Substitution of value other then integer and decimal is not yet implemented",
                            }),
                        },
                        EvalResult::Equation(_) => {
                            return Err(Error::EvalError {
                                node: *v,
                                message: "Got PartEquation where Equation was expected",
                            });
                        }
                    },
                    None => match eq {
                        EvalResult::Equation(e) => Ok(EvalResult::PartEquation(e.solve(variable)?)),
                        EvalResult::PartEquation(e) => Ok(EvalResult::PartEquation(
                            Equation::new(&e, &PartEquation::from(0)).solve(variable)?,
                        )),
                    },
                }
            } else {
                return Err(Error::EvalError {
                    node: *at,
                    message: "Expected a SubstituteNode got something else",
                });
            }
        }
        n @ Nodes::SubstituteNode(_, _) => {
            return Err(Error::EvalError {
                node: n,
                message: "Got SubstituteNode when expecting anything else",
            });
        }
    }
}

pub fn interpret(statement: String) -> Result<EvalResult, Error> {
    let node: Nodes = Parser::new(statement).parse()?;
    eval(node)
}
