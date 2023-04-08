use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops;

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
        lhs: Box<EquationComponentType>,
        rhs: Box<EquationComponentType>,
    },
    PowNode {
        lhs: Box<EquationComponentType>, // base
        rhs: Box<EquationComponentType>, // exponent
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
            EquationComponentType::DivNode { lhs, rhs } => write!(f, "({:?} / {:?})", lhs, rhs),
            EquationComponentType::PowNode { lhs, rhs } => write!(f, "({:?} ^ {:?})", lhs, rhs),
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
            EquationComponentType::DivNode { lhs, rhs } => write!(f, "({} / {})", lhs, rhs),
            EquationComponentType::PowNode { lhs, rhs } => write!(f, "({} ^ {})", lhs, rhs),
            EquationComponentType::MinusNode(value) => write!(f, "-({})", value),
        }
    }
}

impl EquationComponentType {
    fn post_simplify(&self) -> EquationComponentType {
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
            EquationComponentType::DivNode { lhs, rhs } => EquationComponentType::DivNode {
                lhs: Box::new(lhs.post_simplify()),
                rhs: Box::new(rhs.post_simplify()),
            },
            EquationComponentType::PowNode { lhs, rhs } => EquationComponentType::PowNode {
                lhs: Box::new(lhs.post_simplify()),
                rhs: Box::new(rhs.post_simplify()),
            },
        }
    }

    fn simplify(&self) -> EquationComponentType {
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
                    variables_nodes.push(EquationComponentType::MulNode {
                        lhs: Box::new(EquationComponentType::VariableNode(k)),
                        rhs: Box::new(v),
                    })
                }

                // ? Should the following simplification be implemented:
                // ? 5 + (x * y) -> (5 * x) + (5 * y)

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
                            lhs: Box::new(EquationComponentType::VariableNode(i)),
                            rhs: Box::new(EquationComponentType::Integer(k)),
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
                    if let EquationComponentType::PowNode { lhs, rhs } = node_to_simplify {
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
                    variables_nodes.push(EquationComponentType::PowNode {
                        lhs: Box::new(EquationComponentType::VariableNode(k)),
                        rhs: Box::new(v),
                    })
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

            EquationComponentType::DivNode { lhs, rhs } => {
                let lhs: EquationComponentType = lhs.simplify();
                let rhs: EquationComponentType = rhs.simplify();

                if let EquationComponentType::Integer(i) = lhs {
                    if let EquationComponentType::Integer(j) = rhs {
                        let result: i64 = i / j;
                        return EquationComponentType::Integer(result);
                    } else if let EquationComponentType::Decimal(j) = rhs {
                        let result: f64 = i as f64 / j;
                        return EquationComponentType::Decimal(result);
                    } else {
                        return EquationComponentType::DivNode {
                            lhs: Box::new(EquationComponentType::Integer(i)),
                            rhs: Box::new(rhs),
                        };
                    }
                } else if let EquationComponentType::Decimal(i) = lhs {
                    if let EquationComponentType::Integer(j) = rhs {
                        let result: f64 = i / j as f64;
                        return EquationComponentType::Decimal(result);
                    } else if let EquationComponentType::Decimal(j) = rhs {
                        let result: f64 = i / j;
                        return EquationComponentType::Decimal(result);
                    } else {
                        return EquationComponentType::DivNode {
                            lhs: Box::new(EquationComponentType::Decimal(i)),
                            rhs: Box::new(rhs),
                        };
                    }
                } else {
                    return EquationComponentType::DivNode {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }
            } // End EquationComponentType::DivNode

            EquationComponentType::PowNode { lhs, rhs } => {
                let lhs: EquationComponentType = lhs.simplify();
                let rhs: EquationComponentType = rhs.simplify();

                // TODO: implement the following simplification
                // ((x ^ y) ^ z) -> x ^ (z * y)

                if let EquationComponentType::Integer(i) = lhs {
                    if let EquationComponentType::Integer(j) = rhs {
                        let result: i64 = math::powi64(i, j);
                        return EquationComponentType::Integer(result);
                    } else if let EquationComponentType::Decimal(j) = rhs {
                        let result: f64 = math::powf64(i as f64, j);
                        return EquationComponentType::Decimal(result);
                    } else {
                        return EquationComponentType::PowNode {
                            lhs: Box::new(EquationComponentType::Integer(i)),
                            rhs: Box::new(rhs),
                        };
                    }
                } else if let EquationComponentType::Decimal(i) = lhs {
                    if let EquationComponentType::Integer(j) = rhs {
                        let result: f64 = math::powf64(i, j as f64);
                        return EquationComponentType::Decimal(result);
                    } else if let EquationComponentType::Decimal(j) = rhs {
                        let result: f64 = math::powf64(i, j);
                        return EquationComponentType::Decimal(result);
                    } else {
                        return EquationComponentType::PowNode {
                            lhs: Box::new(EquationComponentType::Decimal(i)),
                            rhs: Box::new(rhs),
                        };
                    }
                } else {
                    return EquationComponentType::PowNode {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }
            } // End EquationComponentType::PowNode

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

    // TODO: implement substitutef and substitute for PartEquation

    fn substitutei(&self, variable: char, value: i64) -> EquationComponentType {
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
            EquationComponentType::SubNode { lhs, rhs } => EquationComponentType::AddNode {
                lhs: Box::new(lhs.substitutei(variable, value)),
                rhs: Box::new(rhs.substitutei(variable, value)),
            },
            EquationComponentType::MulNode { lhs, rhs } => EquationComponentType::AddNode {
                lhs: Box::new(lhs.substitutei(variable, value)),
                rhs: Box::new(rhs.substitutei(variable, value)),
            },
            EquationComponentType::DivNode { lhs, rhs } => EquationComponentType::AddNode {
                lhs: Box::new(lhs.substitutei(variable, value)),
                rhs: Box::new(rhs.substitutei(variable, value)),
            },
            EquationComponentType::PowNode { lhs, rhs } => EquationComponentType::AddNode {
                lhs: Box::new(lhs.substitutei(variable, value)),
                rhs: Box::new(rhs.substitutei(variable, value)),
            },
            EquationComponentType::MinusNode(node) => {
                EquationComponentType::MinusNode(Box::new(node.substitutei(variable, value)))
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
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(exponent.eq.clone()),
            },
        }
    }

    pub fn powi32(&self, exponent: i32) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(exponent as i64)),
            },
        }
    }

    pub fn powi64(&self, exponent: i64) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(exponent as i64)),
            },
        }
    }

    pub fn powi(&self, exponent: i64) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(exponent)),
            },
        }
    }

    pub fn powf32(&self, exponent: f32) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(exponent as f64)),
            },
        }
    }

    pub fn powf(&self, exponent: f64) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(exponent)),
            },
        }
    }
}

impl Display for PartEquation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.eq)
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
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl<'a> ops::Div<PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: &'a PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl ops::Div<i64> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(rhs)),
            },
        }
    }
}

impl ops::Div<f64> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Decimal(rhs)),
            },
        }
    }
}

impl ops::Div<PartEquation> for i64 {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                lhs: Box::new(EquationComponentType::Integer(self)),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl ops::Div<PartEquation> for f64 {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                lhs: Box::new(EquationComponentType::Decimal(self)),
                rhs: Box::new(rhs.eq),
            },
        }
    }
}

impl<'a> ops::Div<i64> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(rhs)),
            },
        }
    }
}

impl<'a> ops::Div<f64> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(rhs)),
            },
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for i64 {
    type Output = PartEquation;

    fn div(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                lhs: Box::new(EquationComponentType::Integer(self)),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for f64 {
    type Output = PartEquation;

    fn div(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                lhs: Box::new(EquationComponentType::Decimal(self)),
                rhs: Box::new(rhs.eq.clone()),
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
