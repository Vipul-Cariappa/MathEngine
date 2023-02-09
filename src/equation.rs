use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops;

use super::math;

#[derive(Clone)]
pub enum EquationComponentType {
    Integer(Integer),
    Decimal(Decimal),
    VariableNode(VariableNode),
    AddNode(AddNode),
    SubNode(SubNode),
    MulNode(MulNode),
    DivNode(DivNode),
    PowNode(PowNode),
    MinusNode(MinusNode),
}

impl Debug for EquationComponentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EquationComponentType::Integer(i) => write!(f, "{:?}", i),
            EquationComponentType::Decimal(i) => write!(f, "{:?}", i),
            EquationComponentType::VariableNode(i) => write!(f, "{:?}", i),
            EquationComponentType::AddNode(i) => write!(f, "{:?}", i),
            EquationComponentType::SubNode(i) => write!(f, "{:?}", i),
            EquationComponentType::MulNode(i) => write!(f, "{:?}", i),
            EquationComponentType::DivNode(i) => write!(f, "{:?}", i),
            EquationComponentType::PowNode(i) => write!(f, "{:?}", i),
            EquationComponentType::MinusNode(i) => write!(f, "{:?}", i),
        }
    }
}

impl Display for EquationComponentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EquationComponentType::Integer(i) => write!(f, "{}", i),
            EquationComponentType::Decimal(i) => write!(f, "{}", i),
            EquationComponentType::VariableNode(i) => write!(f, "{}", i),
            EquationComponentType::AddNode(i) => write!(f, "{}", i),
            EquationComponentType::SubNode(i) => write!(f, "{}", i),
            EquationComponentType::MulNode(i) => write!(f, "{}", i),
            EquationComponentType::DivNode(i) => write!(f, "{}", i),
            EquationComponentType::PowNode(i) => write!(f, "{}", i),
            EquationComponentType::MinusNode(i) => write!(f, "{}", i),
        }
    }
}

impl EquationComponentType {
    fn simplify(&self) -> EquationComponentType {
        match self {
            EquationComponentType::Integer(i) => EquationComponentType::Integer(*i),
            EquationComponentType::Decimal(i) => EquationComponentType::Decimal(*i),
            EquationComponentType::VariableNode(i) => EquationComponentType::VariableNode(*i),
            EquationComponentType::AddNode(i) => i.simplify(),
            EquationComponentType::SubNode(i) => i.simplify(),
            EquationComponentType::MulNode(i) => i.simplify(),
            EquationComponentType::DivNode(i) => i.simplify(),
            EquationComponentType::PowNode(i) => i.simplify(),
            EquationComponentType::MinusNode(i) => i.simplify(),
        }
    }

    fn substitutei(&self, variable: char, value: i128) -> EquationComponentType {
        match self {
            EquationComponentType::Integer(i) => EquationComponentType::Integer(*i),
            EquationComponentType::Decimal(i) => EquationComponentType::Decimal(*i),
            EquationComponentType::VariableNode(i) => {
                if i.variable == variable {
                    return EquationComponentType::Integer(Integer { value: value });
                }
                return EquationComponentType::VariableNode(*i);
            }
            EquationComponentType::AddNode(i) => i.substitutei(variable, value),
            EquationComponentType::SubNode(i) => i.substitutei(variable, value),
            EquationComponentType::MulNode(i) => i.substitutei(variable, value),
            EquationComponentType::DivNode(i) => i.substitutei(variable, value),
            EquationComponentType::PowNode(i) => i.substitutei(variable, value),
            EquationComponentType::MinusNode(i) => i.substitutei(variable, value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartEquation {
    pub eq: EquationComponentType,
}

impl PartEquation {
    pub fn substitutei(&self, variable: char, value: i128) -> PartEquation {
        PartEquation {
            eq: self.eq.substitutei(variable, value).simplify(),
        }
    }

    pub fn new(variable: char) -> Self {
        PartEquation {
            eq: EquationComponentType::VariableNode(VariableNode { variable: variable }),
        }
    }

    pub fn newi(value: i128) -> Self {
        PartEquation {
            eq: EquationComponentType::Integer(Integer { value: value }),
        }
    }

    pub fn newf(value: f64) -> Self {
        PartEquation {
            eq: EquationComponentType::Decimal(Decimal { value: value }),
        }
    }

    pub fn simplify(&self) -> Self {
        PartEquation {
            eq: self.eq.simplify(),
        }
    }

    pub fn pow(&self, exponent: &PartEquation) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode(PowNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(exponent.eq.clone()),
            }),
        }
    }

    pub fn powi32(&self, exponent: i32) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode(PowNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: exponent as i128,
                })),
            }),
        }
    }

    pub fn powi64(&self, exponent: i64) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode(PowNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: exponent as i128,
                })),
            }),
        }
    }

    pub fn powi128(&self, exponent: i128) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode(PowNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer { value: exponent })),
            }),
        }
    }

    pub fn powf32(&self, exponent: f32) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode(PowNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: exponent as f64,
                })),
            }),
        }
    }

    pub fn powf64(&self, exponent: f64) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode(PowNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(Decimal { value: exponent })),
            }),
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
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl<'a> ops::Add<&'a PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Add<PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl<'a> ops::Add<&'a PartEquation> for PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: &'a PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl ops::Add<i32> for PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: i32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl ops::Add<i64> for PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl ops::Add<i128> for PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: i128) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer { value: rhs })),
            }),
        }
    }
}

