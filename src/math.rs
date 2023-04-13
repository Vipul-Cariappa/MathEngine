#[derive(Debug)]
pub enum MathError {
    ZeroDivisionError,
    EquationMismatchError,
    InternalError,
}

pub fn powi128(base: i128, exponent: i128) -> i128 {
    let mut result: i128 = 1;
    for _i in 1..=exponent {
        result *= base;
    }

    return result;
}

pub fn powi64(base: i64, exponent: i64) -> i64 {
    let mut result: i64 = 1;
    for _i in 1..=exponent {
        result *= base;
    }

    return result;
}

pub fn powf64(base: f64, exponent: f64) -> f64 {
    internal_powf64(base, exponent, 0.00000000000000001f64)
}

fn internal_powf64(base: f64, power: f64, precision: f64) -> f64 {
    let square = |x| x * x;

    if (power == f64::INFINITY) || (base == f64::INFINITY) {
        return f64::INFINITY;
    }

    if power < 0f64 {
        return 1f64 / internal_powf64(base, -power, precision);
    }

    if power >= 10f64 {
        return square(internal_powf64(base, power / 2f64, precision / 2f64));
    }

    if power >= 1f64 {
        return base * internal_powf64(base, power - 1f64, precision);
    }

    if precision >= 1f64 {
        return base.sqrt();
    }

    return internal_powf64(base, power * 2f64, precision * 2f64).sqrt();
}
