use rug::ops::Pow;
use rug::{Float, Integer, Rational};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone)]
pub enum Number {
    Integer(Integer),
    Rational(Rational),
    Float(Float),
}

impl Number {
    pub fn pow(&self, exponent: &Number) -> Number {        
        match self {
            Number::Integer(b) => match exponent {
                Number::Integer(e) => Number::pow_integer(b, e),
                Number::Rational(e) => {
                    if e.is_integer() {
                        let (e, _) = e.clone().into_numer_denom();
                        Number::pow_integer(b, &e)
                    } else {
                        Number::pow_float(&Float::with_val(100, b), &Float::with_val(100, e))
                    }
                }
                Number::Float(e) => Number::pow_float(&Float::with_val(100, b), e),
            },
            Number::Rational(b) => match exponent {
                Number::Integer(e) => {
                    if b.is_integer() {
                        let (b, _) = b.clone().into_numer_denom();
                        Number::pow_integer(&b, e)
                    } else {
                        Number::pow_rational(b, e)
                    }
                }
                Number::Rational(e) => {
                    if b.is_integer() && e.is_integer() {
                        let (b, _) = b.clone().into_numer_denom();
                        let (e, _) = e.clone().into_numer_denom();
                        Number::pow_integer(&b, &e)
                    } else {
                        Number::pow_float(&Float::with_val(100, b), &Float::with_val(100, e))
                    }
                }
                Number::Float(e) => Number::pow_float(&Float::with_val(100, b), e),
            },
            Number::Float(b) => match exponent {
                // ???: Check if Float is a integer and type cast it
                Number::Integer(e) => Number::pow_float(b, &Float::with_val(100, e)),
                Number::Rational(e) => Number::pow_float(b, &Float::with_val(100, e)),
                Number::Float(e) => Number::pow_float(b, e),
            },
        }
    }

    fn pow_integer(base: &Integer, exponent: &Integer) -> Number {
        let mut result = Integer::from(base);
        let mut count = Integer::from(1);
        while count < *exponent {
            result *= base;
            count += 1;
        }
        return Number::Integer(result);
    }
    
    fn pow_rational(base: &Rational, exponent: &Integer) -> Number {
        let mut result = Rational::from(base);
        let mut count = Integer::from(1);
        while count < *exponent {
            result *= base;
            count += 1;
        }
        return Number::Rational(result);
    }

    fn pow_float(base: &Float, exponent: &Float) -> Number {
        Number::Float(base.pow(exponent.clone()))
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{:?}", i),
            Number::Rational(i) => write!(f, "{:?}", i),
            Number::Float(i) => write!(f, "{:?}", i),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{}", i),
            Number::Rational(i) => write!(f, "{}", i),
            Number::Float(i) => write!(f, "{}", i),
        }
    }
}

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl From<i128> for Number {
    fn from(value: i128) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl From<u128> for Number {
    fn from(value: u128) -> Self {
        Number::Integer(Integer::from(value))
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number::Float(Float::with_val(100, value))
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::Float(Float::with_val(100, value))
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Number::Integer(lhs) => match other {
                Number::Integer(rhs) => lhs == rhs,
                Number::Rational(rhs) => lhs == rhs,
                Number::Float(rhs) => lhs == rhs,
            },
            Number::Rational(lhs) => match other {
                Number::Integer(rhs) => lhs == rhs,
                Number::Rational(rhs) => lhs == rhs,
                Number::Float(rhs) => lhs == rhs,
            },
            Number::Float(lhs) => match other {
                Number::Integer(rhs) => lhs == rhs,
                Number::Rational(rhs) => lhs == rhs,
                Number::Float(rhs) => lhs == rhs,
            },
        }
    }
}