impl ops::Add<f32> for PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: f32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: rhs as f64,
                })),
            }),
        }
    }
}

impl ops::Add<f64> for PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Decimal(Decimal { value: rhs })),
            }),
        }
    }
}

impl ops::Add<PartEquation> for i32 {
    type Output = PartEquation;

    fn add(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Add<PartEquation> for i64 {
    type Output = PartEquation;

    fn add(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Add<PartEquation> for i128 {
    type Output = PartEquation;

    fn add(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(EquationComponentType::Integer(Integer { value: self })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Add<PartEquation> for f32 {
    type Output = PartEquation;

    fn add(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: self as f64,
                })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Add<PartEquation> for f64 {
    type Output = PartEquation;

    fn add(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal { value: self })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl<'a> ops::Add<i32> for &'a PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: i32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl<'a> ops::Add<i64> for &'a PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl<'a> ops::Add<i128> for &'a PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: i128) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer { value: rhs })),
            }),
        }
    }
}

impl<'a> ops::Add<f32> for &'a PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: f32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: rhs as f64,
                })),
            }),
        }
    }
}

impl<'a> ops::Add<f64> for &'a PartEquation {
    type Output = PartEquation;

    fn add(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(Decimal { value: rhs })),
            }),
        }
    }
}

impl<'a> ops::Add<&'a PartEquation> for i32 {
    type Output = PartEquation;

    fn add(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Add<&'a PartEquation> for i64 {
    type Output = PartEquation;

    fn add(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Add<&'a PartEquation> for i128 {
    type Output = PartEquation;

    fn add(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(EquationComponentType::Integer(Integer { value: self })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Add<&'a PartEquation> for f32 {
    type Output = PartEquation;

    fn add(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: self as f64,
                })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Add<&'a PartEquation> for f64 {
    type Output = PartEquation;

    fn add(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::AddNode(AddNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal { value: self })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl ops::Sub<PartEquation> for PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl<'a> ops::Sub<&'a PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Sub<PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl<'a> ops::Sub<&'a PartEquation> for PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: &'a PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl ops::Sub<i32> for PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: i32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl ops::Sub<i64> for PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl ops::Sub<i128> for PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: i128) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer { value: rhs })),
            }),
        }
    }
}

impl ops::Sub<f32> for PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: f32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: rhs as f64,
                })),
            }),
        }
    }
}

impl ops::Sub<f64> for PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Decimal(Decimal { value: rhs })),
            }),
        }
    }
}

impl ops::Sub<PartEquation> for i32 {
    type Output = PartEquation;

    fn sub(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Sub<PartEquation> for i64 {
    type Output = PartEquation;

    fn sub(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Sub<PartEquation> for i128 {
    type Output = PartEquation;

    fn sub(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(EquationComponentType::Integer(Integer { value: self })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Sub<PartEquation> for f32 {
    type Output = PartEquation;

    fn sub(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: self as f64,
                })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Sub<PartEquation> for f64 {
    type Output = PartEquation;

    fn sub(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal { value: self })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl<'a> ops::Sub<i32> for &'a PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: i32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl<'a> ops::Sub<i64> for &'a PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl<'a> ops::Sub<i128> for &'a PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: i128) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer { value: rhs })),
            }),
        }
    }
}

