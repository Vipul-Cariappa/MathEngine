use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops;

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

#[derive(Debug, Clone)]
pub struct PartEquation {
    pub eq: EquationComponentType,
}

impl PartEquation {
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

// trait OperationNode {
//     fn simplify(self) -> EquationComponentType;
//     fn substitute(self) -> EquationComponentType;
// }

#[derive(Debug, Clone, Copy)]
pub struct Integer {
    pub value: i128,
}

impl Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Decimal {
    pub value: f64,
}

impl Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct MinusNode {
    pub value: Box<EquationComponentType>,
}

impl Display for MinusNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(-{})", self.value)
    }
}
