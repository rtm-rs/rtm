pub mod defaults;

use defaults::IntegrationTest;
// pub struct IntegrationTest {
//     pub name: &'static str,
//     pub test_fn: fn(),
//     pub indev: Option<bool>,
// }

// inventory::collect!(IntegrationTest);

// use minitrace_tests::main_runtime;
// use minitrace_tests::main_tokio;
// use tests::IntegrationTest;

fn setup() {
    println!("Setup")
}

fn teardown() {
    println!("Teardown")
}
// NOTE:
// When this code is in src/main.rs, it is executed by `cargo test -- --list`.
// In such cases you can guard it:
// #[cfg(any(feature = "as", feature = "ea", feature = "tk"))]
fn main() {
    // Setup test environment
    setup();

    // Run the tests
    for t in inventory::iter::<IntegrationTest> {
        (t.test_fn)()
    }

    // Teardown test environment
    teardown();
}
