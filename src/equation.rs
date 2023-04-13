use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops;

use crate::math::MathError;

use super::math;

#[derive(Clone)]
enum EquationComponentType {
    Integer(i64),
    Decimal(f64),
    VariableNode(char),
    AddNode {
        lhs: Box<EquationComponentType>,
        rhs: Box<EquationComponentType>,
    },
    SubNode {
        lhs: Box<EquationComponentType>,
        rhs: Box<EquationComponentType>,
    },
    MulNode {
        lhs: Box<EquationComponentType>,
        rhs: Box<EquationComponentType>,
    },
    DivNode {
        numerator: Box<EquationComponentType>,
        denominator: Box<EquationComponentType>,
    },
    PowNode {
        base: Box<EquationComponentType>,
        exponent: Box<EquationComponentType>,
    },
    LogNode {
        base: Box<EquationComponentType>,
        argument: Box<EquationComponentType>,
    },
    MinusNode(Box<EquationComponentType>),
}

impl Debug for EquationComponentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EquationComponentType::Integer(i) => write!(f, "{:?}", i),
            EquationComponentType::Decimal(i) => write!(f, "{:?}", i),
            EquationComponentType::VariableNode(i) => write!(f, "{:?}", i),
            EquationComponentType::AddNode { lhs, rhs } => write!(f, "({:?} + {:?})", lhs, rhs),
            EquationComponentType::SubNode { lhs, rhs } => write!(f, "({:?} - {:?})", lhs, rhs),
            EquationComponentType::MulNode { lhs, rhs } => write!(f, "({:?} * {:?})", lhs, rhs),
            EquationComponentType::DivNode {
                numerator,
                denominator,
            } => write!(f, "({:?} / {:?})", numerator, denominator),
            EquationComponentType::PowNode { base, exponent } => {
                write!(f, "({:?} ^ {:?})", base, exponent)
            }
            EquationComponentType::LogNode { base, argument } => {
                write!(f, "(Log_{:?}({:?}))", base, argument)
            }
            EquationComponentType::MinusNode(value) => write!(f, "-({:?})", value),
        }
    }
}

impl Display for EquationComponentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EquationComponentType::Integer(i) => write!(f, "{}", i),
            EquationComponentType::Decimal(i) => write!(f, "{}", i),
            EquationComponentType::VariableNode(i) => write!(f, "{}", i),
            EquationComponentType::AddNode { lhs, rhs } => write!(f, "({} + {})", lhs, rhs),
            EquationComponentType::SubNode { lhs, rhs } => write!(f, "({} - {})", lhs, rhs),
            EquationComponentType::MulNode { lhs, rhs } => write!(f, "({} * {})", lhs, rhs),
            EquationComponentType::DivNode {
                numerator,
                denominator,
            } => write!(f, "({} / {})", numerator, denominator),
            EquationComponentType::PowNode { base, exponent } => {
                write!(f, "({} ^ {})", base, exponent)
            }
            EquationComponentType::LogNode { base, argument } => {
                write!(f, "(Log_{:?}({:?}))", base, argument)
            }
            EquationComponentType::MinusNode(value) => write!(f, "-({})", value),
        }
    }
}