impl Eq for Number {}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Number::Integer(lhs) => match other {
                Number::Integer(rhs) => lhs.partial_cmp(rhs),
                Number::Rational(rhs) => lhs.partial_cmp(rhs),
                Number::Float(rhs) => lhs.partial_cmp(rhs),
            },
            Number::Rational(lhs) => match other {
                Number::Integer(rhs) => lhs.partial_cmp(rhs),
                Number::Rational(rhs) => lhs.partial_cmp(rhs),
                Number::Float(rhs) => lhs.partial_cmp(rhs),
            },
            Number::Float(lhs) => match other {
                Number::Integer(rhs) => lhs.partial_cmp(rhs),
                Number::Rational(rhs) => lhs.partial_cmp(rhs),
                Number::Float(rhs) => lhs.partial_cmp(rhs),
            },
        }
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Add<&Number> for &Number {
    type Output = Number;

    fn add(self, rhs: &Number) -> Self::Output {
        match self {
            Number::Integer(lhs) => match rhs {
                Number::Integer(rhs) => Number::Integer(lhs.clone() + rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() + rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() + rhs.clone()),
            },
            Number::Rational(lhs) => match rhs {
                Number::Float(rhs) => Number::Float(lhs.clone() + rhs.clone()),
                Number::Integer(rhs) => Number::Rational(lhs.clone() + rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() + rhs.clone()),
            },
            Number::Float(lhs) => match rhs {
                Number::Integer(rhs) => Number::Float(lhs.clone() + rhs.clone()),
                Number::Rational(rhs) => Number::Float(lhs.clone() + rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() + rhs.clone()),
            },
        }
    }
}

impl Sub<&Number> for &Number {
    type Output = Number;

    fn sub(self, rhs: &Number) -> Self::Output {
        match self {
            Number::Integer(lhs) => match rhs {
                Number::Integer(rhs) => Number::Integer(lhs.clone() - rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() - rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() - rhs.clone()),
            },
            Number::Rational(lhs) => match rhs {
                Number::Float(rhs) => Number::Float(lhs.clone() - rhs.clone()),
                Number::Integer(rhs) => Number::Rational(lhs.clone() - rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() - rhs.clone()),
            },
            Number::Float(lhs) => match rhs {
                Number::Integer(rhs) => Number::Float(lhs.clone() - rhs.clone()),
                Number::Rational(rhs) => Number::Float(lhs.clone() - rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() - rhs.clone()),
            },
        }
    }
}

impl Mul<&Number> for &Number {
    type Output = Number;

    fn mul(self, rhs: &Number) -> Self::Output {
        match self {
            Number::Integer(lhs) => match rhs {
                Number::Integer(rhs) => Number::Integer(lhs.clone() * rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() * rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() * rhs.clone()),
            },
            Number::Rational(lhs) => match rhs {
                Number::Float(rhs) => Number::Float(lhs.clone() * rhs.clone()),
                Number::Integer(rhs) => Number::Rational(lhs.clone() * rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() * rhs.clone()),
            },
            Number::Float(lhs) => match rhs {
                Number::Integer(rhs) => Number::Float(lhs.clone() * rhs.clone()),
                Number::Rational(rhs) => Number::Float(lhs.clone() * rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() * rhs.clone()),
            },
        }
    }
}

impl Div<&Number> for &Number {
    type Output = Number;

    fn div(self, rhs: &Number) -> Self::Output {
        match self {
            Number::Integer(lhs) => match rhs {
                Number::Integer(rhs) => Number::Integer(lhs.clone() / rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() / rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() / rhs.clone()),
            },
            Number::Rational(lhs) => match rhs {
                Number::Float(rhs) => Number::Float(lhs.clone() / rhs.clone()),
                Number::Integer(rhs) => Number::Rational(lhs.clone() / rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() / rhs.clone()),
            },
            Number::Float(lhs) => match rhs {
                Number::Integer(rhs) => Number::Float(lhs.clone() / rhs.clone()),
                Number::Rational(rhs) => Number::Float(lhs.clone() / rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() / rhs.clone()),
            },
        }
    }
}

impl Neg for &Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Integer(-lhs.clone()),
            Number::Rational(lhs) => Number::Rational(-lhs.clone()),
            Number::Float(lhs) => Number::Float(-lhs.clone()),
        }
    }
}

