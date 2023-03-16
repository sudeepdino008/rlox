use std::{any::Any, collections::HashMap, rc::Rc};

use crate::result::IResult;

pub(crate) struct Environment {
    bindings: HashMap<String, Rc<dyn Any>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            bindings: HashMap::new(),
        }
    }

    pub fn get<T: Any>(&self, name: &str) -> Option<&T> {
        self.bindings.get(name).and_then(|v| v.downcast_ref::<T>())
    }

    pub fn insert_bind<T: Any>(&mut self, name: &str, value: T) {
        self.bindings.insert(name.to_string(), Rc::new(value));
    }

    pub fn insert(&mut self, name: &str) {
        self.bindings
            .insert(name.to_string(), Rc::new(IResult::None));
    }
}
