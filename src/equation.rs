use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops;

use super::math;

trait OperationNode {
    fn simplify(self) -> EquationComponentType;
    fn substitute(self) -> EquationComponentType;
}

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

impl OperationNode for EquationComponentType {
    fn simplify(self) -> EquationComponentType {
        match self {
            EquationComponentType::Integer(i) => i.simplify(),
            EquationComponentType::Decimal(i) => i.simplify(),
            EquationComponentType::VariableNode(i) => i.simplify(),
            EquationComponentType::AddNode(i) => i.simplify(),
            EquationComponentType::SubNode(i) => i.simplify(),
            EquationComponentType::MulNode(i) => i.simplify(),
            EquationComponentType::DivNode(i) => i.simplify(),
            EquationComponentType::PowNode(i) => i.simplify(),
            EquationComponentType::MinusNode(i) => i.simplify(),
        }
    }

    fn substitute(self) -> EquationComponentType {
        match self {
            EquationComponentType::Integer(i) => i.simplify(),
            EquationComponentType::Decimal(i) => i.simplify(),
            EquationComponentType::VariableNode(i) => i.simplify(),
            EquationComponentType::AddNode(i) => i.simplify(),
            EquationComponentType::SubNode(i) => i.simplify(),
            EquationComponentType::MulNode(i) => i.simplify(),
            EquationComponentType::DivNode(i) => i.simplify(),
            EquationComponentType::PowNode(i) => i.simplify(),
            EquationComponentType::MinusNode(i) => i.simplify(),
        }
    }
}

#[derive(Debug)]
pub struct PartEquation {
    pub eq: EquationComponentType,
}

impl PartEquation {
    pub fn simplify(self) -> Self {
        PartEquation {
            eq: self.eq.simplify(),
        }
    }

    pub fn pow(self, exponent: PartEquation) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode(PowNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(exponent.eq),
            }),
        }
    }

    pub fn powi32(self, exponent: i32) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode(PowNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: exponent as i128,
                })),
            }),
        }
    }

    pub fn powi64(self, exponent: i64) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode(PowNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer {
                    value: exponent as i128,
                })),
            }),
        }
    }

    pub fn powi128(self, exponent: i128) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode(PowNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Integer(Integer { value: exponent })),
            }),
        }
    }

    pub fn powf32(self, exponent: f32) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode(PowNode {
                lhs: Box::new(self.eq),
                rhs: Box::new(EquationComponentType::Decimal(Decimal {
                    value: exponent as f64,
                })),
            }),
        }
    }

    pub fn powf64(self, exponent: f64) -> Self {
        PartEquation {
            eq: EquationComponentType::PowNode(PowNode {
                lhs: Box::new(self.eq),
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

#[derive(Debug)]
pub struct Integer {
    pub value: i128,
}

impl Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl OperationNode for Integer {
    fn simplify(self) -> EquationComponentType {
        EquationComponentType::Integer(self)
    }

    fn substitute(self) -> EquationComponentType {
        // TODO: implement
        EquationComponentType::Integer(self)
    }
}

#[derive(Debug)]
pub struct Decimal {
    pub value: f64,
}

impl Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl OperationNode for Decimal {
    fn simplify(self) -> EquationComponentType {
        EquationComponentType::Decimal(self)
    }

    fn substitute(self) -> EquationComponentType {
        // TODO: implement
        EquationComponentType::Decimal(self)
    }
}

#[derive(Debug)]
pub struct VariableNode {
    pub variable: char,
}

impl Display for VariableNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variable)
    }
}

impl OperationNode for VariableNode {
    fn simplify(self) -> EquationComponentType {
        EquationComponentType::VariableNode(self)
    }

    fn substitute(self) -> EquationComponentType {
        // TODO: implement
        EquationComponentType::VariableNode(self)
    }
}

#[derive(Debug)]
pub struct AddNode {
    pub lhs: Box<EquationComponentType>,
    pub rhs: Box<EquationComponentType>,
}

impl Display for AddNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} + {})", self.lhs, self.rhs)
    }
}

impl OperationNode for AddNode {
    fn simplify(self) -> EquationComponentType {
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

    fn substitute(self) -> EquationComponentType {
        // TODO: implement
        EquationComponentType::AddNode(self)
    }
}

#[derive(Debug)]
pub struct SubNode {
    pub lhs: Box<EquationComponentType>,
    pub rhs: Box<EquationComponentType>,
}

impl Display for SubNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} - {})", self.lhs, self.rhs)
    }
}

impl OperationNode for SubNode {
    fn simplify(self) -> EquationComponentType {
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

    fn substitute(self) -> EquationComponentType {
        // TODO: implement
        EquationComponentType::SubNode(self)
    }
}

#[derive(Debug)]
pub struct MulNode {
    pub lhs: Box<EquationComponentType>,
    pub rhs: Box<EquationComponentType>,
}

impl Display for MulNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} * {})", self.lhs, self.rhs)
    }
}

impl OperationNode for MulNode {
    fn simplify(self) -> EquationComponentType {
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

    fn substitute(self) -> EquationComponentType {
        // TODO: implement
        EquationComponentType::MulNode(self)
    }
}

#[derive(Debug)]
pub struct DivNode {
    pub lhs: Box<EquationComponentType>,
    pub rhs: Box<EquationComponentType>,
}

impl Display for DivNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} / {})", self.lhs, self.rhs)
    }
}

impl OperationNode for DivNode {
    fn simplify(self) -> EquationComponentType {
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

    fn substitute(self) -> EquationComponentType {
        // TODO: implement
        EquationComponentType::DivNode(self)
    }
}

#[derive(Debug)]
pub struct PowNode {
    pub lhs: Box<EquationComponentType>, // lhs is the base
    pub rhs: Box<EquationComponentType>, // rhs is the exponent
}

impl Display for PowNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} ^ {})", self.lhs, self.rhs)
    }
}

impl OperationNode for PowNode {
    fn simplify(self) -> EquationComponentType {
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

    fn substitute(self) -> EquationComponentType {
        // TODO: implement
        EquationComponentType::PowNode(self)
    }
}

#[derive(Debug)]
pub struct MinusNode {
    pub value: Box<EquationComponentType>,
}

impl Display for MinusNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(-{})", self.value)
    }
}

impl OperationNode for MinusNode {
    fn simplify(self) -> EquationComponentType {
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

    fn substitute(self) -> EquationComponentType {
        // TODO: implement
        EquationComponentType::MinusNode(self)
    }
}