impl Add<Number> for Number {
    type Output = Number;

    fn add(self, rhs: Number) -> Self::Output {
        match self {
            Number::Integer(lhs) => match rhs {
                Number::Integer(rhs) => Number::Integer(lhs.clone() + rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() + rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() + rhs.clone()),
            },
            Number::Rational(lhs) => match rhs {
                Number::Float(rhs) => Number::Float(lhs.clone() + rhs.clone()),
                Number::Integer(rhs) => Number::Rational(lhs.clone() + rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() + rhs.clone()),
            },
            Number::Float(lhs) => match rhs {
                Number::Integer(rhs) => Number::Float(lhs.clone() + rhs.clone()),
                Number::Rational(rhs) => Number::Float(lhs.clone() + rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() + rhs.clone()),
            },
        }
    }
}

impl Sub<Number> for Number {
    type Output = Number;

    fn sub(self, rhs: Number) -> Self::Output {
        match self {
            Number::Integer(lhs) => match rhs {
                Number::Integer(rhs) => Number::Integer(lhs.clone() - rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() - rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() - rhs.clone()),
            },
            Number::Rational(lhs) => match rhs {
                Number::Float(rhs) => Number::Float(lhs.clone() - rhs.clone()),
                Number::Integer(rhs) => Number::Rational(lhs.clone() - rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() - rhs.clone()),
            },
            Number::Float(lhs) => match rhs {
                Number::Integer(rhs) => Number::Float(lhs.clone() - rhs.clone()),
                Number::Rational(rhs) => Number::Float(lhs.clone() - rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() - rhs.clone()),
            },
        }
    }
}

impl Mul<Number> for Number {
    type Output = Number;

    fn mul(self, rhs: Number) -> Self::Output {
        match self {
            Number::Integer(lhs) => match rhs {
                Number::Integer(rhs) => Number::Integer(lhs.clone() * rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() * rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() * rhs.clone()),
            },
            Number::Rational(lhs) => match rhs {
                Number::Float(rhs) => Number::Float(lhs.clone() * rhs.clone()),
                Number::Integer(rhs) => Number::Rational(lhs.clone() * rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() * rhs.clone()),
            },
            Number::Float(lhs) => match rhs {
                Number::Integer(rhs) => Number::Float(lhs.clone() * rhs.clone()),
                Number::Rational(rhs) => Number::Float(lhs.clone() * rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() * rhs.clone()),
            },
        }
    }
}

impl Div<Number> for Number {
    type Output = Number;

    fn div(self, rhs: Number) -> Self::Output {
        match self {
            Number::Integer(lhs) => match rhs {
                Number::Integer(rhs) => {
                    Number::Rational(Rational::from((lhs.clone(), rhs.clone())))
                }
                Number::Rational(rhs) => Number::Rational(lhs.clone() / rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() / rhs.clone()),
            },
            Number::Rational(lhs) => match rhs {
                Number::Float(rhs) => Number::Float(lhs.clone() / rhs.clone()),
                Number::Integer(rhs) => Number::Rational(lhs.clone() / rhs.clone()),
                Number::Rational(rhs) => Number::Rational(lhs.clone() / rhs.clone()),
            },
            Number::Float(lhs) => match rhs {
                Number::Integer(rhs) => Number::Float(lhs.clone() / rhs.clone()),
                Number::Rational(rhs) => Number::Float(lhs.clone() / rhs.clone()),
                Number::Float(rhs) => Number::Float(lhs.clone() / rhs.clone()),
            },
        }
    }
}

impl Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Integer(-lhs),
            Number::Rational(lhs) => Number::Rational(-lhs),
            Number::Float(lhs) => Number::Float(-lhs),
        }
    }
}

impl Add<i32> for Number {
    type Output = Number;

    fn add(self, rhs: i32) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Integer(lhs.clone() + rhs),
            Number::Rational(lhs) => Number::Rational(lhs.clone() + rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() + rhs),
        }
    }
}

impl Sub<i32> for Number {
    type Output = Number;

