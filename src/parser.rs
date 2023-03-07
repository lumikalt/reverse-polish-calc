use std::{collections::HashMap, fmt::Display, vec};

use crate::builtins::*;

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
            Self::Double(num) => write!(f, "{}", num),
            Self::Int(num) => write!(f, "{}", num),
        }
    }
}

struct Op(fn(&mut Vec<CValue>) -> CValue);

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
            .map(|token| match token {
                num if num.starts_with(|c: char| c.is_numeric() || c == '.') => {
                    if num.contains('.') {
                        CExpression::Double(num.parse().unwrap())
                    } else {
                        CExpression::Int(num.parse().unwrap())
                    }
                }
                op => CExpression::Op(op.into()),
            })
            .collect()
    }

    pub fn interpret(&mut self) -> CValue {
        self.tokenize().iter().for_each(|token| match token {
            CExpression::Op(name) => {
                let op = self.funcs.get(name).unwrap();

                let val = (op.0)(&mut self.stack);

                self.stack.push(CValue::from(val))
            }
            val => self.stack.push(CValue::from(val.clone())),
        });
        self.stack.pop().unwrap()
    }
}
