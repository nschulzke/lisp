use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use crate::object::Object;

struct EnvImpl {
    parent: Option<Rc<RefCell<EnvImpl>>>,
    vars: HashMap<String, Object>,
}

impl EnvImpl {
    pub fn get(&self, name: &str) -> Option<Object> {
        self.vars.get(name).map(|o| o.clone())
            .or_else(|| self.parent.as_ref().and_then(|p| p.borrow().get(name).clone()))
    }

    pub fn set(&mut self, name: &str, value: Object) {
        self.vars.insert(name.to_string(), value);
    }
}

pub struct Env {
    env: Rc<RefCell<EnvImpl>>,
}

impl Env {
    pub fn base() -> Env {
        let mut vars = HashMap::new();
        vars.insert("+".to_string(), Object::PrimitiveFunction("+".to_string(), 2));
        vars.insert("-".to_string(), Object::PrimitiveFunction("-".to_string(), 2));
        vars.insert("*".to_string(), Object::PrimitiveFunction("*".to_string(), 2));
        vars.insert("/".to_string(), Object::PrimitiveFunction("/".to_string(), 2));
        vars.insert("=".to_string(), Object::PrimitiveFunction("=".to_string(), 2));
        vars.insert("!=".to_string(), Object::PrimitiveFunction("!=".to_string(), 2));
        vars.insert("<".to_string(), Object::PrimitiveFunction("<".to_string(), 2));
        vars.insert(">".to_string(), Object::PrimitiveFunction(">".to_string(), 2));
        vars.insert("<=" .to_string(), Object::PrimitiveFunction("<=".to_string(), 2));
        vars.insert(">=".to_string(), Object::PrimitiveFunction(">=".to_string(), 2));
        vars.insert("progn".to_string(), Object::SpecialForm("progn".to_string(), None));
        vars.insert("let".to_string(), Object::SpecialForm("let".to_string(), Some(2)));
        vars.insert("if".to_string(), Object::SpecialForm("if".to_string(), Some(3)));
        vars.insert("def".to_string(), Object::SpecialForm("def".to_string(), Some(2)));
        vars.insert("fn".to_string(), Object::SpecialForm("fn".to_string(), Some(2)));
        vars.insert("#true".to_string(), Object::Boolean(true));
        vars.insert("#false".to_string(), Object::Boolean(false));
        Env {
            env: Rc::new(RefCell::new(EnvImpl {
                parent: None,
                vars,
            })),
        }
    }

    pub fn extend(&self) -> Env {
        Env {
            env: Rc::new(RefCell::new(EnvImpl {
                parent: Some(self.env.clone()),
                vars: HashMap::new(),
            })),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        self.env.borrow().get(name)
    }

    pub fn set(&mut self, name: &str, value: Object) {
        self.env.borrow_mut().set(name, value)
    }
}