impl EquationComponentType {
    fn post_simplify(&self) -> Self {
        match self {
            EquationComponentType::Integer(i) => EquationComponentType::Integer(*i),
            EquationComponentType::Decimal(i) => EquationComponentType::Decimal(*i),
            EquationComponentType::VariableNode(i) => EquationComponentType::VariableNode(*i),
            EquationComponentType::MinusNode(value) => match &**value {
                i @ EquationComponentType::Integer(_) => {
                    EquationComponentType::MinusNode(Box::new(i.clone()))
                }
                i @ EquationComponentType::Decimal(_) => {
                    EquationComponentType::MinusNode(Box::new(i.clone()))
                }
                i @ EquationComponentType::VariableNode(_) => {
                    EquationComponentType::MinusNode(Box::new(i.clone()))
                }
                n => n.post_simplify(),
            },
            EquationComponentType::AddNode { lhs, rhs } => {
                let lhs = match &**lhs {
                    i @ EquationComponentType::Integer(_) => i.clone(),
                    i @ EquationComponentType::Decimal(_) => i.clone(),
                    i @ EquationComponentType::VariableNode(_) => i.clone(),
                    n => n.post_simplify(),
                };

                let rhs = match &**rhs {
                    i @ EquationComponentType::Integer(_) => i.clone(),
                    i @ EquationComponentType::Decimal(_) => i.clone(),
                    i @ EquationComponentType::VariableNode(_) => i.clone(),
                    n => n.post_simplify(),
                };

                if let EquationComponentType::VariableNode(_) = rhs {
                    EquationComponentType::AddNode {
                        lhs: Box::new(rhs),
                        rhs: Box::new(lhs),
                    }
                } else if let EquationComponentType::Integer(_) = lhs {
                    EquationComponentType::AddNode {
                        lhs: Box::new(rhs),
                        rhs: Box::new(lhs),
                    }
                } else if let EquationComponentType::Decimal(_) = lhs {
                    EquationComponentType::AddNode {
                        lhs: Box::new(rhs),
                        rhs: Box::new(lhs),
                    }
                } else {
                    EquationComponentType::AddNode {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                }
            }
            EquationComponentType::SubNode { lhs, rhs } => EquationComponentType::AddNode {
                lhs: Box::new(lhs.post_simplify()),
                rhs: Box::new(EquationComponentType::MinusNode(Box::new(
                    rhs.post_simplify(),
                ))),
            },
            EquationComponentType::MulNode { lhs, rhs } => {
                let lhs = match &**lhs {
                    i @ EquationComponentType::Integer(_) => i.clone(),
                    i @ EquationComponentType::Decimal(_) => i.clone(),
                    i @ EquationComponentType::VariableNode(_) => i.clone(),
                    n => n.post_simplify(),
                };

                let rhs = match &**rhs {
                    i @ EquationComponentType::Integer(_) => i.clone(),
                    i @ EquationComponentType::Decimal(_) => i.clone(),
                    i @ EquationComponentType::VariableNode(_) => i.clone(),
                    n => n.post_simplify(),
                };

                if let EquationComponentType::VariableNode(_) = rhs {
                    EquationComponentType::MulNode {
                        lhs: Box::new(rhs),
                        rhs: Box::new(lhs),
                    }
                } else if let EquationComponentType::Integer(_) = lhs {
                    EquationComponentType::MulNode {
                        lhs: Box::new(rhs),
                        rhs: Box::new(lhs),
                    }
                } else if let EquationComponentType::Decimal(_) = lhs {
                    EquationComponentType::MulNode {
                        lhs: Box::new(rhs),
                        rhs: Box::new(lhs),
                    }
                } else {
                    EquationComponentType::MulNode {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                }
            }
            EquationComponentType::DivNode {
                numerator,
                denominator,
            } => EquationComponentType::DivNode {
                numerator: Box::new(numerator.post_simplify()),
                denominator: Box::new(denominator.post_simplify()),
            },
            EquationComponentType::PowNode { base, exponent } => EquationComponentType::PowNode {
                base: Box::new(base.post_simplify()),
                exponent: Box::new(exponent.post_simplify()),
            },
            EquationComponentType::LogNode { base, argument } => EquationComponentType::LogNode {
                base: Box::new(base.post_simplify()),
                argument: Box::new(argument.post_simplify()),
            },
        }
    }

    fn simplify(&self) -> Self {
        match self {
            EquationComponentType::Integer(i) => EquationComponentType::Integer(*i),

            EquationComponentType::Decimal(i) => EquationComponentType::Decimal(*i),

            EquationComponentType::VariableNode(i) => EquationComponentType::VariableNode(*i),

            EquationComponentType::AddNode { lhs: _, rhs: _ } => {
                // extracting simplified child nodes
                let mut variables: Vec<char> = Vec::new();
                let mut integers: Vec<i64> = Vec::new();
                let mut decimals: Vec<f64> = Vec::new();
                let mut nodes: Vec<EquationComponentType> = Vec::new();

                self.extract(&mut variables, &mut integers, &mut decimals, &mut nodes);

                // calculating the constant's value
                let mut sum_i64: i64 = 0;
                integers.iter().for_each(|x| sum_i64 += x);

                let mut sum_f64: f64 = 0.0;
                decimals.iter().for_each(|x| sum_f64 += x);

                // no constant required if sum is 0
                let constant_is_zero: bool = sum_f64 + sum_i64 as f64 == 0.0;
                let constant: EquationComponentType = {
                    if sum_f64 == 0.0 {
                        EquationComponentType::Integer(sum_i64)
                    } else {
                        EquationComponentType::Decimal(sum_f64 + sum_i64 as f64)
                    }
                };

                // updating nodes with MulNode if there are many AddNode's over a variable
                // example: x + x -> 2 * x
                let mut variable_occurrence: HashMap<char, i64> = HashMap::new();

                for i in variables.iter() {
                    match variable_occurrence.get(&i) {
                        Some(n) => variable_occurrence.insert(*i, n + 1),
                        None => variable_occurrence.insert(*i, 1),
                    };
                }

                let mut variables_nodes: Vec<EquationComponentType> = Vec::new();

                for (i, k) in variable_occurrence.into_iter() {
                    if k > 1 {
                        variables_nodes.push(EquationComponentType::MulNode {
                            lhs: Box::new(EquationComponentType::VariableNode(i)),
                            rhs: Box::new(EquationComponentType::Integer(k)),
                        });
                    } else {
                        variables_nodes.push(EquationComponentType::VariableNode(i));
                    }
                }

                variables_nodes.extend(nodes);

                // collect common terms of Variable MulNodes and create unique MulNodes
                // example: (3 * x) + x -> (4 * x)
                // example: (3 * x) + (x * 5) -> (8 * x)

                let mut variable_occurrence: HashMap<char, EquationComponentType> = HashMap::new();

                variables_nodes.retain(|node_to_simplify| {
                    if let EquationComponentType::MulNode { lhs, rhs } = node_to_simplify {
                        if let EquationComponentType::VariableNode(v) = **lhs {
                            if let EquationComponentType::Integer(c) = **rhs {
                                // variable * integer
                                match variable_occurrence.remove(&v) {
                                    Some(x) => {
                                        if let EquationComponentType::Integer(o) = x {
                                            variable_occurrence
                                                .insert(v, EquationComponentType::Integer(o + c));
                                        }
                                        if let EquationComponentType::Decimal(o) = x {
                                            variable_occurrence.insert(
                                                v,
                                                EquationComponentType::Decimal(o + c as f64),
                                            );
                                        }
                                    }
                                    None => {
                                        variable_occurrence
                                            .insert(v, EquationComponentType::Integer(c));
                                    }
                                };
                                return false;
                            } else if let EquationComponentType::Decimal(c) = **rhs {
                                // variable * decimal
                                match variable_occurrence.remove(&v) {
                                    Some(x) => {
                                        if let EquationComponentType::Integer(o) = x {
                                            variable_occurrence.insert(
                                                v,
                                                EquationComponentType::Decimal(o as f64 + c),
                                            );
                                        }
                                        if let EquationComponentType::Decimal(o) = x {
                                            variable_occurrence
                                                .insert(v, EquationComponentType::Decimal(o + c));
                                        }
                                    }
                                    None => {
                                        variable_occurrence
                                            .insert(v, EquationComponentType::Decimal(c));
                                    }
                                };
                                return false;
                            }
                        } else if let EquationComponentType::VariableNode(v) = **rhs {
                            if let EquationComponentType::Integer(c) = **lhs {
                                // integer * variable
                                match variable_occurrence.remove(&v) {
                                    Some(x) => {
                                        if let EquationComponentType::Integer(o) = x {
                                            variable_occurrence
                                                .insert(v, EquationComponentType::Integer(o + c));
                                        }
                                        if let EquationComponentType::Decimal(o) = x {
                                            variable_occurrence.insert(
                                                v,
                                                EquationComponentType::Decimal(o + c as f64),
                                            );
                                        }
                                    }
                                    None => {
                                        variable_occurrence
                                            .insert(v, EquationComponentType::Integer(c));
                                    }
                                };
                                return false;
                            } else if let EquationComponentType::Decimal(c) = **lhs {
                                // decimal * variable
                                match variable_occurrence.remove(&v) {
                                    Some(x) => {
                                        if let EquationComponentType::Integer(o) = x {
                                            variable_occurrence.insert(
                                                v,
                                                EquationComponentType::Decimal(o as f64 + c),
                                            );
                                        }
                                        if let EquationComponentType::Decimal(o) = x {
                                            variable_occurrence
                                                .insert(v, EquationComponentType::Decimal(o + c));
                                        }
                                    }
                                    None => {
                                        variable_occurrence
                                            .insert(v, EquationComponentType::Decimal(c));
                                    }
                                };
                                return false;
                            }
                        }
                    }

                    if let EquationComponentType::VariableNode(v) = node_to_simplify {
                        match variable_occurrence.remove(&v) {
                            Some(x) => {
                                if let EquationComponentType::Integer(o) = x {
                                    variable_occurrence
                                        .insert(*v, EquationComponentType::Integer(o + 1));
                                }
                                if let EquationComponentType::Decimal(o) = x {
                                    variable_occurrence
                                        .insert(*v, EquationComponentType::Decimal(o + 1.0));
                                }
                            }
                            None => {
                                variable_occurrence.insert(*v, EquationComponentType::Integer(1));
                            }
                        };
                        return false;
                    }
                    return true;
                });

                for (k, v) in variable_occurrence.into_iter() {
                    if let EquationComponentType::Integer(o) = v {
                        if o != 1 {
                            variables_nodes.push(EquationComponentType::MulNode {
                                lhs: Box::new(EquationComponentType::VariableNode(k)),
                                rhs: Box::new(v),
                            });
                        } else {
                            variables_nodes.push(EquationComponentType::VariableNode(k));
                        }
                    } else if let EquationComponentType::Decimal(o) = v {
                        if o != 1.0 {
                            variables_nodes.push(EquationComponentType::MulNode {
                                lhs: Box::new(EquationComponentType::VariableNode(k)),
                                rhs: Box::new(v),
                            });
                        } else {
                            variables_nodes.push(EquationComponentType::VariableNode(k));
                        }
                    }
                }

                // ? Should the following simplification be implemented:
                // ? 5 * (x + y) -> (5 * x) + (5 * y)

                // creating new AddNode with all the computed and simplified nodes
                if variables_nodes.len() == 0 {
                    return constant;
                }

                if variables_nodes.len() == 1 {
                    if constant_is_zero {
                        return variables_nodes.pop().unwrap().simplify();
                    }

                    return EquationComponentType::AddNode {
                        lhs: Box::new(constant),
                        rhs: Box::new(variables_nodes.pop().unwrap().simplify()),
                    };
                }

                let mut base_node: EquationComponentType = EquationComponentType::AddNode {
                    lhs: Box::new(variables_nodes.pop().unwrap().simplify()),
                    rhs: Box::new(variables_nodes.pop().unwrap().simplify()),
                };

                loop {
                    match variables_nodes.pop() {
                        Some(i) => {
                            base_node = EquationComponentType::AddNode {
                                lhs: Box::new(i.simplify()),
                                rhs: Box::new(base_node),
                            };
                        }
                        None => break,
                    }
                }

                if constant_is_zero {
                    return base_node;
                }
                return EquationComponentType::AddNode {
                    lhs: Box::new(constant),
                    rhs: Box::new(base_node),
                };
            } // End EquationComponentType::AddNode

            EquationComponentType::SubNode { lhs, rhs } => {
                let lhs: EquationComponentType = lhs.simplify();
                let rhs: EquationComponentType = rhs.simplify();

                return EquationComponentType::AddNode {
                    lhs: Box::new(lhs),
                    rhs: Box::new(EquationComponentType::MinusNode(Box::new(rhs)).simplify()),
                }
                .simplify();
            } // End EquationComponentType::SubNode

            EquationComponentType::MulNode { lhs: _, rhs: _ } => {
                // extracting simplified child nodes
                let mut variables: Vec<char> = Vec::new();
                let mut integers: Vec<i64> = Vec::new();
                let mut decimals: Vec<f64> = Vec::new();
                let mut nodes: Vec<EquationComponentType> = Vec::new();

                self.extract(&mut variables, &mut integers, &mut decimals, &mut nodes);

                // calculating the constant's value
                let mut product_i64: i64 = 1;
                integers.iter().for_each(|x| product_i64 *= x);

                let mut product_f64: f64 = 1.0;
                decimals.iter().for_each(|x| product_f64 *= x);

                // no constant required if product is 1
                let constant_is_one: bool = product_f64 * product_i64 as f64 == 1.0;
                let constant: EquationComponentType = {
                    if product_f64 == 1.0 {
                        EquationComponentType::Integer(product_i64)
                    } else {
                        EquationComponentType::Decimal(product_f64 * product_i64 as f64)
                    }
                };

                // updating node with PowNode of there are many MulNode's over a variable
                // example: x * x -> x ^ 2
                let mut variable_occurrence: HashMap<char, i64> = HashMap::new();

                for i in variables.iter() {
                    match variable_occurrence.get(&i) {
                        Some(n) => variable_occurrence.insert(*i, n + 1),
                        None => variable_occurrence.insert(*i, 1),
                    };
                }

                let mut variables_nodes: Vec<EquationComponentType> = Vec::new();

                for (i, k) in variable_occurrence.into_iter() {
                    if k > 1 {
                        variables_nodes.push(EquationComponentType::PowNode {
                            base: Box::new(EquationComponentType::VariableNode(i)),
                            exponent: Box::new(EquationComponentType::Integer(k)),
                        });
                    } else {
                        variables_nodes.push(EquationComponentType::VariableNode(i));
                    }
                }

                variables_nodes.extend(nodes);

                // collect common terms of Variable MulNodes and create unique PowNodes
                // example: (x ^ 2) * (x ^ 5) -> (x ^ 7)

                let mut variable_occurrence: HashMap<char, EquationComponentType> = HashMap::new();

                variables_nodes.retain(|node_to_simplify| {
                    if let EquationComponentType::PowNode { base, exponent } = node_to_simplify {
                        if let EquationComponentType::VariableNode(v) = **base {
                            if let EquationComponentType::Integer(c) = **exponent {
                                // variable * integer
                                match variable_occurrence.remove(&v) {
                                    Some(x) => {
                                        if let EquationComponentType::Integer(o) = x {
                                            variable_occurrence
                                                .insert(v, EquationComponentType::Integer(o + c));
                                        }
                                        if let EquationComponentType::Decimal(o) = x {
                                            variable_occurrence.insert(
                                                v,
                                                EquationComponentType::Decimal(o + c as f64),
                                            );
                                        }
                                    }
                                    None => {
                                        variable_occurrence
                                            .insert(v, EquationComponentType::Integer(c));
                                    }
                                };
                                return false;
                            } else if let EquationComponentType::Decimal(c) = **exponent {
                                // variable * decimal
                                match variable_occurrence.remove(&v) {
                                    Some(x) => {
                                        if let EquationComponentType::Integer(o) = x {
                                            variable_occurrence.insert(
                                                v,
                                                EquationComponentType::Decimal(o as f64 + c),
                                            );
                                        }
                                        if let EquationComponentType::Decimal(o) = x {
                                            variable_occurrence
                                                .insert(v, EquationComponentType::Decimal(o + c));
                                        }
                                    }
                                    None => {
                                        variable_occurrence
                                            .insert(v, EquationComponentType::Decimal(c));
                                    }
                                };
                                return false;
                            }
                        }
                    }

                    if let EquationComponentType::VariableNode(v) = node_to_simplify {
                        match variable_occurrence.remove(&v) {
                            Some(x) => {
                                if let EquationComponentType::Integer(o) = x {
                                    variable_occurrence
                                        .insert(*v, EquationComponentType::Integer(o + 1));
                                }
                                if let EquationComponentType::Decimal(o) = x {
                                    variable_occurrence
                                        .insert(*v, EquationComponentType::Decimal(o + 1.0));
                                }
                            }
                            None => {
                                variable_occurrence.insert(*v, EquationComponentType::Integer(1));
                            }
                        };
                        return false;
                    }
                    return true;
                });

                for (k, v) in variable_occurrence.into_iter() {
                    if let EquationComponentType::Integer(o) = v {
                        if o != 1 {
                            variables_nodes.push(EquationComponentType::PowNode {
                                base: Box::new(EquationComponentType::VariableNode(k)),
                                exponent: Box::new(v),
                            });
                        } else {
                            variables_nodes.push(EquationComponentType::VariableNode(k));
                        }
                    } else if let EquationComponentType::Decimal(o) = v {
                        if o != 1.0 {
                            variables_nodes.push(EquationComponentType::PowNode {
                                base: Box::new(EquationComponentType::VariableNode(k)),
                                exponent: Box::new(v),
                            });
                        } else {
                            variables_nodes.push(EquationComponentType::VariableNode(k));
                        }
                    }
                }

                // creating new MulNode with all the computed and simplified nodes
                if variables_nodes.len() == 0 {
                    return constant;
                }

                if variables_nodes.len() == 1 {
                    if constant_is_one {
                        return variables_nodes.pop().unwrap().simplify();
                    }
                    return EquationComponentType::MulNode {
                        lhs: Box::new(constant),
                        rhs: Box::new(variables_nodes.pop().unwrap().simplify()),
                    };
                }

                let mut base_node: EquationComponentType = EquationComponentType::MulNode {
                    lhs: Box::new(variables_nodes.pop().unwrap().simplify()),
                    rhs: Box::new(variables_nodes.pop().unwrap().simplify()),
                };

                loop {
                    match variables_nodes.pop() {
                        Some(i) => {
                            base_node = EquationComponentType::MulNode {
                                lhs: Box::new(i.simplify()),
                                rhs: Box::new(base_node),
                            };
                        }
                        None => break,
                    }
                }

                if constant_is_one {
                    return base_node;
                }
                return EquationComponentType::MulNode {
                    lhs: Box::new(constant),
                    rhs: Box::new(base_node),
                };
            } // End EquationComponentType::MulNod

            EquationComponentType::DivNode {
                numerator,
                denominator,
            } => {
                let numerator: EquationComponentType = numerator.simplify();
                let denominator: EquationComponentType = denominator.simplify();

                if let EquationComponentType::Integer(i) = numerator {
                    if let EquationComponentType::Integer(j) = denominator {
                        let result: f64 = i as f64 / j as f64;
                        return EquationComponentType::Decimal(result);
                    } else if let EquationComponentType::Decimal(j) = denominator {
                        let result: f64 = i as f64 / j;
                        return EquationComponentType::Decimal(result);
                    } else {
                        return EquationComponentType::DivNode {
                            numerator: Box::new(EquationComponentType::Integer(i)),
                            denominator: Box::new(denominator),
                        };
                    }
                } else if let EquationComponentType::Decimal(i) = numerator {
                    if let EquationComponentType::Integer(j) = denominator {
                        let result: f64 = i / j as f64;
                        return EquationComponentType::Decimal(result);
                    } else if let EquationComponentType::Decimal(j) = denominator {
                        let result: f64 = i / j;
                        return EquationComponentType::Decimal(result);
                    } else {
                        return EquationComponentType::DivNode {
                            numerator: Box::new(EquationComponentType::Decimal(i)),
                            denominator: Box::new(denominator),
                        };
                    }
                } else {
                    return EquationComponentType::DivNode {
                        numerator: Box::new(numerator),
                        denominator: Box::new(denominator),
                    };
                }
            } // End EquationComponentType::DivNode

            EquationComponentType::PowNode { base, exponent } => {
                let base: EquationComponentType = base.simplify();
                let exponent: EquationComponentType = exponent.simplify();

                // ((x ^ y) ^ z) -> x ^ (z * y)
                if let EquationComponentType::PowNode {
                    base: lvalue,
                    exponent: rvalue,
                } = base
                {
                    return EquationComponentType::PowNode {
                        base: lvalue,
                        exponent: Box::new(EquationComponentType::MulNode {
                            lhs: rvalue,
                            rhs: Box::new(exponent),
                        }),
                    };
                } else if let EquationComponentType::Integer(i) = base {
                    if let EquationComponentType::Integer(j) = exponent {
                        let result: i64 = math::powi64(i, j);
                        return EquationComponentType::Integer(result);
                    } else if let EquationComponentType::Decimal(j) = exponent {
                        let result: f64 = math::powf64(i as f64, j);
                        return EquationComponentType::Decimal(result);
                    } else {
                        return EquationComponentType::PowNode {
                            base: Box::new(EquationComponentType::Integer(i)),
                            exponent: Box::new(exponent),
                        };
                    }
                } else if let EquationComponentType::Decimal(i) = base {
                    if let EquationComponentType::Integer(j) = exponent {
                        let result: f64 = math::powf64(i, j as f64);
                        return EquationComponentType::Decimal(result);
                    } else if let EquationComponentType::Decimal(j) = exponent {
                        let result: f64 = math::powf64(i, j);
                        return EquationComponentType::Decimal(result);
                    } else {
                        return EquationComponentType::PowNode {
                            base: Box::new(EquationComponentType::Decimal(i)),
                            exponent: Box::new(exponent),
                        };
                    }
                } else {
                    return EquationComponentType::PowNode {
                        base: Box::new(base),
                        exponent: Box::new(exponent),
                    };
                }
            } // End EquationComponentType::PowNode

            EquationComponentType::LogNode { base, argument } => EquationComponentType::LogNode {
                base: Box::new(base.simplify()),
                argument: Box::new(argument.simplify()),
            }, // End EquationComponentType::LogNode

            EquationComponentType::MinusNode(value) => {
                let value: EquationComponentType = value.simplify();

                if let EquationComponentType::Integer(i) = value {
                    return EquationComponentType::Integer(-i);
                } else if let EquationComponentType::Decimal(i) = value {
                    return EquationComponentType::Decimal(-i);
                } else if let EquationComponentType::MinusNode(i) = value {
                    // -(-x) -> x
                    return *i;
                } else {
                    return EquationComponentType::MinusNode(Box::new(value));
                }
            }
        }
    }

    // TODO: implement substitute for PartEquation

    fn substitutei(&self, variable: char, value: i64) -> Self {
        match self {
            EquationComponentType::Integer(i) => EquationComponentType::Integer(*i),
            EquationComponentType::Decimal(i) => EquationComponentType::Decimal(*i),
            EquationComponentType::VariableNode(i) => {
                if *i == variable {
                    return EquationComponentType::Integer(value);
                }
                return EquationComponentType::VariableNode(*i);
            }
            EquationComponentType::AddNode { lhs, rhs } => EquationComponentType::AddNode {
                lhs: Box::new(lhs.substitutei(variable, value)),
                rhs: Box::new(rhs.substitutei(variable, value)),
            },
            EquationComponentType::SubNode { lhs, rhs } => EquationComponentType::SubNode {
                lhs: Box::new(lhs.substitutei(variable, value)),
                rhs: Box::new(rhs.substitutei(variable, value)),
            },
            EquationComponentType::MulNode { lhs, rhs } => EquationComponentType::MulNode {
                lhs: Box::new(lhs.substitutei(variable, value)),
                rhs: Box::new(rhs.substitutei(variable, value)),
            },
            EquationComponentType::DivNode {
                numerator,
                denominator,
            } => EquationComponentType::DivNode {
                numerator: Box::new(numerator.substitutei(variable, value)),
                denominator: Box::new(denominator.substitutei(variable, value)),
            },
            EquationComponentType::PowNode { base, exponent } => EquationComponentType::PowNode {
                base: Box::new(base.substitutei(variable, value)),
                exponent: Box::new(exponent.substitutei(variable, value)),
            },
            EquationComponentType::LogNode { base, argument } => EquationComponentType::LogNode {
                base: Box::new(base.substitutei(variable, value)),
                argument: Box::new(argument.substitutei(variable, value)),
            },
            EquationComponentType::MinusNode(node) => {
                EquationComponentType::MinusNode(Box::new(node.substitutei(variable, value)))
            }
        }
    }

    fn substitutef(&self, variable: char, value: f64) -> Self {
        match self {
            EquationComponentType::Integer(i) => EquationComponentType::Integer(*i),
            EquationComponentType::Decimal(i) => EquationComponentType::Decimal(*i),
            EquationComponentType::VariableNode(i) => {
                if *i == variable {
                    return EquationComponentType::Decimal(value);
                }
                return EquationComponentType::VariableNode(*i);
            }
            EquationComponentType::AddNode { lhs, rhs } => EquationComponentType::AddNode {
                lhs: Box::new(lhs.substitutef(variable, value)),
                rhs: Box::new(rhs.substitutef(variable, value)),
            },
            EquationComponentType::SubNode { lhs, rhs } => EquationComponentType::SubNode {
                lhs: Box::new(lhs.substitutef(variable, value)),
                rhs: Box::new(rhs.substitutef(variable, value)),
            },
            EquationComponentType::MulNode { lhs, rhs } => EquationComponentType::MulNode {
                lhs: Box::new(lhs.substitutef(variable, value)),
                rhs: Box::new(rhs.substitutef(variable, value)),
            },
            EquationComponentType::DivNode {
                numerator,
                denominator,
            } => EquationComponentType::DivNode {
                numerator: Box::new(numerator.substitutef(variable, value)),
                denominator: Box::new(denominator.substitutef(variable, value)),
            },
            EquationComponentType::PowNode { base, exponent } => EquationComponentType::PowNode {
                base: Box::new(base.substitutef(variable, value)),
                exponent: Box::new(exponent.substitutef(variable, value)),
            },
            EquationComponentType::LogNode { base, argument } => EquationComponentType::LogNode {
                base: Box::new(base.substitutef(variable, value)),
                argument: Box::new(argument.substitutef(variable, value)),
            },
            EquationComponentType::MinusNode(node) => {
                EquationComponentType::MinusNode(Box::new(node.substitutef(variable, value)))
            }
        }
    }

    fn extract(
        &self,
        variables: &mut Vec<char>,
        integers: &mut Vec<i64>,
        decimals: &mut Vec<f64>,
        nodes: &mut Vec<EquationComponentType>,
    ) {
        match self {
            EquationComponentType::AddNode { lhs, rhs } => {
                match &**lhs {
                    EquationComponentType::Integer(i) => integers.push(*i),
                    EquationComponentType::Decimal(i) => decimals.push(*i),
                    EquationComponentType::VariableNode(i) => variables.push(*i),
                    i @ EquationComponentType::AddNode { lhs: _, rhs: _ } => {
                        i.extract(variables, integers, decimals, nodes)
                    }
                    n => {
                        let m = n.simplify();
                        match m {
                            EquationComponentType::Integer(i) => integers.push(i),
                            EquationComponentType::Decimal(i) => decimals.push(i),
                            EquationComponentType::VariableNode(i) => variables.push(i),
                            i @ EquationComponentType::AddNode { lhs: _, rhs: _ } => {
                                i.extract(variables, integers, decimals, nodes)
                            }
                            n => nodes.push(n),
                        }
                    }
                };

                match &**rhs {
                    EquationComponentType::Integer(i) => integers.push(*i),
                    EquationComponentType::Decimal(i) => decimals.push(*i),
                    EquationComponentType::VariableNode(i) => variables.push(*i),
                    i @ EquationComponentType::AddNode { lhs: _, rhs: _ } => {
                        i.extract(variables, integers, decimals, nodes)
                    }
                    n => {
                        let m = n.simplify();
                        match m {
                            EquationComponentType::Integer(i) => integers.push(i),
                            EquationComponentType::Decimal(i) => decimals.push(i),
                            EquationComponentType::VariableNode(i) => variables.push(i),
                            i @ EquationComponentType::AddNode { lhs: _, rhs: _ } => {
                                i.extract(variables, integers, decimals, nodes)
                            }
                            n => nodes.push(n),
                        }
                    }
                };
            } // End EquationComponentType::AddNode

            EquationComponentType::MulNode { lhs, rhs } => {
                match &**lhs {
                    EquationComponentType::Integer(i) => integers.push(*i),
                    EquationComponentType::Decimal(i) => decimals.push(*i),
                    EquationComponentType::VariableNode(i) => variables.push(*i),
                    i @ EquationComponentType::MulNode { lhs: _, rhs: _ } => {
                        i.extract(variables, integers, decimals, nodes)
                    }
                    n => {
                        let m = n.simplify();

                        match m {
                            EquationComponentType::Integer(i) => integers.push(i),
                            EquationComponentType::Decimal(i) => decimals.push(i),
                            EquationComponentType::VariableNode(i) => variables.push(i),
                            i @ EquationComponentType::MulNode { lhs: _, rhs: _ } => {
                                i.extract(variables, integers, decimals, nodes)
                            }
                            n => nodes.push(n),
                        }
                    }
                };

                match &**rhs {
                    EquationComponentType::Integer(i) => integers.push(*i),
                    EquationComponentType::Decimal(i) => decimals.push(*i),
                    EquationComponentType::VariableNode(i) => variables.push(*i),
                    i @ EquationComponentType::MulNode { lhs: _, rhs: _ } => {
                        i.extract(variables, integers, decimals, nodes)
                    }
                    n => {
                        let m = n.simplify();

                        match m {
                            EquationComponentType::Integer(i) => integers.push(i),
                            EquationComponentType::Decimal(i) => decimals.push(i),
                            EquationComponentType::VariableNode(i) => variables.push(i),
                            i @ EquationComponentType::MulNode { lhs: _, rhs: _ } => {
                                i.extract(variables, integers, decimals, nodes)
                            }
                            n => nodes.push(n),
                        }
                    }
                };
            } // End EquationComponentType::MulNode
            _ => return,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartEquation {
    eq: EquationComponentType,
}

impl PartEquation {
    pub fn substitutei(&self, variable: char, value: i64) -> PartEquation {
        PartEquation {
            eq: self.eq.substitutei(variable, value).simplify(),
        }
    }

    pub fn substitutef(&self, variable: char, value: f64) -> PartEquation {
        PartEquation {
            eq: self.eq.substitutef(variable, value), /*.simplify()*/
        }
    }

    pub fn new(variable: char) -> Self {
        PartEquation {
            eq: EquationComponentType::VariableNode(variable),
        }
    }

    pub fn newi(value: i64) -> Self {
        PartEquation {
            eq: EquationComponentType::Integer(value),
        }
    }

    pub fn newf(value: f64) -> Self {
        PartEquation {
            eq: EquationComponentType::Decimal(value),
        }
    }

    pub fn simplify(&self) -> Self {
        PartEquation {
            eq: self.eq.simplify().post_simplify(),
        }
    }

    pub fn pow(&self, exponent: &PartEquation) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode {
                base: Box::new(self.eq.clone()),
                exponent: Box::new(exponent.eq.clone()),
            },
        }
    }

    pub fn powi(&self, exponent: i64) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode {
                base: Box::new(self.eq.clone()),
                exponent: Box::new(EquationComponentType::Integer(exponent)),
            },
        }
    }

    pub fn powf(&self, exponent: f64) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode {
                base: Box::new(self.eq.clone()),
                exponent: Box::new(EquationComponentType::Decimal(exponent)),
            },
        }
    }
}

