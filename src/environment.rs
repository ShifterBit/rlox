use crate::interpreter::RuntimeError;
use crate::token::{Literal, Token};
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &String, value: Literal) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn get(&self, name: Token) -> Result<Literal, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            Ok(self.values.get(&name.lexeme).unwrap().clone())
        } else {
            Err(RuntimeError::new(
                name.clone(),
                format!("Undefined variable {}.", &name.lexeme),
            ))
        }
    }

    pub fn assign(&mut self, name: Token, value: Literal) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            Ok(())
        } else {
            Err(RuntimeError::new(
                name.clone(),
                format!("Undefined variable {}.", &name.lexeme),
            ))
        }
    }
}
