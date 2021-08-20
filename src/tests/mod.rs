mod actual_nested_tests;
mod actual_tests;
mod capture_tests;
mod expect_nested_tests;
mod expect_tests;
mod version;

/// testing support macro
#[macro_export]
macro_rules! debug {
    ($code:block) => {
        #[cfg(debug_assertions)]
        $code
    }
}

/// testing support macro
#[macro_export]
macro_rules! release {
    ($code:block) => {
        #[cfg(not(debug_assertions))]
        $code
    }
}