impl Display for PartEquation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.eq)
    }
}

#[derive(Debug, Clone)]
pub struct Equation {
    lhs: EquationComponentType,
    rhs: EquationComponentType,
}

enum AntiOperations {
    AddLHS,
    AddRHS,
    SubLHS,
    SubRHS,
    MulNumerator,
    MulDenominator,
    DivLHS,
    DivRHS,
    PowLHS,
    PowRHS,
    LogLHS,
    LogRHS,
    Minus,
}

impl Equation {
    pub fn new(lhs: &PartEquation, rhs: &PartEquation) -> Self {
        Equation {
            lhs: lhs.eq.clone(),
            rhs: rhs.eq.clone(),
        }
    }

    pub fn solve(&self, variable: char) -> Result<PartEquation, MathError> {
        let eq: EquationComponentType = EquationComponentType::AddNode {
            lhs: Box::new(self.lhs.simplify()),
            rhs: Box::new(EquationComponentType::MinusNode(Box::new(
                self.rhs.simplify(),
            ))),
        }
        .simplify();

        if Self::count_occurrences(&eq, variable) > 1 {
            todo!()
        } else if Self::count_occurrences(&eq, variable) == 0 {
            return Err(MathError::EquationMismatchError);
        }

        match Self::do_inverse(&eq, variable) {
            Ok(result) => Ok(PartEquation {
                eq: result.simplify(),
            }),
            Err(err) => Err(err),
        }
    }

