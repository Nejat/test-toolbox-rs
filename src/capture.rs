/// Macro to capture `stdout` and `stderr` during the evaluation of a statement
///
/// ```rust
/// # use test_toolbox::capture;
/// let (out, err) = capture!{{
///     println!("stdout example");
///     eprint!("stderr example");
/// }};
///
/// assert_eq!("stdout example\n", out);
/// assert_eq!("stderr example", err);
/// ```
#[macro_export]
macro_rules! capture {
    ($eval:stmt) => {{
        let mut out = $crate::gag::BufferRedirect::stdout().expect("redirected stdout required for test");
        let mut err = $crate::gag::BufferRedirect::stderr().expect("redirected stderr required for test");

        $eval

        // manually flush buffers; stdout and stderr flush on newline
        <std::io::Stdout as std::io::Write>::flush(&mut std::io::stdout()).expect("to flush stdout");
        <std::io::Stderr as std::io::Write>::flush(&mut std::io::stderr()).expect("to flush stderr");

        let mut stdout = String::new();
        let mut stderr = String::new();

        std::io::Read::read_to_string(&mut out, &mut stdout).expect("to copy captured stdout to string");
        std::io::Read::read_to_string(&mut err, &mut stderr).expect("to copy captured stderr to string");

        (stdout, stderr)
    }}
}
