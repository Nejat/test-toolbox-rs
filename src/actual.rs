/// Macro to define different actual variables for `debug` and `release`
///
/// This is useful when testing logic with differing actual variable definitions based on build.
///
/// Basically erogonomifies the following ...
///
/// ```rust
/// # use cfg_if::cfg_if;
/// cfg_if! {
///    if #[cfg(not(debug_assertions))] {
///        let expected = Default::default();
///    } else {
///        let expected = 42;
///    }
/// }
/// cfg_if! {
///    if #[cfg(not(debug_assertions))] {
///        let actual: usize = Default::default();
///    } else {
///        let actual: usize;
///    }
/// }
///
/// #[cfg(debug_assertions)]
/// { actual = sut(); }
///
/// assert_eq!(expected, actual);
/// # #[cfg(debug_assertions)]
/// # fn sut() -> usize { 42 }
/// ```
/// with ..
/// ```rust
/// # use test_toolbox::actual;
/// # use test_toolbox::expect;
/// expect! { expected = Default::default(), 42 }
/// actual! { @dbg actual: usize }
///
/// #[cfg(debug_assertions)]
/// { actual = sut(); }
///
/// assert_eq!(expected, actual);
/// # #[cfg(debug_assertions)]
/// # fn sut() -> usize { 42 }
/// ```
///
/// # Features
///
/// *
/// *
///
/// # Examples
///
/// * debug; uninitialized actual variable
///   release; actual variable initialized to default
///
/// ```rust
/// # use test_toolbox::actual;
/// # use test_toolbox::expect;
/// expect! { expected: usize = Default::default(), 42 }
/// actual! { @dbg actual: usize }
///
/// #[cfg(debug_assertions)]
/// { actual = sut(); }
///
/// assert_eq!(expected, actual);
/// # #[cfg(debug_assertions)]
/// # fn sut() -> usize { 42 }
/// ```
///
/// * release; uninitialized actual variable
///   debug; actual variable initialized to default
///
/// ```rust
/// # use test_toolbox::actual;
/// # use test_toolbox::expect;
/// expect! { expected: usize = 42, Default::default() }
/// actual! { @rls actual: usize }
///
/// #[cfg(not(debug_assertions))]
/// { actual = sut(); }
///
/// assert_eq!(expected, actual);
/// # #[cfg(not(debug_assertions))]
/// # fn sut() -> usize { 42 }
/// ```
///
///  * debug; uninitialized actual variable
///    release; actual variable initialized to a value
///
/// ```rust
/// # use test_toolbox::actual;
/// # use test_toolbox::expect;
/// expect! { expected: usize = 2, 42 }
/// actual! { @dbg actual: usize; 2 }
///
/// #[cfg(debug_assertions)]
/// { actual = sut(); }
///
/// assert_eq!(expected, actual);
/// # #[cfg(debug_assertions)]
/// # fn sut() -> usize { 42 }
/// ```
///
/// * release; uninitialized actual variable
///   debug; actual variable initialized to a value
///
/// ```rust
/// # use test_toolbox::actual;
/// # use test_toolbox::expect;
/// expect! { expected: usize = 42, 2 }
/// actual! { @rls actual: usize; 2 }
///
/// #[cfg(not(debug_assertions))]
/// { actual = sut(); }
///
/// assert_eq!(expected, actual);
/// # #[cfg(not(debug_assertions))]
/// # fn sut() -> usize { 42 }
/// ```
///
/// * debug; mutable actual variable initialized to a value
///   release; immutable actual variable initialized to a value
///
/// ```rust
/// # use test_toolbox::actual;
/// # use test_toolbox::expect;
/// expect! { expected = "", "Forty Two" }
/// actual! { @dbg mut actual: String; String::new() }
///
/// #[cfg(debug_assertions)]
/// { actual.push_str(&sut()); }
///
/// assert_eq!(expected, &actual);
/// # #[cfg(debug_assertions)]
/// # fn sut() -> String { String::from("Forty Two") }
/// ```
///
/// * release; mutable actual variable initialized to a value
///   debug; immutable actual variable initialized to a value
///
/// ```rust
/// # use test_toolbox::actual;
/// # use test_toolbox::expect;
/// expect! { expected = "Forty Two", "" }
/// actual! { @rls mut actual: String; String::new() }
///
/// #[cfg(not(debug_assertions))]
/// { actual.push_str(&sut()); }
///
/// assert_eq!(expected, &actual);
/// # #[cfg(not(debug_assertions))]
/// # fn sut() -> String { String::from("Forty Two") }
/// ```
#[macro_export]
macro_rules! actual {
    // debug; uninitialized actual variable
    // release; actual variable initialized to default
    (@dbg $var:ident: $typ:ty) => {
        $crate::cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let $var: $typ = Default::default();
            } else {
                let $var: $typ;
            }
        }
    };
    // release; uninitialized actual variable
    // debug; actual variable initialized to default
    (@rls $var:ident: $typ:ty) => {
        $crate::cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let $var: $typ;
            } else {
                let $var: $typ = Default::default();
            }
        }
    };
    // debug; uninitialized actual variable
    // release; actual variable initialized to a value
    (@dbg $var:ident: $typ:ty; $val:expr) => {
        $crate::cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let $var: $typ = $val;
            } else {
                let $var: $typ;
            }
        }
    };
    // release; uninitialized actual variable
    // debug; actual variable initialized to a value
    (@rls $var:ident: $typ:ty; $val:expr) => {
        $crate::cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let $var: $typ;
            } else {
                let $var: $typ = $val;
            }
        }
    };
    // debug; mutable actual variable initialized to a value
    // release; immutable actual variable initialized to a value
    (@dbg mut $var:ident: $typ:ty; $exp:expr) => {
        $crate::cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let $var = $exp;
            } else {
                let mut $var = $exp;
            }
        }
    };
    // release; mutable actual variable initialized to a value
    // debug; immutable actual variable initialized to a value
    (@rls mut $var:ident: $typ:ty; $exp:expr) => {
        $crate::cfg_if::cfg_if! {
            if #[cfg(not(debug_assertions))] {
                let mut $var = $exp;
            } else {
                let $var = $exp;
            }
        }
    };
}