    fn count_occurrences(eq: &EquationComponentType, variable: char) -> i64 {
        let mut occurrences = 0;

        match eq {
            EquationComponentType::VariableNode(i) => {
                if *i == variable {
                    occurrences += 1;
                }
            }
            EquationComponentType::AddNode { lhs, rhs } => {
                occurrences += Self::count_occurrences(lhs, variable);
                occurrences += Self::count_occurrences(rhs, variable);
            }
            EquationComponentType::SubNode { lhs, rhs } => {
                occurrences += Self::count_occurrences(lhs, variable);
                occurrences += Self::count_occurrences(rhs, variable);
            }
            EquationComponentType::MulNode { lhs, rhs } => {
                occurrences += Self::count_occurrences(lhs, variable);
                occurrences += Self::count_occurrences(rhs, variable);
            }
            EquationComponentType::DivNode {
                numerator,
                denominator,
            } => {
                occurrences += Self::count_occurrences(numerator, variable);
                occurrences += Self::count_occurrences(denominator, variable);
            }
            EquationComponentType::PowNode { base, exponent } => {
                occurrences += Self::count_occurrences(base, variable);
                occurrences += Self::count_occurrences(exponent, variable);
            }
            EquationComponentType::LogNode { base, argument } => {
                occurrences += Self::count_occurrences(base, variable);
                occurrences += Self::count_occurrences(argument, variable);
            }
            EquationComponentType::MinusNode(value) => {
                occurrences += Self::count_occurrences(value, variable);
            }
            _ => {}
        }

        return occurrences;
    }

