/// Macro to define different expected values for `debug` and `release`
///
/// This is useful when testing logic with differing expected results based on build.
///
/// Basically erogonomifies the following ...
///
/// ```rust
/// # use cfg_if::cfg_if;
/// cfg_if! {
///    if #[cfg(not(debug_assertions))] {
///        let expected = 42; // release build value
///    } else {
///        let expected = 0; // debug build value
///    }
/// }
/// ```
/// with ...
/// ```rust
/// # use test_toolbox::expect;
/// expect! { expected = 42, 0 }
/// ```
///
/// # Features
///
/// * lets you define both `release` and `debug` initialization values for expected variable
///
/// * lets you optionally provide an explicit `type` when defining an expected variable<br><br>
/// \* `release` _initialization value is defined first, followed by the_ `debug` _value_
///
/// # Examples
///
/// * implicit type
///
/// ```rust
/// # use test_toolbox::expect;
/// // 42 is the expected value for release
/// // 0 is the expected value for debug
/// expect! { expected = 42, 0 }
/// ```
///
/// * explict type
///
/// ```rust
/// # use test_toolbox::expect;
/// // 42 is the expected value for release
/// // default is the expected value for debug
/// expect! { expected: usize = 42, Default::default() }
/// ```
#[macro_export]
macro_rules! expect {
    // declare expected variable implicitly typed
    ($var:ident = $rls:expr, $dgb:expr) => {
        cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let $var = $rls;
            } else {
                let $var = $dgb;
            }
        }
    };
    // declare expected variable explicitly typed
    ($var:ident: $typ:ty = $rls:expr, $dgb:expr) => {
        cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let $var: $typ = $rls;
            } else {
                let $var: $typ = $dgb;
            }
        }
    };
}
