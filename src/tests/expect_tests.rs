use crate::expect;

#[test]
fn expect_explicit_declaration() {
    expect! { expected = "release", "debug" }
    let actual;

    #[cfg(debug_assertions)] {
        actual = "debug";
    }

    #[cfg(not(debug_assertions))] {
        actual = "release";
    }

    assert_eq!(expected, actual);
}

#[test]
fn expect_implicit_declaration() {
    expect! { expected: &str = "release", "debug" }

    let actual;

    #[cfg(debug_assertions)] {
        actual = "debug";
    }

    #[cfg(not(debug_assertions))] {
        actual = "release";
    }

    assert_eq!(expected, actual);

}