    fn make_anti_operations_list(
        eq: &EquationComponentType,
        variable: char,
        list: &mut Vec<AntiOperations>,
    ) -> bool {
        match eq {
            EquationComponentType::VariableNode(i) => {
                if *i == variable {
                    true
                } else {
                    false
                }
            }
            EquationComponentType::AddNode { lhs, rhs } => {
                if Self::make_anti_operations_list(lhs, variable, list) {
                    list.push(AntiOperations::SubRHS);
                    true
                } else if Self::make_anti_operations_list(rhs, variable, list) {
                    list.push(AntiOperations::SubLHS);
                    true
                } else {
                    false
                }
            }
            EquationComponentType::SubNode { lhs, rhs } => {
                if Self::make_anti_operations_list(lhs, variable, list) {
                    list.push(AntiOperations::AddRHS);
                    true
                } else if Self::make_anti_operations_list(rhs, variable, list) {
                    list.push(AntiOperations::AddLHS);
                    true
                } else {
                    false
                }
            }
            EquationComponentType::MulNode { lhs, rhs } => {
                if Self::make_anti_operations_list(lhs, variable, list) {
                    list.push(AntiOperations::DivRHS);
                    true
                } else if Self::make_anti_operations_list(rhs, variable, list) {
                    list.push(AntiOperations::DivLHS);
                    true
                } else {
                    false
                }
            }
            EquationComponentType::DivNode {
                numerator,
                denominator,
            } => {
                if Self::make_anti_operations_list(numerator, variable, list) {
                    list.push(AntiOperations::MulDenominator);
                    true
                } else if Self::make_anti_operations_list(denominator, variable, list) {
                    list.push(AntiOperations::MulNumerator);
                    true
                } else {
                    false
                }
            }
            EquationComponentType::PowNode { base, exponent } => {
                if Self::make_anti_operations_list(base, variable, list) {
                    list.push(AntiOperations::PowRHS);
                    true
                } else if Self::make_anti_operations_list(exponent, variable, list) {
                    list.push(AntiOperations::LogLHS);
                    true
                } else {
                    false
                }
            }
            EquationComponentType::LogNode { base, argument } => {
                if Self::make_anti_operations_list(base, variable, list) {
                    list.push(AntiOperations::LogRHS);
                    true
                } else if Self::make_anti_operations_list(argument, variable, list) {
                    list.push(AntiOperations::PowLHS);
                    true
                } else {
                    false
                }
            }
            EquationComponentType::MinusNode(value) => {
                if Self::make_anti_operations_list(value, variable, list) {
                    list.push(AntiOperations::Minus);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn do_inverse(
        eq: &EquationComponentType,
        variable: char,
    ) -> Result<EquationComponentType, MathError> {
        // Step 1: make a list of anti operations to perform
        let mut anti_ops: Vec<AntiOperations> = Vec::new();
        Self::make_anti_operations_list(&eq, variable, &mut anti_ops);

        let mut result: EquationComponentType = EquationComponentType::Integer(0);
        let mut eq: EquationComponentType = eq.clone();

        // Step 2: perform the anti operations`
        for _ in 0..anti_ops.len() {
            match anti_ops.pop().unwrap() {
                AntiOperations::AddLHS => {
                    if let EquationComponentType::SubNode { lhs, rhs } = eq {
                        eq = *rhs;
                        result = EquationComponentType::AddNode {
                            lhs: Box::new(result),
                            rhs: Box::new(EquationComponentType::MinusNode(lhs)),
                        }
                    } else {
                        return Err(MathError::InternalError);
                    }
                }
                AntiOperations::AddRHS => {
                    if let EquationComponentType::SubNode { lhs, rhs } = eq {
                        eq = *lhs;
                        result = EquationComponentType::AddNode {
                            lhs: Box::new(result),
                            rhs: rhs,
                        }
                    } else {
                        return Err(MathError::InternalError);
                    }
                }
                AntiOperations::SubLHS => {
                    if let EquationComponentType::AddNode { lhs, rhs } = eq {
                        eq = *rhs;
                        result = EquationComponentType::SubNode {
                            lhs: Box::new(result),
                            rhs: lhs,
                        }
                    } else {
                        return Err(MathError::InternalError);
                    }
                }
                AntiOperations::SubRHS => {
                    if let EquationComponentType::AddNode { lhs, rhs } = eq {
                        eq = *lhs;
                        result = EquationComponentType::SubNode {
                            lhs: Box::new(result),
                            rhs: rhs,
                        }
                    } else {
                        return Err(MathError::InternalError);
                    }
                }
                AntiOperations::MulNumerator => {
                    if let EquationComponentType::DivNode {
                        numerator,
                        denominator,
                    } = eq
                    {
                        eq = *denominator;
                        result = EquationComponentType::DivNode {
                            numerator: numerator,
                            denominator: Box::new(result),
                        }
                    } else {
                        return Err(MathError::InternalError);
                    }
                }
                AntiOperations::MulDenominator => {
                    if let EquationComponentType::DivNode {
                        numerator,
                        denominator,
                    } = eq
                    {
                        eq = *numerator;
                        result = EquationComponentType::MulNode {
                            lhs: Box::new(result),
                            rhs: denominator,
                        }
                    } else {
                        return Err(MathError::InternalError);
                    }
                }
                AntiOperations::DivLHS => {
                    if let EquationComponentType::MulNode { lhs, rhs } = eq {
                        eq = *rhs;
                        result = EquationComponentType::DivNode {
                            numerator: Box::new(result),
                            denominator: lhs,
                        }
                    } else {
                        return Err(MathError::InternalError);
                    }
                }
                AntiOperations::DivRHS => {
                    if let EquationComponentType::MulNode { lhs, rhs } = eq {
                        eq = *lhs;
                        result = EquationComponentType::DivNode {
                            numerator: Box::new(result),
                            denominator: rhs,
                        }
                    } else {
                        return Err(MathError::InternalError);
                    }
                }
                AntiOperations::PowLHS => {
                    if let EquationComponentType::LogNode { base, argument } = eq {
                        eq = *argument;
                        result = EquationComponentType::PowNode {
                            base: base,
                            exponent: Box::new(result),
                        }
                    } else {
                        return Err(MathError::InternalError);
                    }
                }
                AntiOperations::PowRHS => {
                    if let EquationComponentType::PowNode { base, exponent } = eq {
                        eq = *base;
                        result = EquationComponentType::PowNode {
                            base: Box::new(result),
                            exponent: Box::new(EquationComponentType::DivNode {
                                numerator: Box::new(EquationComponentType::Integer(1)),
                                denominator: exponent,
                            }),
                        }
                    } else {
                        return Err(MathError::InternalError);
                    }
                }
                AntiOperations::LogLHS => {
                    if let EquationComponentType::PowNode { base, exponent } = eq {
                        eq = *exponent;
                        result = EquationComponentType::LogNode {
                            base: base,
                            argument: Box::new(result),
                        }
                    } else {
                        return Err(MathError::InternalError);
                    }
                }
                AntiOperations::LogRHS => {
                    if let EquationComponentType::PowNode { base, exponent } = eq {
                        eq = *base;
                        result = EquationComponentType::PowNode {
                            base: exponent,
                            exponent: Box::new(EquationComponentType::DivNode {
                                numerator: Box::new(EquationComponentType::Integer(1)),
                                denominator: Box::new(result),
                            }),
                        }
                    } else {
                        return Err(MathError::InternalError);
                    }
                }
                AntiOperations::Minus => {
                    if let EquationComponentType::MinusNode(v) = eq {
                        eq = *v;
                        result = EquationComponentType::MinusNode(Box::new(result));
                    } else {
                        return Err(MathError::InternalError);
                    }
                }
            }
        }

        // Step 3: return the simplified answer
        return Ok(result.simplify());
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.lhs, self.rhs)
    }
}

impl ops::Add<PartEquation> for PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Add<&'a PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl<'a> ops::Add<PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Add<&'a PartEquation> for PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: &'a PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl ops::Add<i64> for PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(rhs)),
            },
        }
    }
}

