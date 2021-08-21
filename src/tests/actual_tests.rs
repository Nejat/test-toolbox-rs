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


#[test]
fn debug_actual_declaration_release_set_to_default() {
    expect! { expected = "", "debug" }
    actual! { DBG actual: String }

    debug! {{
        actual = String::from("debug");
    }}

    assert_eq!(expected, actual);
}

#[test]
fn debug_actual_declaration_release_set_to_value() {
    expect! { expected = "release value", "debug" }
    actual! { DBG actual: String; String::from("release value") }

    debug! {{
        actual = String::from("debug");
    }}

    assert_eq!(expected, actual);
}

#[test]
fn debug_actual_mutable_declaration_release_set_to_value() {
    expect! { expected = "", "debug" }
    actual! { DBG mut actual: String; String::new() }

    debug! {{
        actual.push_str("debug");
    }}

    assert_eq!(expected, actual);
}

#[test]
fn release_actual_mutable_declaration_debug_set_to_value() {
    expect! { expected = "release", "" }
    actual! { RLS mut actual: String; String::new() }

    release! {{
        actual.push_str("release");
    }}

    assert_eq!(expected, actual);
}

#[test]
fn release_actual_declaration_debug_set_to_default() {
    expect! { expected = "release", "" }
    actual! { RLS actual: String }

    release! {{
        actual = String::from("release");
    }}

    assert_eq!(expected, actual);
}

#[test]
fn release_actual_declaration_debug_set_to_value() {
    expect! { expected = "release", "debug value" }
    actual! { RLS actual: String; String::from("debug value") }

    release! {{
        actual = String::from("release");
    }}

    assert_eq!(expected, actual);
}
