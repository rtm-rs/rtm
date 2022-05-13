use rtm::prelude::*;

#[test]
#[cfg(not(feature = "ci"))]
pub fn expand_default_dev() {
    // To generate macro result files
    macrotest::expand("tests/expand/default/*.rs");
}

#[test]
#[cfg(not(feature = "ci"))]
pub fn expand_custom_dev() {
    // Generate macro result files
    macrotest::expand("tests/expand/custom/*.rs");
}

#[test]
#[cfg(not(feature = "ci"))]
pub fn expand_regression_dev() {
    // Generate macro result files
    macrotest::expand("tests/expand/regression/*.rs");
}

#[test]
#[cfg(feature = "ci")]
pub fn expand_default_ci() {
    // Do not generate macro result files
    macrotest::expand_without_refresh("tests/expand/default/*.rs");
}

#[test]
#[cfg(feature = "ci")]
pub fn expand_custom_ci() {
    // Do not generate macro result files
    macrotest::expand_without_refresh("tests/expand/custom/*.rs");
}

#[test]
#[cfg(feature = "ci")]
pub fn expand_regression_ci() {
    // Do not generate macro result files
    macrotest::expand("tests/expand/regression/*.rs");
}
