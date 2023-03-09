use std::{collections::HashMap, fmt::Display};

use crate::{builtins::*, error::CError};

#[derive(Debug, Clone)]
pub enum CExpression {
    Int(i64),
    Double(f64),
    Op(String),
}

#[derive(Debug, Clone, Copy)]
pub enum CValue {
    Int(i64),
    Double(f64),
}

impl From<CValue> for CExpression {
    fn from(value: CValue) -> Self {
        match value {
            CValue::Double(f) => CExpression::Double(f),
            CValue::Int(n) => CExpression::Int(n),
        }
    }
}

impl From<CExpression> for CValue {
    fn from(value: CExpression) -> Self {
        match value {
            CExpression::Double(f) => CValue::Double(f),
            CExpression::Int(n) => CValue::Int(n),
            _ => panic!("An Op can't be turned into a Value."),
        }
    }
}

impl Display for CValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CValue::Double(num) => write!(f, "{}", num),
            CValue::Int(num) => write!(f, "{}", num),
        }
    }
}

struct Op(fn(&mut Vec<CValue>) -> Result<CValue, CError>);

pub struct Env {
    src: String,
    stack: Vec<CValue>,
    funcs: HashMap<String, Box<Op>>,
}

impl Env {
    pub fn new(src: impl ToString) -> Env {
        Env {
            src: src.to_string(),
            stack: vec![],
            funcs: HashMap::from([
                ("+".to_string(), Box::new(Op(add))),
                ("-".to_string(), Box::new(Op(sub))),
                ("*".to_string(), Box::new(Op(mul))),
                ("/".to_string(), Box::new(Op(div))),
                ("pi".to_string(), Box::new(Op(pi))),
                ("e".to_string(), Box::new(Op(e))),
            ]),
        }
    }

    pub fn tokenize(&self) -> Vec<CExpression> {
        self.src
            .split_whitespace()
            .map(|token| match token.parse::<i64>() {
                Ok(i) => CExpression::Int(i),
                Err(_) => match token.parse::<f64>() {
                    Ok(f) => CExpression::Double(f),
                    Err(_) => CExpression::Op(token.to_string()),
                },
            })
            .collect()
    }

    pub fn interpret(&mut self) -> String {
        match self
            .tokenize()
            .iter()
            .try_for_each(|token| -> Result<(), CError> {
                match token {
                    CExpression::Op(name) => Ok({
                        let Some(op) = self.funcs.get(name) else {
                            return Err(CError::OpNotFound);
                        };

                        let val = (op.0)(&mut self.stack);

                        self.stack.push(CValue::from(val?));
                    }),
                    val => Ok(self.stack.push(CValue::from(val.clone()))),
                }
            }) {
            Err(e) => e.to_string(),
            Ok(()) => match self.stack.pop() {
                None => "".into(),
                Some(val) => val.to_string(),
            },
        }
    }
}