impl<'a> ops::Sub<f32> for &'a PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: f32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: rhs as f64,
                })),
            }),
        }
    }
}

impl<'a> ops::Sub<f64> for &'a PartEquation {
    type Output = PartEquation;

    fn sub(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(Decimal { value: rhs })),
            }),
        }
    }
}

impl<'a> ops::Sub<&'a PartEquation> for i32 {
    type Output = PartEquation;

    fn sub(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Sub<&'a PartEquation> for i64 {
    type Output = PartEquation;

    fn sub(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Sub<&'a PartEquation> for i128 {
    type Output = PartEquation;

    fn sub(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(EquationComponentType::Integer(Integer { value: self })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Sub<&'a PartEquation> for f32 {
    type Output = PartEquation;

    fn sub(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: self as f64,
                })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Sub<&'a PartEquation> for f64 {
    type Output = PartEquation;

    fn sub(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::SubNode(SubNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal { value: self })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl ops::Mul<PartEquation> for PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl<'a> ops::Mul<&'a PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Mul<PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl<'a> ops::Mul<&'a PartEquation> for PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: &'a PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl ops::Mul<i32> for PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: i32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl ops::Mul<i64> for PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl ops::Mul<i128> for PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: i128) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer { value: rhs })),
            }),
        }
    }
}

impl ops::Mul<f32> for PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: f32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: rhs as f64,
                })),
            }),
        }
    }
}

impl ops::Mul<f64> for PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Decimal(Decimal { value: rhs })),
            }),
        }
    }
}

impl ops::Mul<PartEquation> for i32 {
    type Output = PartEquation;

    fn mul(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Mul<PartEquation> for i64 {
    type Output = PartEquation;

    fn mul(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Mul<PartEquation> for i128 {
    type Output = PartEquation;

    fn mul(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(EquationComponentType::Integer(Integer { value: self })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Mul<PartEquation> for f32 {
    type Output = PartEquation;

    fn mul(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: self as f64,
                })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Mul<PartEquation> for f64 {
    type Output = PartEquation;

    fn mul(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal { value: self })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl<'a> ops::Mul<i32> for &'a PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: i32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl<'a> ops::Mul<i64> for &'a PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl<'a> ops::Mul<i128> for &'a PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: i128) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer { value: rhs })),
            }),
        }
    }
}

impl<'a> ops::Mul<f32> for &'a PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: f32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: rhs as f64,
                })),
            }),
        }
    }
}

impl<'a> ops::Mul<f64> for &'a PartEquation {
    type Output = PartEquation;

    fn mul(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(Decimal { value: rhs })),
            }),
        }
    }
}

impl<'a> ops::Mul<&'a PartEquation> for i32 {
    type Output = PartEquation;

    fn mul(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Mul<&'a PartEquation> for i64 {
    type Output = PartEquation;

    fn mul(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Mul<&'a PartEquation> for i128 {
    type Output = PartEquation;

    fn mul(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(EquationComponentType::Integer(Integer { value: self })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Mul<&'a PartEquation> for f32 {
    type Output = PartEquation;

    fn mul(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: self as f64,
                })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Mul<&'a PartEquation> for f64 {
    type Output = PartEquation;

    fn mul(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MulNode(MulNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal { value: self })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl ops::Div<PartEquation> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: Self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Div<PartEquation> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: &'a PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl ops::Div<i32> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: i32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl ops::Div<i64> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl ops::Div<i128> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: i128) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer { value: rhs })),
            }),
        }
    }
}

impl ops::Div<f32> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: f32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: rhs as f64,
                })),
            }),
        }
    }
}

impl ops::Div<f64> for PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Decimal(Decimal { value: rhs })),
            }),
        }
    }
}

impl ops::Div<PartEquation> for i32 {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Div<PartEquation> for i64 {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Div<PartEquation> for i128 {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(EquationComponentType::Integer(Integer { value: self })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Div<PartEquation> for f32 {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: self as f64,
                })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl ops::Div<PartEquation> for f64 {
    type Output = PartEquation;

    fn div(self, rhs: PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal { value: self })),
                rhs: Box::new(rhs.eq),
            }),
        }
    }
}

