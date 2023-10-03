use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops;

use super::number::Number;
use crate::math::MathError;

#[derive(Clone, PartialEq, Eq)]
enum EquationComponentType {
    ConstantNode(Number),
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
            EquationComponentType::ConstantNode(i) => write!(f, "{:?}", i),
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
            EquationComponentType::ConstantNode(i) => write!(f, "{}", i),
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
    fn simplify(&self) -> Self {
        match self {
            EquationComponentType::ConstantNode(i) => {
                EquationComponentType::ConstantNode(i.clone())
            }

            EquationComponentType::VariableNode(i) => EquationComponentType::VariableNode(*i),

            EquationComponentType::AddNode { lhs: _, rhs: _ } => {
                // TODO: implement the following simplification `log(x) + log(x) = log(2x)`

                // TODO: implement the following simplification `x^n + x^n = 2*x^n`
                //  where n can a function
                //  similarly f + f = 2*f for any function

                // extracting simplified child nodes
                let mut variables: Vec<char> = Vec::new();
                let mut constants: Vec<Number> = Vec::new();
                let mut nodes: Vec<EquationComponentType> = Vec::new();

                self.extract(&mut variables, &mut constants, &mut nodes);

                // calculating the constant's value
                let mut constant: Number = Number::from(0);
                constants.iter().for_each(|x| constant = &constant + x);

                // no constant required if sum is 0
                let constant_is_zero: bool = constant == Number::from(0);

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
                            rhs: Box::new(EquationComponentType::ConstantNode(Number::from(k))),
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
                            if let EquationComponentType::ConstantNode(c) = *(*rhs).clone() {
                                // variable * constant
                                match variable_occurrence.remove(&v) {
                                    Some(x) => {
                                        if let EquationComponentType::ConstantNode(o) = x {
                                            variable_occurrence.insert(
                                                v,
                                                EquationComponentType::ConstantNode(o + c),
                                            );
                                        }
                                    }
                                    None => {
                                        variable_occurrence
                                            .insert(v, EquationComponentType::ConstantNode(c));
                                    }
                                };
                                return false;
                            }
                        } else if let EquationComponentType::VariableNode(v) = **rhs {
                            if let EquationComponentType::ConstantNode(c) = *(*lhs).clone() {
                                // constant * variable
                                match variable_occurrence.remove(&v) {
                                    Some(x) => {
                                        if let EquationComponentType::ConstantNode(o) = x {
                                            variable_occurrence.insert(
                                                v,
                                                EquationComponentType::ConstantNode(o + c),
                                            );
                                        }
                                    }
                                    None => {
                                        variable_occurrence
                                            .insert(v, EquationComponentType::ConstantNode(c));
                                    }
                                };
                                return false;
                            }
                        }
                    }

                    if let EquationComponentType::VariableNode(v) = node_to_simplify {
                        match variable_occurrence.remove(&v) {
                            Some(x) => {
                                if let EquationComponentType::ConstantNode(o) = x {
                                    variable_occurrence
                                        .insert(*v, EquationComponentType::ConstantNode(o + 1));
                                }
                            }
                            None => {
                                variable_occurrence.insert(
                                    *v,
                                    EquationComponentType::ConstantNode(Number::from(1)),
                                );
                            }
                        };
                        return false;
                    }

                    if let EquationComponentType::MinusNode(n) = node_to_simplify {
                        if let EquationComponentType::VariableNode(v) = **n {
                            match variable_occurrence.remove(&v) {
                                Some(x) => {
                                    if let EquationComponentType::ConstantNode(o) = x {
                                        variable_occurrence
                                            .insert(v, EquationComponentType::ConstantNode(o - 1));
                                    }
                                }
                                None => {
                                    variable_occurrence.insert(
                                        v,
                                        EquationComponentType::ConstantNode(Number::from(-1)),
                                    );
                                }
                            }
                            return false;
                        }
                    }
                    return true;
                });

                for (k, v) in variable_occurrence.into_iter() {
                    if let EquationComponentType::ConstantNode(o) = v.clone() {
                        if o != Number::from(1) {
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
                    return EquationComponentType::ConstantNode(constant);
                }

                if variables_nodes.len() == 1 {
                    if constant_is_zero {
                        return variables_nodes.pop().unwrap().simplify();
                    }

                    return EquationComponentType::AddNode {
                        lhs: Box::new(EquationComponentType::ConstantNode(constant)),
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
                    lhs: Box::new(EquationComponentType::ConstantNode(constant)),
                    rhs: Box::new(base_node),
                };
            } // End EquationComponentType::AddNode

            EquationComponentType::SubNode { lhs, rhs } => {
                // TODO: implement the following simplifications `log(x) - log(y) = log(x/y)`

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
                let mut constants: Vec<Number> = Vec::new();
                let mut nodes: Vec<EquationComponentType> = Vec::new();

                self.extract(&mut variables, &mut constants, &mut nodes);

                // calculating the constant's value
                let mut constant = Number::from(1);
                constants.iter().for_each(|x| constant = &constant * x);

                // return 0, if constant is 0
                if constant == Number::from(0) {
                    return EquationComponentType::ConstantNode(Number::from(0));
                }

                // no constant required if product is 1
                let constant_is_one: bool = constant == Number::from(1);

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
                            exponent: Box::new(EquationComponentType::ConstantNode(Number::from(
                                k,
                            ))),
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
                            if let EquationComponentType::ConstantNode(c) = *(*exponent).clone() {
                                // variable * constant
                                match variable_occurrence.remove(&v) {
                                    Some(x) => {
                                        if let EquationComponentType::ConstantNode(o) = x {
                                            variable_occurrence.insert(
                                                v,
                                                EquationComponentType::ConstantNode(o + c),
                                            );
                                        }
                                    }
                                    None => {
                                        variable_occurrence
                                            .insert(v, EquationComponentType::ConstantNode(c));
                                    }
                                };
                                return false;
                            }
                        }
                    }

                    if let EquationComponentType::VariableNode(v) = node_to_simplify {
                        match variable_occurrence.remove(&v) {
                            Some(x) => {
                                if let EquationComponentType::ConstantNode(o) = x {
                                    variable_occurrence
                                        .insert(*v, EquationComponentType::ConstantNode(o + 1));
                                }
                            }
                            None => {
                                variable_occurrence.insert(
                                    *v,
                                    EquationComponentType::ConstantNode(Number::from(1)),
                                );
                            }
                        };
                        return false;
                    }
                    return true;
                });

                for (k, v) in variable_occurrence.into_iter() {
                    if let EquationComponentType::ConstantNode(o) = v.clone() {
                        if o != Number::from(1) {
                            variables_nodes.push(EquationComponentType::PowNode {
                                base: Box::new(EquationComponentType::VariableNode(k)),
                                exponent: Box::new(v),
                            });
                        } else {
                            variables_nodes.push(EquationComponentType::VariableNode(k));
                        }
                    }
                }

                // TODO: implement the following simplifications
                // x * (y + z) = x * y + x * z

                // creating new MulNode with all the computed and simplified nodes
                if variables_nodes.len() == 0 {
                    return EquationComponentType::ConstantNode(constant);
                }

                if variables_nodes.len() == 1 {
                    if constant_is_one {
                        return variables_nodes.pop().unwrap().simplify();
                    }
                    return EquationComponentType::MulNode {
                        lhs: Box::new(EquationComponentType::ConstantNode(constant)),
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
                    lhs: Box::new(EquationComponentType::ConstantNode(constant)),
                    rhs: Box::new(base_node),
                };
            } // End EquationComponentType::MulNod

            EquationComponentType::DivNode {
                numerator,
                denominator,
            } => {
                // TODO: implement the following simplifications `2 * x / x = 2`

                // TODO: implement the following simplifications `x^3 / x^2 = x`

                // TODO: implement the following simplifications `x / (y / z) = (x * z) / y`

                let numerator: EquationComponentType = numerator.simplify();
                let denominator: EquationComponentType = denominator.simplify();

                if let EquationComponentType::ConstantNode(i) = numerator {
                    if let EquationComponentType::ConstantNode(j) = denominator {
                        let result = i / j;
                        return EquationComponentType::ConstantNode(result);
                    } else {
                        return EquationComponentType::DivNode {
                            numerator: Box::new(EquationComponentType::ConstantNode(i)),
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

                // x^1 -> x
                if let EquationComponentType::ConstantNode(i) = exponent.clone() {
                    if i == Number::from(1) {
                        return base.simplify();
                    }
                }

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
                } else if let EquationComponentType::ConstantNode(i) = base {
                    if let EquationComponentType::ConstantNode(j) = exponent {
                        let result = i.pow(&j);
                        return EquationComponentType::ConstantNode(result);
                    } else {
                        return EquationComponentType::PowNode {
                            base: Box::new(EquationComponentType::ConstantNode(i)),
                            exponent: Box::new(exponent),
                        };
                    }
                } else if let EquationComponentType::ConstantNode(i) = base {
                    if let EquationComponentType::ConstantNode(j) = exponent {
                        let result = i.pow(&j);
                        return EquationComponentType::ConstantNode(result);
                    } else {
                        return EquationComponentType::PowNode {
                            base: Box::new(EquationComponentType::ConstantNode(i)),
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

            EquationComponentType::LogNode { base, argument } => {
                // TODO: implement the following simplification `log_x(x^4) = 4`
                //  log_base(base ^ n) = n

                // TODO: implement the following simplification `log(x^n) = n*log(x)`

                EquationComponentType::LogNode {
                    base: Box::new(base.simplify()),
                    argument: Box::new(argument.simplify()),
                }
            } // End EquationComponentType::LogNode

            EquationComponentType::MinusNode(value) => {
                let value: EquationComponentType = value.simplify();

                match value {
                    EquationComponentType::ConstantNode(i) => {
                        EquationComponentType::ConstantNode(-i)
                    }
                    EquationComponentType::AddNode { lhs, rhs } => EquationComponentType::AddNode {
                        lhs: Box::new(EquationComponentType::MinusNode(lhs)),
                        rhs: Box::new(EquationComponentType::MinusNode(rhs)),
                    }
                    .simplify(),
                    EquationComponentType::SubNode { lhs, rhs } => EquationComponentType::SubNode {
                        lhs: Box::new(EquationComponentType::MinusNode(lhs)),
                        rhs: Box::new(EquationComponentType::MinusNode(rhs)),
                    }
                    .simplify(),
                    EquationComponentType::MulNode { lhs, rhs } => EquationComponentType::MulNode {
                        lhs: Box::new(EquationComponentType::MinusNode(lhs)),
                        rhs: rhs,
                    }
                    .simplify(),
                    EquationComponentType::DivNode {
                        numerator,
                        denominator,
                    } => EquationComponentType::DivNode {
                        numerator: Box::new(EquationComponentType::MinusNode(numerator)),
                        denominator: denominator,
                    }
                    .simplify(),
                    EquationComponentType::MinusNode(i) => *i,
                    n => EquationComponentType::MinusNode(Box::new(n.simplify())),
                }
            }
        }
    }

    fn order(&self) -> Self {
        let sort = |terms: &mut Vec<EquationComponentType>, weights: &mut Vec<Number>| {
            for i in 0..terms.len() {
                let mut highest = i;
                for j in i + 1..terms.len() {
                    if weights[highest] < weights[j] {
                        highest = j;
                    }
                }
                if i != highest {
                    weights.swap(i, highest);
                    terms.swap(i, highest);
                }
            }
        };
        match self {
            EquationComponentType::ConstantNode(i) => {
                EquationComponentType::ConstantNode(i.clone())
            }
            EquationComponentType::VariableNode(i) => EquationComponentType::VariableNode(*i),
            EquationComponentType::AddNode { lhs, rhs } => {
                let mut terms: Vec<EquationComponentType> = Vec::new();
                lhs.separate_terms(&mut terms);
                rhs.separate_terms(&mut terms);

                let mut weights: Vec<Number> = Vec::new();
                for i in 0..terms.len() {
                    weights.push(terms[i].calculate_weight());
                }
                sort(&mut terms, &mut weights);
                EquationComponentType::construct_from_terms(terms)
            }
            EquationComponentType::MulNode { lhs, rhs } => {
                let mut terms: Vec<EquationComponentType> = Vec::new();
                lhs.separate_products(&mut terms);
                rhs.separate_products(&mut terms);

                let mut weights: Vec<Number> = Vec::new();
                for i in 0..terms.len() {
                    weights.push(terms[i].calculate_weight());
                }
                sort(&mut terms, &mut weights);
                EquationComponentType::construct_from_products(terms)
            }
            EquationComponentType::SubNode { lhs, rhs } => EquationComponentType::SubNode {
                // ???: This not should not exist after the simplify step
                lhs: Box::new(lhs.order()),
                rhs: Box::new(rhs.order()),
            },
            EquationComponentType::DivNode {
                numerator,
                denominator,
            } => EquationComponentType::DivNode {
                numerator: Box::new(numerator.order()),
                denominator: Box::new(denominator.order()),
            },
            EquationComponentType::PowNode { base, exponent } => EquationComponentType::PowNode {
                base: Box::new(base.order()),
                exponent: Box::new(exponent.order()),
            },
            EquationComponentType::LogNode { base, argument } => EquationComponentType::LogNode {
                base: Box::new(base.order()),
                argument: Box::new(argument.order()),
            },
            EquationComponentType::MinusNode(i) => {
                EquationComponentType::MinusNode(Box::new(i.order()))
            }
        }
    }

    fn calculate_weight(&self) -> Number {
        match self {
            EquationComponentType::ConstantNode(i) => i.clone(),
            EquationComponentType::VariableNode(i) => Number::from((*i) as u32),
            EquationComponentType::AddNode { lhs, rhs } => {
                lhs.calculate_weight() + rhs.calculate_weight()
            }
            EquationComponentType::SubNode { lhs, rhs } => {
                lhs.calculate_weight() - rhs.calculate_weight()
            }
            EquationComponentType::MulNode { lhs, rhs } => {
                lhs.calculate_weight() * rhs.calculate_weight()
            }
            EquationComponentType::DivNode {
                numerator,
                denominator,
            } => numerator.calculate_weight() / denominator.calculate_weight(),
            EquationComponentType::PowNode { base, exponent } => {
                base.calculate_weight().pow(&exponent.calculate_weight())
            }
            EquationComponentType::LogNode {
                base: _,
                argument: _,
            } => {
                // TODO: implement
                todo!();
            }
            EquationComponentType::MinusNode(i) => -(i.calculate_weight()),
        }
    }

    fn construct_from_terms(mut terms: Vec<EquationComponentType>) -> EquationComponentType {
        if terms.len() == 0 {
            EquationComponentType::ConstantNode(Number::from(0))
        } else if terms.len() == 1 {
            terms.remove(0)
        } else {
            EquationComponentType::AddNode {
                lhs: Box::new(terms.remove(0)),
                rhs: Box::new(EquationComponentType::construct_from_terms(terms)),
            }
        }
    }

    fn construct_from_products(mut terms: Vec<EquationComponentType>) -> EquationComponentType {
        if terms.len() == 0 {
            EquationComponentType::ConstantNode(Number::from(0))
        } else if terms.len() == 1 {
            terms.remove(0)
        } else {
            EquationComponentType::MulNode {
                lhs: Box::new(terms.remove(0)),
                rhs: Box::new(EquationComponentType::construct_from_products(terms)),
            }
        }
    }

    fn separate_terms(&self, terms: &mut Vec<EquationComponentType>) {
        match self {
            EquationComponentType::AddNode { lhs, rhs } => {
                lhs.separate_terms(terms);
                rhs.separate_terms(terms);
            }
            n => terms.push(n.order()),
        };
    }

    fn separate_products(&self, products: &mut Vec<EquationComponentType>) {
        match self {
            EquationComponentType::MulNode { lhs, rhs } => {
                lhs.separate_products(products);
                rhs.separate_products(products);
            }
            n => products.push(n.order()),
        }
    }

    fn substitute(&self, variable: char, value: &EquationComponentType) -> Self {
        match self {
            EquationComponentType::ConstantNode(i) => {
                EquationComponentType::ConstantNode(i.clone())
            }
            EquationComponentType::VariableNode(i) => {
                if *i == variable {
                    return value.clone();
                }
                return EquationComponentType::VariableNode(*i);
            }
            EquationComponentType::AddNode { lhs, rhs } => EquationComponentType::AddNode {
                lhs: Box::new(lhs.substitute(variable, value)),
                rhs: Box::new(rhs.substitute(variable, value)),
            },
            EquationComponentType::SubNode { lhs, rhs } => EquationComponentType::SubNode {
                lhs: Box::new(lhs.substitute(variable, value)),
                rhs: Box::new(rhs.substitute(variable, value)),
            },
            EquationComponentType::MulNode { lhs, rhs } => EquationComponentType::MulNode {
                lhs: Box::new(lhs.substitute(variable, value)),
                rhs: Box::new(rhs.substitute(variable, value)),
            },
            EquationComponentType::DivNode {
                numerator,
                denominator,
            } => EquationComponentType::DivNode {
                numerator: Box::new(numerator.substitute(variable, value)),
                denominator: Box::new(denominator.substitute(variable, value)),
            },
            EquationComponentType::PowNode { base, exponent } => EquationComponentType::PowNode {
                base: Box::new(base.substitute(variable, value)),
                exponent: Box::new(exponent.substitute(variable, value)),
            },
            EquationComponentType::LogNode { base, argument } => EquationComponentType::LogNode {
                base: Box::new(base.substitute(variable, value)),
                argument: Box::new(argument.substitute(variable, value)),
            },
            EquationComponentType::MinusNode(node) => {
                EquationComponentType::MinusNode(Box::new(node.substitute(variable, value)))
            }
        }
    }

    fn extract(
        &self,
        variables: &mut Vec<char>,
        constants: &mut Vec<Number>,
        nodes: &mut Vec<EquationComponentType>,
    ) {
        match self {
            EquationComponentType::AddNode { lhs, rhs } => {
                match &**lhs {
                    EquationComponentType::ConstantNode(i) => constants.push(i.clone()),
                    EquationComponentType::VariableNode(i) => variables.push(*i),
                    i @ EquationComponentType::AddNode { lhs: _, rhs: _ } => {
                        i.extract(variables, constants, nodes)
                    }
                    n => {
                        let m = n.simplify();
                        match m {
                            EquationComponentType::ConstantNode(i) => constants.push(i),
                            EquationComponentType::VariableNode(i) => variables.push(i),
                            i @ EquationComponentType::AddNode { lhs: _, rhs: _ } => {
                                i.extract(variables, constants, nodes)
                            }
                            n => nodes.push(n),
                        }
                    }
                };

                match &**rhs {
                    EquationComponentType::ConstantNode(i) => constants.push(i.clone()),
                    EquationComponentType::VariableNode(i) => variables.push(*i),
                    i @ EquationComponentType::AddNode { lhs: _, rhs: _ } => {
                        i.extract(variables, constants, nodes)
                    }
                    n => {
                        let m = n.simplify();
                        match m {
                            EquationComponentType::ConstantNode(i) => constants.push(i),
                            EquationComponentType::VariableNode(i) => variables.push(i),
                            i @ EquationComponentType::AddNode { lhs: _, rhs: _ } => {
                                i.extract(variables, constants, nodes)
                            }
                            n => nodes.push(n),
                        }
                    }
                };
            } // End EquationComponentType::AddNode

            EquationComponentType::MulNode { lhs, rhs } => {
                match &**lhs {
                    EquationComponentType::ConstantNode(i) => constants.push(i.clone()),
                    EquationComponentType::VariableNode(i) => variables.push(*i),
                    i @ EquationComponentType::MulNode { lhs: _, rhs: _ } => {
                        i.extract(variables, constants, nodes)
                    }
                    n => {
                        let m = n.simplify();

                        match m {
                            EquationComponentType::ConstantNode(i) => constants.push(i),
                            EquationComponentType::VariableNode(i) => variables.push(i),
                            i @ EquationComponentType::MulNode { lhs: _, rhs: _ } => {
                                i.extract(variables, constants, nodes)
                            }
                            n => nodes.push(n),
                        }
                    }
                };

                match &**rhs {
                    EquationComponentType::ConstantNode(i) => constants.push(i.clone()),
                    EquationComponentType::VariableNode(i) => variables.push(*i),
                    i @ EquationComponentType::MulNode { lhs: _, rhs: _ } => {
                        i.extract(variables, constants, nodes)
                    }
                    n => {
                        let m = n.simplify();

                        match m {
                            EquationComponentType::ConstantNode(i) => constants.push(i),
                            EquationComponentType::VariableNode(i) => variables.push(i),
                            i @ EquationComponentType::MulNode { lhs: _, rhs: _ } => {
                                i.extract(variables, constants, nodes)
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
    pub fn substitute(&self, variable: char, value: &PartEquation) -> PartEquation {
        PartEquation {
            eq: self.eq.substitute(variable, &value.eq).simplify().order(),
        }
    }

    fn simplify(&self) -> Self {
        PartEquation {
            eq: self.eq.simplify().order(),
        }
    }

    pub fn pow(&self, exponent: &PartEquation) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode {
                base: Box::new(self.eq.clone()),
                exponent: Box::new(exponent.eq.clone()),
            }
            .simplify()
            .order(),
        }
    }
}

impl Display for PartEquation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.eq)
    }
}

impl PartialEq for PartEquation {
    fn eq(&self, other: &Self) -> bool {
        self.eq.simplify().order() == other.eq.simplify().order()
    }
}

impl Eq for PartEquation {}

impl From<char> for PartEquation {
    fn from(value: char) -> Self {
        PartEquation {
            eq: EquationComponentType::VariableNode(value),
        }
    }
}

impl From<i8> for PartEquation {
    fn from(value: i8) -> Self {
        PartEquation {
            eq: EquationComponentType::ConstantNode(Number::from(value)),
        }
    }
}

impl From<i16> for PartEquation {
    fn from(value: i16) -> Self {
        PartEquation {
            eq: EquationComponentType::ConstantNode(Number::from(value)),
        }
    }
}

impl From<i32> for PartEquation {
    fn from(value: i32) -> Self {
        PartEquation {
            eq: EquationComponentType::ConstantNode(Number::from(value)),
        }
    }
}

impl From<i64> for PartEquation {
    fn from(value: i64) -> Self {
        PartEquation {
            eq: EquationComponentType::ConstantNode(Number::from(value)),
        }
    }
}

impl From<i128> for PartEquation {
    fn from(value: i128) -> Self {
        PartEquation {
            eq: EquationComponentType::ConstantNode(Number::from(value)),
        }
    }
}

impl From<u8> for PartEquation {
    fn from(value: u8) -> Self {
        PartEquation {
            eq: EquationComponentType::ConstantNode(Number::from(value)),
        }
    }
}

impl From<u16> for PartEquation {
    fn from(value: u16) -> Self {
        PartEquation {
            eq: EquationComponentType::ConstantNode(Number::from(value)),
        }
    }
}

impl From<u32> for PartEquation {
    fn from(value: u32) -> Self {
        PartEquation {
            eq: EquationComponentType::ConstantNode(Number::from(value)),
        }
    }
}

impl From<u64> for PartEquation {
    fn from(value: u64) -> Self {
        PartEquation {
            eq: EquationComponentType::ConstantNode(Number::from(value)),
        }
    }
}

impl From<u128> for PartEquation {
    fn from(value: u128) -> Self {
        PartEquation {
            eq: EquationComponentType::ConstantNode(Number::from(value)),
        }
    }
}

impl From<f32> for PartEquation {
    fn from(value: f32) -> Self {
        PartEquation {
            eq: EquationComponentType::ConstantNode(Number::from(value)),
        }
    }
}

impl From<f64> for PartEquation {
    fn from(value: f64) -> Self {
        PartEquation {
            eq: EquationComponentType::ConstantNode(Number::from(value)),
        }
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
            // TODO: Implement numeric approximation
            return Err(MathError::NotYetImplemented);
        } else if Self::count_occurrences(&eq, variable) == 0 {
            return Err(MathError::EquationMismatchError);
        }

        match Self::do_inverse(&eq, variable) {
            Ok(result) => Ok(PartEquation { eq: result }),
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

        let mut result: EquationComponentType =
            EquationComponentType::ConstantNode(Number::from(0));
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
                                numerator: Box::new(EquationComponentType::ConstantNode(
                                    Number::from(1),
                                )),
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
                                numerator: Box::new(EquationComponentType::ConstantNode(
                                    Number::from(1),
                                )),
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
        return Ok(result.simplify().order());
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
        .simplify()
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
        .simplify()
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
        .simplify()
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
        .simplify()
    }
}

impl ops::Add<i64> for PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl ops::Add<f64> for PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl ops::Add<PartEquation> for i64 {
    type Output = PartEquation;

    fn add(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                rhs: Box::new(rhs.eq),
            },
        }
        .simplify()
    }
}

impl ops::Add<PartEquation> for f64 {
    type Output = PartEquation;

    fn add(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                rhs: Box::new(rhs.eq),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Add<i64> for &'a PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Add<f64> for &'a PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Add<&'a PartEquation> for i64 {
    type Output = PartEquation;

    fn add(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Add<&'a PartEquation> for f64 {
    type Output = PartEquation;

    fn add(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode {
                lhs: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
        .simplify()
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
        .simplify()
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
        .simplify()
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
        .simplify()
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
        .simplify()
    }
}

impl ops::Sub<i64> for PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl ops::Sub<f64> for PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl ops::Sub<PartEquation> for i64 {
    type Output = PartEquation;

    fn sub(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                rhs: Box::new(rhs.eq),
            },
        }
        .simplify()
    }
}

impl ops::Sub<PartEquation> for f64 {
    type Output = PartEquation;

    fn sub(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                rhs: Box::new(rhs.eq),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Sub<i64> for &'a PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Sub<f64> for &'a PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Sub<&'a PartEquation> for i64 {
    type Output = PartEquation;

    fn sub(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Sub<&'a PartEquation> for f64 {
    type Output = PartEquation;

    fn sub(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode {
                lhs: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
        .simplify()
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
        .simplify()
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
        .simplify()
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
        .simplify()
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
        .simplify()
    }
}

impl ops::Mul<i64> for PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl ops::Mul<f64> for PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl ops::Mul<PartEquation> for i64 {
    type Output = PartEquation;

    fn mul(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                rhs: Box::new(rhs.eq),
            },
        }
        .simplify()
    }
}

impl ops::Mul<PartEquation> for f64 {
    type Output = PartEquation;

    fn mul(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                rhs: Box::new(rhs.eq),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Mul<i64> for &'a PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Mul<f64> for &'a PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Mul<&'a PartEquation> for i64 {
    type Output = PartEquation;

    fn mul(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Mul<&'a PartEquation> for f64 {
    type Output = PartEquation;

    fn mul(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode {
                lhs: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                rhs: Box::new(rhs.eq.clone()),
            },
        }
        .simplify()
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
        .simplify()
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
        .simplify()
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
        .simplify()
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
        .simplify()
    }
}

impl ops::Div<i64> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(self.eq),
                denominator: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl ops::Div<f64> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(self.eq),
                denominator: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl ops::Div<PartEquation> for i64 {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                denominator: Box::new(rhs.eq),
            },
        }
        .simplify()
    }
}

impl ops::Div<PartEquation> for f64 {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                denominator: Box::new(rhs.eq),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Div<i64> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(self.eq.clone()),
                denominator: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Div<f64> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(self.eq.clone()),
                denominator: Box::new(EquationComponentType::ConstantNode(Number::from(rhs))),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Div<&'a PartEquation> for i64 {
    type Output = PartEquation;

    fn div(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                denominator: Box::new(rhs.eq.clone()),
            },
        }
        .simplify()
    }
}

impl<'a> ops::Div<&'a PartEquation> for f64 {
    type Output = PartEquation;

    fn div(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode {
                numerator: Box::new(EquationComponentType::ConstantNode(Number::from(self))),
                denominator: Box::new(rhs.eq.clone()),
            },
        }
        .simplify()
    }
}

