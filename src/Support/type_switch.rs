use std::{any::TypeId, collections::HashMap};

/// Advanced type switching
pub struct TypeSwitch<T> {
    switch_cases: HashMap<TypeId, T>,
}

impl<T> TypeSwitch<T> {
    /// Creates an empty `TypeSwitch`
    pub fn new() -> Self {
        Self {
            switch_cases: HashMap::new()
        }
    }

    /// adds an `TypeSwitch` case
    pub fn case(&mut self, id: TypeId, case: T) {
        self.switch_cases.insert(id, case);
    }

    /// "Switches
    pub fn switch(&mut self, id: TypeId) -> Option<&T> {
        for (type_id, value) in &self.switch_cases {
            if *type_id == id {
                return Some(&value);
            }
        }

        return None;
    }
}