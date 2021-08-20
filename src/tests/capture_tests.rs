use crate::capture;

#[test]
fn capture_combined_stdout_and_stderr() {
    let expected_err = "stderr newline\nstderr no newline";
    let expected_out = "stdout no newline, stdout newline\n";

    let (out, err) = capture! {{
        eprintln!("stderr newline");
        print!("stdout no newline, ");
        eprint!("stderr no newline");
        println!("stdout newline");
    }};

    assert_eq!(expected_err, err);
    assert_eq!(expected_out, out);
}

#[test]
fn capture_stderr_newline() {
    let expected_err = "stderr newline\n";
    let expected_out = "";

    let (out, err) = capture! {{
        eprintln!("stderr newline");
    }};

    assert_eq!(expected_err, err);
    assert_eq!(expected_out, out);
}

#[test]
fn capture_stderr_no_newline() {
    let expected_err = "stderr no newline";
    let expected_out = "";

    let (out, err) = capture! {{
        eprint!("stderr no newline");
    }};

    assert_eq!(expected_err, err);
    assert_eq!(expected_out, out);
}

#[test]
fn capture_stdout_no_newline() {
    let expected_err = "";
    let expected_out = "stdout no newline";

    let (out, err) = capture! {{
        print!("stdout no newline");
    }};

    assert_eq!(expected_err, err);
    assert_eq!(expected_out, out);
}

#[test]
fn capture_stdout_newline() {
    let expected_err = "";
    let expected_out = "stdout newline\n";

    let (out, err) = capture! {{
        println!("stdout newline");
    }};

    assert_eq!(expected_err, err);
    assert_eq!(expected_out, out);
}
