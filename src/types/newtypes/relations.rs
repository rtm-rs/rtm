// `Relations` are a Vec-newtype
//
// A wrapper that allows us to implement *any* trait. It is a new-type and
// rescinds the orphan rule so we have headroom.
//
// Implementing the [`From`] trait provides these conveniences (`match` branch):
//
//     Err(err) => return err.into_compile_error().into(),
//
// Below the following traits are implemented:
//
// - `Debug` (via `#[derive(...)]`)
// - `Default`
// - `Deref`
// - `DerefMut`
// - `Display`
#[derive(Debug, Clone, PartialEq)]
pub struct Relations<T>(Vec<T>);

impl<T: std::fmt::Debug> Relations<T> {
    pub fn new() -> Relations<T> {
        Relations(Vec::<T>::new())
    }

    #[allow(dead_code)]
    pub fn with_capacity(capacity: usize) -> Relations<T> {
        Relations(Vec::<T>::with_capacity(capacity))
    }
}

impl<T: std::fmt::Debug> Default for Relations<T> {
    fn default() -> Relations<T> {
        Relations::new()
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Relations<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T: std::fmt::Debug> std::ops::Deref for Relations<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T: std::fmt::Debug> std::ops::DerefMut for Relations<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    trait Entity {}

    #[derive(Clone, Debug, PartialEq)]
    struct Model {
        a: String,
        b: String,
    }
    impl Entity for Model {}

    #[test]
    fn relations_are_cloneable() {
        let relations = Relations::<Model>::new();
        let clones = relations.clone();
        assert_eq!(relations, clones);
    }
}
