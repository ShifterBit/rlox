use crate::interpreter::RuntimeError;
use crate::token::{Literal, Token};
use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Environment {
        Environment {
            enclosing: enclosing,
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
            match &self.enclosing {
                Some(s) => return s.get(name),
                None => Err(RuntimeError::new(
                    name.clone(),
                    format!("Undefined variable {}.", &name.lexeme),
                )),
            }
        }
    }

    pub fn assign(&mut self, name: Token, value: Literal) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            Ok(())
        } else if self.enclosing.is_some() {
            let enclosing_env = self.enclosing.clone();
            enclosing_env.unwrap().assign(name, value)
        } else {
            Err(RuntimeError::new(
                name.clone(),
                format!("Undefined variable {}.", &name.lexeme),
            ))
        }
    }
}
