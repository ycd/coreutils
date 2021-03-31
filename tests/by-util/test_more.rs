use crate::common::util::*;

#[test]
fn test_more_no_arg() {
    let (_, mut ucmd) = at_and_ucmd!();
    let result = ucmd.run();
    assert!(!result.success);
}

#[test]
fn test_invalid_input() {
    let (_, mut ucmd) = at_and_ucmd!();
    let result = ucmd.arg("invalid").run();

    println!("stdout: {}, stderr:{}", result.stdout, result.stderr);
    assert!(result
        .stderr
        .contains("more: error: cannot open 'invalid': No such file or directory"));
    assert!(!result.success);

    let (at, mut ucmd) = at_and_ucmd!();
    at.mkdir("abc");
    let result = ucmd.arg("abc").run();

    println!("stdout: {}, stderr:{}", result.stdout, result.stderr);

    assert!(result
        .stderr
        .contains("more: error: cannot read 'abc': Is a directory"));
    assert!(!result.success);
}
