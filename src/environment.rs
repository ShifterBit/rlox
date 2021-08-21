use crate::interpreter::RuntimeError;
use crate::token::{Literal, Token};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::default(),
        }
    }

    pub fn from(enclosing: Environment) -> Environment {
        Environment {
            enclosing: Some(Rc::new(RefCell::new(enclosing))),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &String, value: Literal) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn get(&self, name: Token) -> Result<Literal, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            Ok(self.values.get(&name.lexeme).unwrap().to_owned())
        } else if self.enclosing.is_some() {
            self.enclosing.as_ref().unwrap().borrow().get(name)
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
        } else if self.enclosing.is_some() {
            self.enclosing
                .as_mut()
                .unwrap()
                .borrow_mut()
                .assign(name, value)
        } else {
            Err(RuntimeError::new(
                name.clone(),
                format!("Undefined variable {}.", &name.lexeme),
            ))
        }
    }
}
