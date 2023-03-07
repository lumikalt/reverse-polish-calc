use crate::parser::CValue;

pub fn add(stack: &mut Vec<CValue>) -> CValue {
    let y = stack.pop().unwrap();
    let x = stack.pop().unwrap();

    match (x, y) {
        (CValue::Double(x), CValue::Double(y)) => CValue::Double(x + y),
        (CValue::Double(x), CValue::Int(y)) => CValue::Double(x + y as f64),
        (CValue::Int(x), CValue::Double(y)) => CValue::Double(x as f64 + y),
        (CValue::Int(x), CValue::Int(y)) => CValue::Int(x + y),
    }
}

pub fn sub(stack: &mut Vec<CValue>) -> CValue {
    let y = stack.pop().unwrap();
    let x = stack.pop().unwrap();

    match (x, y) {
        (CValue::Double(x), CValue::Double(y)) => CValue::Double(x - y),
        (CValue::Double(x), CValue::Int(y)) => CValue::Double(x - y as f64),
        (CValue::Int(x), CValue::Double(y)) => CValue::Double(x as f64 - y),
        (CValue::Int(x), CValue::Int(y)) => CValue::Int(x - y),
    }
}

pub fn mul(stack: &mut Vec<CValue>) -> CValue {
    let y = stack.pop().unwrap();
    let x = stack.pop().unwrap();

    match (x, y) {
        (CValue::Double(x), CValue::Double(y)) => CValue::Double(x * y),
        (CValue::Double(x), CValue::Int(y)) => CValue::Double(x * y as f64),
        (CValue::Int(x), CValue::Double(y)) => CValue::Double(x as f64 * y),
        (CValue::Int(x), CValue::Int(y)) => CValue::Int(x * y),
    }
}

pub fn div(stack: &mut Vec<CValue>) -> CValue {
    let y = stack.pop().unwrap();
    let x = stack.pop().unwrap();

    match (x, y) {
        (CValue::Double(x), CValue::Double(y)) => CValue::Double(x / y),
        (CValue::Double(x), CValue::Int(y)) => CValue::Double(x / y as f64),
        (CValue::Int(x), CValue::Double(y)) => CValue::Double(x as f64 / y),
        (CValue::Int(x), CValue::Int(y)) => CValue::Int(x / y),
    }
}

pub fn pi(_stack: &mut Vec<CValue>) -> CValue {
    CValue::Double(std::f64::consts::PI)
}

pub fn e(_stack: &mut Vec<CValue>) -> CValue {
    CValue::Double(std::f64::consts::E)
}
