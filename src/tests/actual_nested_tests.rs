use crate::actual;
use crate::expect;

/// testing support macro
macro_rules! debug {
    ($code:block) => {
        #[cfg(debug_assertions)]
        $code
    }
}

/// testing support macro
macro_rules! release {
    ($code:block) => {
        #[cfg(not(debug_assertions))]
        $code
    }
}

macro_rules! nested {
    // debug; uninitialized actual variable
    // release; actual variable initialized to default
    (@dbg $var:ident: $typ:ty) => {
        expect! { expected = "", "debug" }
        actual! { @dbg $var: $typ }

        debug! {{
            $var = String::from("debug");
        }}

        assert_eq!(expected, $var);
    };
    // release; uninitialized actual variable
    // debug; actual variable initialized to default
    (@rls $var:ident: $typ:ty) => {
        expect! { expected = "release", "" }
        actual! { @rls $var: $typ }

        release! {{
            $var = String::from("release");
        }}

        assert_eq!(expected, $var);
    };
    // debug; uninitialized actual variable
    // release; actual variable initialized to a value
    (@dbg $var:ident: $typ:ty; $val:expr) => {
        expect! { expected = "release value", "debug" }
        actual! { @dbg $var: $typ; $val }

        debug! {{
            $var = String::from("debug");
        }}

        assert_eq!(expected, $var);
    };
    // release; uninitialized actual variable
    // debug; actual variable initialized to a value
    (@rls $var:ident: $typ:ty; $val:expr) => {
        expect! { expected = "release", "debug value" }
        actual! { @rls $var: $typ; $val }

        release! {{
            $var = String::from("release");
        }}

        assert_eq!(expected, $var);
    };
    // debug; mutable actual variable initialized to a value
    // release; immutable actual variable initialized to a value
    (@dbg mut $var:ident: $typ:ty; $exp:expr) => {
        expect! { expected = "", "debug" }
        actual! { @dbg mut $var: $typ; $exp }

        debug! {{
            $var.push_str("debug");
        }}

        assert_eq!(expected, $var);
    };
    // release; mutable actual variable initialized to a value
    // debug; immutable actual variable initialized to a value
    (@rls mut $var:ident: $typ:ty; $exp:expr) => {
        expect! { expected = "release", "" }
        actual! { @rls mut $var: $typ; $exp }

        release! {{
            $var.push_str("release");
        }}

        assert_eq!(expected, $var);
    };
}

#[test]
fn debug_actual_declaration_release_set_to_default() {
    nested! { @dbg actual: String }
}

#[test]
fn debug_actual_declaration_release_set_to_value() {
    nested! { @dbg actual: String; String::from("release value") }
}

#[test]
fn debug_actual_mutable_declaration_release_set_to_value() {
    nested! { @dbg mut actual: String; String::new() }
}

#[test]
fn release_actual_mutable_declaration_debug_set_to_value() {
    nested! { @rls mut actual: String; String::new() }
}

#[test]
fn release_actual_declaration_debug_set_to_default() {
    nested! { @rls actual: String }
}

#[test]
fn release_actual_declaration_debug_set_to_value() {
    nested! { @rls actual: String; String::from("debug value") }
}
