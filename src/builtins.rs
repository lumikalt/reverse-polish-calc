use crate::{
    error::CError,
    parser::CValue::{self, Double as CDouble, Int as CInt},
};

pub fn add(stack: &mut Vec<CValue>) -> Result<CValue, CError> {
    let [y, x] = pop_n::<2>(&mut stack.clone())?;

    Ok(match (x, y) {
        (CDouble(x), CDouble(y)) => CDouble(x + y),
        (CDouble(x), CInt(y)) => CDouble(x + y as f64),
        (CInt(x), CDouble(y)) => CDouble(x as f64 + y),
        (CInt(x), CInt(y)) => CInt(x + y),
    })
}

pub fn sub(stack: &mut Vec<CValue>) -> Result<CValue, CError> {
    let [y, x] = pop_n::<2>(&mut stack.clone())?;

    Ok(match (x, y) {
        (CDouble(x), CDouble(y)) => CDouble(x - y),
        (CDouble(x), CInt(y)) => CDouble(x - y as f64),
        (CInt(x), CDouble(y)) => CDouble(x as f64 - y),
        (CInt(x), CInt(y)) => CInt(x - y),
    })
}

pub fn mul(stack: &mut Vec<CValue>) -> Result<CValue, CError> {
    let [y, x] = pop_n::<2>(&mut stack.clone())?;

    Ok(match (x, y) {
        (CDouble(x), CDouble(y)) => CDouble(x * y),
        (CDouble(x), CInt(y)) => CDouble(x * y as f64),
        (CInt(x), CDouble(y)) => CDouble(x as f64 * y),
        (CInt(x), CInt(y)) => CInt(x * y),
    })
}

pub fn div(stack: &mut Vec<CValue>) -> Result<CValue, CError> {
    let [y, x] = pop_n::<2>(&mut stack.clone())?;

    Ok(match (x, y) {
        (CDouble(x), CDouble(y)) => CDouble(x / y),
        (CDouble(x), CInt(y)) => CDouble(x / y as f64),
        (CInt(x), CDouble(y)) => CDouble(x as f64 / y),
        (CInt(x), CInt(y)) => CInt(x / y),
    })
}

pub fn pi(_stack: &mut Vec<CValue>) -> Result<CValue, CError> {
    Ok(CDouble(std::f64::consts::PI))
}

pub fn e(_stack: &mut Vec<CValue>) -> Result<CValue, CError> {
    Ok(CDouble(std::f64::consts::E))
}

fn pop_n<const N: usize>(stack: &mut Vec<CValue>) -> Result<[CValue; N], CError> {
    let vals = [CValue::Int(0); N];
    let mut index = 0;

    vals.try_map(|_val| {
        index += 1;
        match stack.pop() {
            None => Err(CError::IncorrectNumArgs {
                expected: N,
                found: index - 1,
            }),
            Some(val) => Ok(val),
        }
    })
}
