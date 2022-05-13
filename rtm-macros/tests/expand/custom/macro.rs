#[macro_use]
use rtm::attributes::*;
use rtm::macros::*;

pub fn main() {
    my_macro! { struct Test; }
}