impl ops::Add<f64> for PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Decimal(rhs)),
            },
        }
    }
}

impl ops::Add<PartEquation> for i64 {
    type Output = PartEquation;

    fn add(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(EquationComponentType::Integer(self)),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl ops::Add<PartEquation> for f64 {
    type Output = PartEquation;

    fn add(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(EquationComponentType::Decimal(self)),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Add<i64> for &'a PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(rhs)),
            },
        }
    }
}

impl<'a> ops::Add<f64> for &'a PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(rhs)),
            },
        }
    }
}

impl<'a> ops::Add<&'a PartEquation> for i64 {
    type Output = PartEquation;

    fn add(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(EquationComponentType::Integer(self)),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl<'a> ops::Add<&'a PartEquation> for f64 {
    type Output = PartEquation;

    fn add(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(EquationComponentType::Decimal(self)),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl ops::Sub<PartEquation> for PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Sub<&'a PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl<'a> ops::Sub<PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Sub<&'a PartEquation> for PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: &'a PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl ops::Sub<i64> for PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(rhs)),
            },
        }
    }
}

impl ops::Sub<f64> for PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Decimal(rhs)),
            },
        }
    }
}

impl ops::Sub<PartEquation> for i64 {
    type Output = PartEquation;

    fn sub(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(EquationComponentType::Integer(self)),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl ops::Sub<PartEquation> for f64 {
    type Output = PartEquation;

    fn sub(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(EquationComponentType::Decimal(self)),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Sub<i64> for &'a PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(rhs)),
            },
        }
    }
}

