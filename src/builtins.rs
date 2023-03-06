use crate::parser::CValue;

pub fn add(xs: &mut Vec<CValue>) -> CValue {
    let x = xs.pop().unwrap();
    let y = xs.pop().unwrap();

    match (x, y) {
        (CValue::Double(x), CValue::Double(y)) => CValue::Double(x + y),
        (CValue::Double(x), CValue::Int(y)) => CValue::Double(x + y as f64),
        (CValue::Int(x), CValue::Double(y)) => CValue::Double(x as f64 + y),
        (CValue::Int(x), CValue::Int(y)) => CValue::Int(x + y),
    }
}

pub fn sub(xs: &mut Vec<CValue>) -> CValue {
    let x = xs.pop().unwrap();
    let y = xs.pop().unwrap();

    match (x, y) {
        (CValue::Double(x), CValue::Double(y)) => CValue::Double(x - y),
        (CValue::Double(x), CValue::Int(y)) => CValue::Double(x - y as f64),
        (CValue::Int(x), CValue::Double(y)) => CValue::Double(x as f64 - y),
        (CValue::Int(x), CValue::Int(y)) => CValue::Int(x - y),
    }
}

pub fn mul(xs: &mut Vec<CValue>) -> CValue {
    let x = xs.pop().unwrap();
    let y = xs.pop().unwrap();

    match (x, y) {
        (CValue::Double(x), CValue::Double(y)) => CValue::Double(x * y),
        (CValue::Double(x), CValue::Int(y)) => CValue::Double(x * y as f64),
        (CValue::Int(x), CValue::Double(y)) => CValue::Double(x as f64 * y),
        (CValue::Int(x), CValue::Int(y)) => CValue::Int(x * y),
    }
}

pub fn div(xs: &mut Vec<CValue>) -> CValue {
    let x = xs.pop().unwrap();
    let y = xs.pop().unwrap();

    match (x, y) {
        (CValue::Double(x), CValue::Double(y)) => CValue::Double(x / y),
        (CValue::Double(x), CValue::Int(y)) => CValue::Double(x / y as f64),
        (CValue::Int(x), CValue::Double(y)) => CValue::Double(x as f64 / y),
        (CValue::Int(x), CValue::Int(y)) => CValue::Int(x / y),
    }
}
