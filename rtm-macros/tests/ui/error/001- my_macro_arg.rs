use rtm::attributes::*;
use rtm::macros::*;

pub fn main() {
    my_macro! { "wrong", struct Test; }
}
