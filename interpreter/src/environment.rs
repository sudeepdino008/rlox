use std::{collections::HashMap, rc::Rc};

use rustcore::Shared;

use crate::result::IResult;

pub type EnvironmentRef = Shared<Environment>;

pub struct Environment {
    parent: Option<EnvironmentRef>,
    bindings: HashMap<String, Rc<IResult>>,
}

impl Environment {
    pub fn new() -> EnvironmentRef {
        Shared::new(Environment {
            parent: None,
            bindings: HashMap::new(),
        })
    }

    pub fn new_with_parent(parent: EnvironmentRef) -> EnvironmentRef {
        Shared::new(Environment {
            parent: Some(parent),
            bindings: HashMap::new(),
        })
    }

    pub fn is_binded(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
            || (self.parent.is_some() && self.parent.as_ref().unwrap().is_binded(name))
    }

    pub fn get(&self, name: &str) -> Option<Rc<IResult>> {
        self.bindings
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref().and_then(|p| p.get(name)))
    }

    pub fn declare_and_init(&mut self, name: &str, value: IResult) {
        self.bindings.insert(name.to_string(), Rc::new(value));
    }

    pub fn declare(&mut self, name: &str) {
        self.bindings
            .insert(name.to_string(), Rc::new(IResult::None));
    }

    pub fn assign(&mut self, name: &str, value: IResult) -> bool {
        if self.bindings.contains_key(name) {
            self.bindings.insert(name.to_string(), Rc::new(value));
            true
        } else {
            self.parent
                .as_ref()
                .map_or(false, |p| p.borrow_mut().assign(name, value))
        }
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        //println!("dropping out of scope");
        self.bindings.clear();
    }
}
