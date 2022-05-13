pub mod prelude;

mod types;

pub use rtm_attributes as attributes;
pub use rtm_macros as macros;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
