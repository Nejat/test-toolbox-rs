use crate::expect;

macro_rules! nested {
    // declare implicitly typed expected variable
    ($var:ident = $rls:expr, $dgb:expr) => {
        expect! { $var = $rls, $dgb }
        let actual;

        #[cfg(debug_assertions)] {
            actual = $dgb;
        }

        #[cfg(not(debug_assertions))] {
            actual = $rls;
        }

        assert_eq!($var, actual);
    };
    // declare explicitly typed expected variable
    ($var:ident: $typ:ty = $rls:expr, $dgb:expr) => {
        expect! { $var: $typ = $rls, $dgb }
        let actual;

        #[cfg(debug_assertions)] {
            actual = $dgb;
        }

        #[cfg(not(debug_assertions))] {
            actual = $rls;
        }

        assert_eq!($var, actual);
    };
}

#[test]
fn expect_implicit_declaration() {
    nested! { expected = "release", "debug" }
}

#[test]
fn expect_explicit_declaration() {
    nested! { expected: &str = "release", "debug" }
}