    fn sub(self, rhs: i32) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Integer(lhs.clone() - rhs),
            Number::Rational(lhs) => Number::Rational(lhs.clone() - rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() - rhs),
        }
    }
}

impl Mul<i32> for Number {
    type Output = Number;

    fn mul(self, rhs: i32) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Integer(lhs.clone() * rhs),
            Number::Rational(lhs) => Number::Rational(lhs.clone() * rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() * rhs),
        }
    }
}

impl Div<i32> for Number {
    type Output = Number;

    fn div(self, rhs: i32) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Integer(lhs.clone() / rhs),
            Number::Rational(lhs) => Number::Rational(lhs.clone() / rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() / rhs),
        }
    }
}

impl Add<i64> for Number {
    type Output = Number;

    fn add(self, rhs: i64) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Integer(lhs.clone() + rhs),
            Number::Rational(lhs) => Number::Rational(lhs.clone() + rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() + rhs),
        }
    }
}

impl Sub<i64> for Number {
    type Output = Number;

    fn sub(self, rhs: i64) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Integer(lhs.clone() - rhs),
            Number::Rational(lhs) => Number::Rational(lhs.clone() - rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() - rhs),
        }
    }
}

impl Mul<i64> for Number {
    type Output = Number;

    fn mul(self, rhs: i64) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Integer(lhs.clone() * rhs),
            Number::Rational(lhs) => Number::Rational(lhs.clone() * rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() * rhs),
        }
    }
}

impl Div<i64> for Number {
    type Output = Number;

    fn div(self, rhs: i64) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Integer(lhs.clone() / rhs),
            Number::Rational(lhs) => Number::Rational(lhs.clone() / rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() / rhs),
        }
    }
}

impl Add<f32> for Number {
    type Output = Number;

    fn add(self, rhs: f32) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Float(Float::with_val(100, lhs) + rhs),
            Number::Rational(lhs) => Number::Float(Float::with_val(100, lhs) + rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() + rhs),
        }
    }
}

impl Sub<f32> for Number {
    type Output = Number;

    fn sub(self, rhs: f32) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Float(Float::with_val(100, lhs) - rhs),
            Number::Rational(lhs) => Number::Float(Float::with_val(100, lhs) - rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() - rhs),
        }
    }
}

impl Mul<f32> for Number {
    type Output = Number;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Float(Float::with_val(100, lhs) * rhs),
            Number::Rational(lhs) => Number::Float(Float::with_val(100, lhs) * rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() * rhs),
        }
    }
}

impl Div<f32> for Number {
    type Output = Number;

    fn div(self, rhs: f32) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Float(Float::with_val(100, lhs) / rhs),
            Number::Rational(lhs) => Number::Float(Float::with_val(100, lhs) / rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() / rhs),
        }
    }
}

impl Add<f64> for Number {
    type Output = Number;

    fn add(self, rhs: f64) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Float(Float::with_val(100, lhs) + rhs),
            Number::Rational(lhs) => Number::Float(Float::with_val(100, lhs) + rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() + rhs),
        }
    }
}

impl Sub<f64> for Number {
    type Output = Number;

    fn sub(self, rhs: f64) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Float(Float::with_val(100, lhs) - rhs),
            Number::Rational(lhs) => Number::Float(Float::with_val(100, lhs) - rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() - rhs),
        }
    }
}

impl Mul<f64> for Number {
    type Output = Number;

    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Float(Float::with_val(100, lhs) * rhs),
            Number::Rational(lhs) => Number::Float(Float::with_val(100, lhs) * rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() * rhs),
        }
    }
}

impl Div<f64> for Number {
    type Output = Number;

    fn div(self, rhs: f64) -> Self::Output {
        match self {
            Number::Integer(lhs) => Number::Float(Float::with_val(100, lhs) / rhs),
            Number::Rational(lhs) => Number::Float(Float::with_val(100, lhs) / rhs),
            Number::Float(lhs) => Number::Float(lhs.clone() / rhs),
        }
    }
}
