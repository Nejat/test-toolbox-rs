use crate::actual;
use crate::debug;
use crate::expect;
use crate::release;

macro_rules! nested {
    // debug; uninitialized actual variable
    // release; actual variable initialized to default
    (DBG $var:ident: $typ:ty) => {
        expect! { expected = "", "debug" }
        actual! { DBG $var: $typ }

        debug! {{
            $var = String::from("debug");
        }}

        assert_eq!(expected, $var);
    };
    // release; uninitialized actual variable
    // debug; actual variable initialized to default
    (RLS $var:ident: $typ:ty) => {
        expect! { expected = "release", "" }
        actual! { RLS $var: $typ }

        release! {{
            $var = String::from("release");
        }}

        assert_eq!(expected, $var);
    };
    // debug; uninitialized actual variable
    // release; actual variable initialized to a value
    (DBG $var:ident: $typ:ty; $val:expr) => {
        expect! { expected = "release value", "debug" }
        actual! { DBG $var: $typ; $val }

        debug! {{
            $var = String::from("debug");
        }}

        assert_eq!(expected, $var);
    };
    // release; uninitialized actual variable
    // debug; actual variable initialized to a value
    (RLS $var:ident: $typ:ty; $val:expr) => {
        expect! { expected = "release", "debug value" }
        actual! { RLS $var: $typ; $val }

        release! {{
            $var = String::from("release");
        }}

        assert_eq!(expected, $var);
    };
    // debug; mutable actual variable initialized to a value
    // release; immutable actual variable initialized to a value
    (DBG mut $var:ident: $typ:ty; $exp:expr) => {
        expect! { expected = "", "debug" }
        actual! { DBG mut $var: $typ; $exp }

        debug! {{
            $var.push_str("debug");
        }}

        assert_eq!(expected, $var);
    };
    // release; mutable actual variable initialized to a value
    // debug; immutable actual variable initialized to a value
    (RLS mut $var:ident: $typ:ty; $exp:expr) => {
        expect! { expected = "release", "" }
        actual! { RLS mut $var: $typ; $exp }

        release! {{
            $var.push_str("release");
        }}

        assert_eq!(expected, $var);
    };
}

#[test]
fn debug_actual_declaration_release_set_to_default() {
    nested! { DBG actual: String }
}

#[test]
fn debug_actual_declaration_release_set_to_value() {
    nested! { DBG actual: String; String::from("release value") }
}

#[test]
fn debug_actual_mutable_declaration_release_set_to_value() {
    nested! { DBG mut actual: String; String::new() }
}

#[test]
fn release_actual_mutable_declaration_debug_set_to_value() {
    nested! { RLS mut actual: String; String::new() }
}

#[test]
fn release_actual_declaration_debug_set_to_default() {
    nested! { RLS actual: String }
}

#[test]
fn release_actual_declaration_debug_set_to_value() {
    nested! { RLS actual: String; String::from("debug value") }
}