impl<'a> ops::Sub<f64> for &'a PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(rhs)),
            },
        }
    }
}

impl<'a> ops::Sub<&'a PartEquation> for i64 {
    type Output = PartEquation;

    fn sub(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(EquationComponentType::Integer(self)),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl<'a> ops::Sub<&'a PartEquation> for f64 {
    type Output = PartEquation;

    fn sub(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(EquationComponentType::Decimal(self)),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl ops::Mul<PartEquation> for PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Mul<&'a PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl<'a> ops::Mul<PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Mul<&'a PartEquation> for PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: &'a PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl ops::Mul<i64> for PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(rhs)),
            },
        }
    }
}

impl ops::Mul<f64> for PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Decimal(rhs)),
            },
        }
    }
}

impl ops::Mul<PartEquation> for i64 {
    type Output = PartEquation;

    fn mul(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(EquationComponentType::Integer(self)),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl ops::Mul<PartEquation> for f64 {
    type Output = PartEquation;

    fn mul(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(EquationComponentType::Decimal(self)),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Mul<i64> for &'a PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(rhs)),
            },
        }
    }
}

impl<'a> ops::Mul<f64> for &'a PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(rhs)),
            },
        }
    }
}

impl<'a> ops::Mul<&'a PartEquation> for i64 {
    type Output = PartEquation;