impl ops::Neg for PartEquation {
    type Output = PartEquation;

    fn neg(self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MinusNode(Box::new(self.eq)),
        }
        .simplify()
    }
}

impl<'a> ops::Neg for &'a PartEquation {
    type Output = PartEquation;

    fn neg(self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MinusNode(Box::new(self.eq.clone())),
        }
        .simplify()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solving_equation_1() {
        let x: PartEquation = PartEquation::from('x');
        let eq: Equation = Equation::new(&x, &PartEquation::from(12));

        if let EquationComponentType::ConstantNode(i) = eq.solve('x').unwrap().eq {
            assert_eq!(i, Number::from(12));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_solving_equation_2() {
        let x: PartEquation = PartEquation::from('x');
        let eq: Equation = Equation::new(&PartEquation::from(3.14), &x);

        if let EquationComponentType::ConstantNode(i) = eq.solve('x').unwrap().eq {
            assert_eq!(i, Number::from(3.14));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_solving_equation_3() {
        let x: PartEquation = PartEquation::from('x');
        let eq: Equation = Equation::new(&PartEquation::from(3), &(x * 2));

        if let EquationComponentType::ConstantNode(i) = eq.solve('x').unwrap().eq {
            assert_eq!(i, Number::from(1.5));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_solving_equation_4() {
        let x: PartEquation = PartEquation::from('x');
        let eq: Equation = Equation::new(&PartEquation::from(3), &(x + 2));

        if let EquationComponentType::ConstantNode(i) = eq.solve('x').unwrap().eq {
            assert_eq!(i, Number::from(1));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_solving_equation_5() {
        let x: PartEquation = PartEquation::from('x');
        let eq: Equation = Equation::new(&PartEquation::from(3), &(x / 2));

        if let EquationComponentType::ConstantNode(i) = eq.solve('x').unwrap().eq {
            assert_eq!(i, Number::from(6));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_solving_equation_6() {
        let x: PartEquation = PartEquation::from('x');
        let eq: Equation = Equation::new(&PartEquation::from(9), &(&x.pow(&PartEquation::from(2))));

        if let EquationComponentType::ConstantNode(i) = eq.solve('x').unwrap().eq {
            assert_eq!(i, Number::from(3));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_solving_equation_7() {
        // TODO: evaluate log
        let x: PartEquation = PartEquation::from('x');
        let eq: Equation = Equation::new(&PartEquation::from(8), &(&PartEquation::from(2).pow(&x)));

        if let EquationComponentType::LogNode { base, argument } = eq.solve('x').unwrap().eq {
            if let EquationComponentType::ConstantNode(i) = *base {
                assert_eq!(i, Number::from(2));
            } else {
                assert!(false);
            }

            if let EquationComponentType::ConstantNode(i) = *argument {
                assert_eq!(i, Number::from(8));
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_solving_equation_8() {
        let x: PartEquation = PartEquation::from('x');
        let eq: Equation = Equation::new(&(-x), &PartEquation::from(1));

        if let EquationComponentType::ConstantNode(i) = eq.solve('x').unwrap().eq {
            assert_eq!(i, Number::from(-1));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_solving_equation_9() {
        let x: PartEquation = PartEquation::from('x');
        let eq: Equation = Equation::new(&(&x + 5), &(2 * &x));

        if let EquationComponentType::ConstantNode(i) = eq.solve('x').unwrap().eq {
            assert_eq!(i, Number::from(5));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_solving_equation_10() {
        let x: PartEquation = PartEquation::from('x');
        let eq: Equation = Equation::new(&(-&x + 5), &(2 * &x));

        if let EquationComponentType::ConstantNode(i) = eq.solve('x').unwrap().eq {
            assert_eq!(i, Number::from(5) / Number::from(3));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_equality_for_part_equation_1() {
        let x: PartEquation = PartEquation::from('x');
        let y: PartEquation = PartEquation::from('y');
        let z: PartEquation = PartEquation::from('z');

        let eq1 = &x + &y + &z;

        assert_eq!(eq1, &x + &z + &y);
        assert_eq!(eq1, &y + &x + &z);
        assert_eq!(eq1, &y + &z + &x);
        assert_eq!(eq1, &z + &y + &x);
        assert_eq!(eq1, &z + &x + &y);
    }

    #[test]
    fn test_equality_for_part_equation_2() {
        let x: PartEquation = PartEquation::from('x');
        let y: PartEquation = PartEquation::from('y');
        let z: PartEquation = PartEquation::from('z');

        let eq1 = &x * &y * &z;

        assert_eq!(eq1, &x * &z * &y);
        assert_eq!(eq1, &y * &x * &z);
        assert_eq!(eq1, &y * &z * &x);
        assert_eq!(eq1, &z * &y * &x);
        assert_eq!(eq1, &z * &x * &y);
    }

    #[test]
    fn test_equality_for_part_equation_3() {
        let x: PartEquation = PartEquation::from('x');
        let y: PartEquation = PartEquation::from('y');
        let z: PartEquation = PartEquation::from('z');

        let eq1 = &x * (&y + &z);

        assert_eq!(eq1, &x * (&z + &y));
        assert_eq!(eq1, (&y + &z) * (&x));
        assert_eq!(eq1, (&z + &y) * (&x));
    }
}
