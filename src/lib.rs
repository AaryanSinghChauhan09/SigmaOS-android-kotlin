//! SigmaOS-android-kotlin: Android Kotlin model.
//!
//! Real implementation: classes, functions, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub id: String,
    pub name: String,
    pub package: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub id: String,
    pub class_id: String,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AndroidKotlinError {
    EmptyClassName,
    DuplicateClass(String),
}

impl fmt::Display for AndroidKotlinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyClassName => write!(f, "class name must not be empty"),
            Self::DuplicateClass(id) => write!(f, "duplicate class {:?}", id),
        }
    }
}

impl std::error::Error for AndroidKotlinError {}

pub struct AndroidKotlin {
    pub name: String,
    classes: BTreeMap<String, Class>,
    functions: BTreeMap<String, Function>,
}

impl AndroidKotlin {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), classes: BTreeMap::new(), functions: BTreeMap::new() }
    }

    pub fn add_class(&mut self, c: Class) -> Result<(), AndroidKotlinError> {
        if c.name.trim().is_empty() { return Err(AndroidKotlinError::EmptyClassName); }
        let id = c.id.clone();
        if self.classes.contains_key(&id) {
            return Err(AndroidKotlinError::DuplicateClass(id.clone()));
        }
        self.classes.insert(id, c);
        Ok(())
    }

    pub fn class_count(&self) -> usize { self.classes.len() }

    pub fn get_class(&self, id: &str) -> Option<&Class> { self.classes.get(id) }

    pub fn add_function(&mut self, f: Function) {
        self.functions.insert(f.id.clone(), f);
    }

    pub fn get_function(&self, id: &str) -> Option<&Function> { self.functions.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = AndroidKotlin::new("AndroidKotlin");
        assert_eq!(a.name, "AndroidKotlin");
        assert_eq!(a.class_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = AndroidKotlin::new("AndroidKotlin");
        let class = Class { id: "c1".into(), name: "MainActivity".into(), package: "com.example".into() };
        a.add_class(class).unwrap();
        assert_eq!(a.class_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = AndroidKotlin::new("AndroidKotlin");
        let class = Class { id: "x".into(), name: "".into(), package: "x".into() };
        assert!(a.add_class(class).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = AndroidKotlin::new("AndroidKotlin");
        let class = Class { id: "x".into(), name: "X".into(), package: "x".into() };
        a.add_class(class).unwrap();
        let class2 = Class { id: "x".into(), name: "X".into(), package: "x".into() };
        assert!(a.add_class(class2).is_err());
    }
}