impl<'a> ops::Div<i32> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: i32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl<'a> ops::Div<i64> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: i64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: rhs as i128,
                })),
            }),
        }
    }
}

impl<'a> ops::Div<i128> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: i128) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Integer(Integer { value: rhs })),
            }),
        }
    }
}

impl<'a> ops::Div<f32> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: f32) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: rhs as f64,
                })),
            }),
        }
    }
}

impl<'a> ops::Div<f64> for &'a PartEquation {
    type Output = PartEquation;

    fn div(self, rhs: f64) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(self.eq.clone()),
                rhs: Box::new(EquationComponentType::Decimal(Decimal { value: rhs })),
            }),
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for i32 {
    type Output = PartEquation;

    fn div(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for i64 {
    type Output = PartEquation;

    fn div(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(EquationComponentType::Integer(Integer {
                    value: self as i128,
                })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for i128 {
    type Output = PartEquation;

    fn div(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(EquationComponentType::Integer(Integer { value: self })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for f32 {
    type Output = PartEquation;

    fn div(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: self as f64,
                })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl<'a> ops::Div<&'a PartEquation> for f64 {
    type Output = PartEquation;

    fn div(self, rhs: &PartEquation) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::DivNode(DivNode {
                lhs: Box::new(EquationComponentType::Decimal(Decimal { value: self })),
                rhs: Box::new(rhs.eq.clone()),
            }),
        }
    }
}

impl ops::Neg for PartEquation {
    type Output = PartEquation;

    fn neg(self) -> Self::Output {
        PartEquation {
            eq: EquationComponentType::MinusNode(MinusNode {
                value: Box::new(self.eq),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Integer {
    pub value: i128,
}

impl Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Decimal {
    pub value: f64,
}

impl Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VariableNode {
    pub variable: char,
}

impl Display for VariableNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variable)
    }
}

#[derive(Debug, Clone)]
pub struct AddNode {
    pub lhs: Box<EquationComponentType>,
    pub rhs: Box<EquationComponentType>,
}

impl Display for AddNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} + {})", self.lhs, self.rhs)
    }
}

impl AddNode {
    fn extract(
        &self,
        variables: &mut Vec<char>,
        integers: &mut Vec<i128>,
        decimals: &mut Vec<f64>,
        nodes: &mut Vec<EquationComponentType>,
    ) {
        match &*self.lhs {
            EquationComponentType::Integer(i) => integers.push(i.value),
            EquationComponentType::Decimal(i) => decimals.push(i.value),
            EquationComponentType::VariableNode(i) => variables.push(i.variable),
            EquationComponentType::AddNode(i) => i.extract(variables, integers, decimals, nodes),
            n => nodes.push(n.simplify()),
        };

        match &*self.rhs {
            EquationComponentType::Integer(i) => integers.push(i.value),
            EquationComponentType::Decimal(i) => decimals.push(i.value),
            EquationComponentType::VariableNode(i) => variables.push(i.variable),
            EquationComponentType::AddNode(i) => i.extract(variables, integers, decimals, nodes),
            n => nodes.push(n.simplify()),
        };
    }

    fn simplify(&self) -> EquationComponentType {
        let mut variables: Vec<char> = Vec::new();
        let mut integers: Vec<i128> = Vec::new();
        let mut decimals: Vec<f64> = Vec::new();
        let mut nodes: Vec<EquationComponentType> = Vec::new();

        self.extract(&mut variables, &mut integers, &mut decimals, &mut nodes);

        // println!("\n\nCall on: {}", self);
        // dbg!(integers);
        // dbg!(decimals);
        // dbg!(variables);
        // dbg!(nodes);

        let mut sum_i128: i128 = 0;
        integers.iter().for_each(|x| sum_i128 += x);

        let mut sum_f64: f64 = 0.0;
        decimals.iter().for_each(|x| sum_f64 += x);

        let constant: EquationComponentType = {
            if sum_f64 != 0.0 {
                EquationComponentType::Integer(Integer { value: sum_i128 })
            } else {
                EquationComponentType::Decimal(Decimal {
                    value: sum_f64 + sum_i128 as f64,
                })
            }
        };

        // let new_variables: Vec<EquationComponentType> = Vec::new();
        let mut variable_occurrence: HashMap<char, i32> = HashMap::new();

        for i in variables.iter() {
            match variable_occurrence.get(&i) {
                Some(n) => variable_occurrence.insert(*i, n + 1),
                None => variable_occurrence.insert(*i, 1),
            };
        }

        let mut variables_nodes: Vec<EquationComponentType> = Vec::new();

        for (i, k) in variable_occurrence.iter() {
            variables_nodes.push(EquationComponentType::MulNode(MulNode {
                lhs: Box::new(EquationComponentType::VariableNode(VariableNode {
                    variable: *i,
                })),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: *k as i128,
                })),
            }));
        }

        variables_nodes.extend(nodes);

        if variables_nodes.len() == 0 {
            return constant;
        }

        if variables_nodes.len() == 1 {
            return EquationComponentType::AddNode(AddNode {
                lhs: Box::new(constant),
                rhs: Box::new(variables_nodes.pop().unwrap().simplify()),
            });
        }

        let mut base_node: Box<EquationComponentType> =
            Box::new(EquationComponentType::AddNode(AddNode {
                lhs: Box::new(variables_nodes.pop().unwrap().simplify()),
                rhs: Box::new(variables_nodes.pop().unwrap().simplify()),
            }));

        loop {
            match variables_nodes.pop() {
                Some(i) => {
                    base_node = Box::new(EquationComponentType::AddNode(AddNode {
                        lhs: Box::new(i.simplify()),
                        rhs: base_node,
                    }));
                }
                None => break,
            }
        }

        return EquationComponentType::AddNode(AddNode {
            lhs: Box::new(constant),
            rhs: base_node,
        });

        // return EquationComponentType::AddNode(self.clone());
    }

    fn _simplify(&self) -> EquationComponentType {
        let lhs: EquationComponentType = self.lhs.simplify();
        let rhs: EquationComponentType = self.rhs.simplify();

        if let EquationComponentType::Integer(i) = lhs {
            if let EquationComponentType::Integer(j) = rhs {
                let result: i128 = i.value + j.value;
                return EquationComponentType::Integer(Integer { value: result });
            } else if let EquationComponentType::Decimal(j) = rhs {
                let result: f64 = i.value as f64 + j.value;
                return EquationComponentType::Decimal(Decimal { value: result });
            } else {
                return EquationComponentType::AddNode(AddNode {
                    lhs: Box::new(EquationComponentType::Integer(i)),
                    rhs: Box::new(rhs),
                });
            }
        } else if let EquationComponentType::Decimal(i) = lhs {
            if let EquationComponentType::Integer(j) = rhs {
                let result: f64 = i.value + j.value as f64;
                return EquationComponentType::Decimal(Decimal { value: result });
            } else if let EquationComponentType::Decimal(j) = rhs {
                let result: f64 = i.value + j.value;
                return EquationComponentType::Decimal(Decimal { value: result });
            } else {
                return EquationComponentType::AddNode(AddNode {
                    lhs: Box::new(EquationComponentType::Decimal(i)),
                    rhs: Box::new(rhs),
                });
            }
        } else {
            return EquationComponentType::AddNode(AddNode {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        }
    }

    fn substitutei(&self, variable: char, value: i128) -> EquationComponentType {
        let lhs: EquationComponentType =
            EquationComponentType::substitutei(&self.lhs, variable, value).simplify();
        let rhs: EquationComponentType =
            EquationComponentType::substitutei(&self.rhs, variable, value).simplify();
        EquationComponentType::AddNode(AddNode {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}

#[derive(Debug, Clone)]
pub struct SubNode {
    pub lhs: Box<EquationComponentType>,
    pub rhs: Box<EquationComponentType>,
}

impl Display for SubNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} - {})", self.lhs, self.rhs)
    }
}

impl SubNode {
    fn simplify(&self) -> EquationComponentType {
        let lhs: EquationComponentType = self.lhs.simplify();
        let rhs: EquationComponentType = self.rhs.simplify();

        if let EquationComponentType::Integer(i) = lhs {
            if let EquationComponentType::Integer(j) = rhs {
                let result: i128 = i.value - j.value;
                return EquationComponentType::Integer(Integer { value: result });
            } else if let EquationComponentType::Decimal(j) = rhs {
                let result: f64 = i.value as f64 - j.value;
                return EquationComponentType::Decimal(Decimal { value: result });
            } else {
                return EquationComponentType::SubNode(SubNode {
                    lhs: Box::new(EquationComponentType::Integer(i)),
                    rhs: Box::new(rhs),
                });
            }
        } else if let EquationComponentType::Decimal(i) = lhs {
            if let EquationComponentType::Integer(j) = rhs {
                let result: f64 = i.value - j.value as f64;
                return EquationComponentType::Decimal(Decimal { value: result });
            } else if let EquationComponentType::Decimal(j) = rhs {
                let result: f64 = i.value - j.value;
                return EquationComponentType::Decimal(Decimal { value: result });
            } else {
                return EquationComponentType::SubNode(SubNode {
                    lhs: Box::new(EquationComponentType::Decimal(i)),
                    rhs: Box::new(rhs),
                });
            }
        } else {
            return EquationComponentType::SubNode(SubNode {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        }
    }

    fn substitutei(&self, variable: char, value: i128) -> EquationComponentType {
        let lhs: EquationComponentType =
            EquationComponentType::substitutei(&self.lhs, variable, value).simplify();
        let rhs: EquationComponentType =
            EquationComponentType::substitutei(&self.rhs, variable, value).simplify();
        EquationComponentType::SubNode(SubNode {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MulNode {
    pub lhs: Box<EquationComponentType>,
    pub rhs: Box<EquationComponentType>,
}

impl Display for MulNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} * {})", self.lhs, self.rhs)
    }
}

impl MulNode {
    fn simplify(&self) -> EquationComponentType {
        let lhs: EquationComponentType = self.lhs.simplify();
        let rhs: EquationComponentType = self.rhs.simplify();

        if let EquationComponentType::Integer(i) = lhs {
            if let EquationComponentType::Integer(j) = rhs {
                let result: i128 = i.value * j.value;
                return EquationComponentType::Integer(Integer { value: result });
            } else if let EquationComponentType::Decimal(j) = rhs {
                let result: f64 = i.value as f64 * j.value;
                return EquationComponentType::Decimal(Decimal { value: result });
            } else {
                return EquationComponentType::MulNode(MulNode {
                    lhs: Box::new(EquationComponentType::Integer(i)),
                    rhs: Box::new(rhs),
                });
            }
        } else if let EquationComponentType::Decimal(i) = lhs {
            if let EquationComponentType::Integer(j) = rhs {
                let result: f64 = i.value * j.value as f64;
                return EquationComponentType::Decimal(Decimal { value: result });
            } else if let EquationComponentType::Decimal(j) = rhs {
                let result: f64 = i.value * j.value;
                return EquationComponentType::Decimal(Decimal { value: result });
            } else {
                return EquationComponentType::MulNode(MulNode {
                    lhs: Box::new(EquationComponentType::Decimal(i)),
                    rhs: Box::new(rhs),
                });
            }
        } else {
            return EquationComponentType::MulNode(MulNode {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        }
    }

    fn substitutei(&self, variable: char, value: i128) -> EquationComponentType {
        let lhs: EquationComponentType =
            EquationComponentType::substitutei(&self.lhs, variable, value).simplify();
        let rhs: EquationComponentType =
            EquationComponentType::substitutei(&self.rhs, variable, value).simplify();
        EquationComponentType::MulNode(MulNode {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DivNode {
    pub lhs: Box<EquationComponentType>,
    pub rhs: Box<EquationComponentType>,
}

impl Display for DivNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} / {})", self.lhs, self.rhs)
    }
}

impl DivNode {
    fn simplify(&self) -> EquationComponentType {
        let lhs: EquationComponentType = self.lhs.simplify();
        let rhs: EquationComponentType = self.rhs.simplify();

        if let EquationComponentType::Integer(i) = lhs {
            if let EquationComponentType::Integer(j) = rhs {
                let result: i128 = i.value / j.value;
                return EquationComponentType::Integer(Integer { value: result });
            } else if let EquationComponentType::Decimal(j) = rhs {
                let result: f64 = i.value as f64 / j.value;
                return EquationComponentType::Decimal(Decimal { value: result });
            } else {
                return EquationComponentType::DivNode(DivNode {
                    lhs: Box::new(EquationComponentType::Integer(i)),
                    rhs: Box::new(rhs),
                });
            }
        } else if let EquationComponentType::Decimal(i) = lhs {
            if let EquationComponentType::Integer(j) = rhs {
                let result: f64 = i.value / j.value as f64;
                return EquationComponentType::Decimal(Decimal { value: result });
            } else if let EquationComponentType::Decimal(j) = rhs {
                let result: f64 = i.value / j.value;
                return EquationComponentType::Decimal(Decimal { value: result });
            } else {
                return EquationComponentType::DivNode(DivNode {
                    lhs: Box::new(EquationComponentType::Decimal(i)),
                    rhs: Box::new(rhs),
                });
            }
        } else {
            return EquationComponentType::DivNode(DivNode {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        }
    }

    fn substitutei(&self, variable: char, value: i128) -> EquationComponentType {
        let lhs: EquationComponentType =
            EquationComponentType::substitutei(&self.lhs, variable, value).simplify();
        let rhs: EquationComponentType =
            EquationComponentType::substitutei(&self.rhs, variable, value).simplify();
        EquationComponentType::DivNode(DivNode {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PowNode {
    pub lhs: Box<EquationComponentType>, // lhs is the base
    pub rhs: Box<EquationComponentType>, // rhs is the exponent
}

impl Display for PowNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} ^ {})", self.lhs, self.rhs)
    }
}

impl PowNode {
    fn simplify(&self) -> EquationComponentType {
        let lhs: EquationComponentType = self.lhs.simplify();
        let rhs: EquationComponentType = self.rhs.simplify();

        if let EquationComponentType::Integer(i) = lhs {
            if let EquationComponentType::Integer(j) = rhs {
                let result: i128 = math::powi128(i.value, j.value);
                return EquationComponentType::Integer(Integer { value: result });
            } else if let EquationComponentType::Decimal(j) = rhs {
                let result: f64 = math::powf64(i.value as f64, j.value);
                return EquationComponentType::Decimal(Decimal { value: result });
            } else {
                return EquationComponentType::PowNode(PowNode {
                    lhs: Box::new(EquationComponentType::Integer(i)),
                    rhs: Box::new(rhs),
                });
            }
        } else if let EquationComponentType::Decimal(i) = lhs {
            if let EquationComponentType::Integer(j) = rhs {
                let result: f64 = math::powf64(i.value, j.value as f64);
                return EquationComponentType::Decimal(Decimal { value: result });
            } else if let EquationComponentType::Decimal(j) = rhs {
                let result: f64 = math::powf64(i.value, j.value);
                return EquationComponentType::Decimal(Decimal { value: result });
            } else {
                return EquationComponentType::PowNode(PowNode {
                    lhs: Box::new(EquationComponentType::Decimal(i)),
                    rhs: Box::new(rhs),
                });
            }
        } else {
            return EquationComponentType::PowNode(PowNode {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            });
        }
    }

    fn substitutei(&self, variable: char, value: i128) -> EquationComponentType {
        let lhs: EquationComponentType =
            EquationComponentType::substitutei(&self.lhs, variable, value).simplify();
        let rhs: EquationComponentType =
            EquationComponentType::substitutei(&self.rhs, variable, value).simplify();
        EquationComponentType::PowNode(PowNode {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MinusNode {
    pub value: Box<EquationComponentType>,
}

impl Display for MinusNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(-{})", self.value)
    }
}

impl MinusNode {
    fn simplify(&self) -> EquationComponentType {
        let value: EquationComponentType = self.value.simplify();

        if let EquationComponentType::Integer(i) = value {
            return EquationComponentType::Integer(Integer { value: -(i.value) });
        } else if let EquationComponentType::Decimal(i) = value {
            return EquationComponentType::Decimal(Decimal { value: -(i.value) });
        } else {
            return EquationComponentType::MinusNode(MinusNode {
                value: Box::new(value),
            });
        }
    }

    fn substitutei(&self, variable: char, value: i128) -> EquationComponentType {
        let value: EquationComponentType =
            EquationComponentType::substitutei(&self.value, variable, value).simplify();
        EquationComponentType::MinusNode(MinusNode {
            value: Box::new(value),
        })
    }
}