    fn mul(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(EquationComponentType::Integer(self)),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl<'a> ops::Mul<&'a PartEquation> for f64 {
    type Output = PartEquation;

    fn mul(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(EquationComponentType::Decimal(self)),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl ops::Div<PartEquation> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(self.eq),
                denominator: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(self.eq.clone()),
                denominator: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl<'a> ops::Div<PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(self.eq.clone()),
                denominator: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: &'a PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(self.eq),
                denominator: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl ops::Div<i64> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(self.eq),
                denominator: Box::new(EquationComponentType::Integer(rhs)),
            },
        }
    }
}

impl ops::Div<f64> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(self.eq),
                denominator: Box::new(EquationComponentType::Decimal(rhs)),
            },
        }
    }
}

impl ops::Div<PartEquation> for i64 {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(EquationComponentType::Integer(self)),
                denominator: Box::new(rhs.eq),
            },
        }
    }
}

impl ops::Div<PartEquation> for f64 {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(EquationComponentType::Decimal(self)),
                denominator: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Div<i64> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(self.eq.clone()),
                denominator: Box::new(EquationComponentType::Integer(rhs)),
            },
        }
    }
}

impl<'a> ops::Div<f64> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(self.eq.clone()),
                denominator: Box::new(EquationComponentType::Decimal(rhs)),
            },
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for i64 {
    type Output = PartEquation;

    fn div(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(EquationComponentType::Integer(self)),
                denominator: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for f64 {
    type Output = PartEquation;

    fn div(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(EquationComponentType::Decimal(self)),
                denominator: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl ops::Neg for PartEquation {
    type Output = PartEquation;

    fn neg(self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MinusNode(Box::new(self.eq)),
        }
    }
}

impl<'a> ops::Neg for &'a PartEquation {
    type Output = PartEquation;

    fn neg(self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MinusNode(Box::new(self.eq.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_equation() {
        let x: PartEquation = PartEquation::new('x');
        let eq: PartEquation = &x + 2;
        let eq_str: String = eq.to_string();
        assert_eq!(eq_str, String::from("(x + 2)"));
    }
}
