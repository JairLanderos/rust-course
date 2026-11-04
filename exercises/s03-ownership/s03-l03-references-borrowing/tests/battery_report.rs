//! Checks for exercise 3.3. You don't need to read this file:
//! Section 11 explains how tests like this one work.

use std::process::Command;

const EXPECTED: [&str; 5] = [
    "Ferris: battery at 80%",
    "Ferris: battery at 95%",
    "Ferris: battery at 65%",
    "Battery went from 65% to 0%",
    "Ferris: battery at 0%",
];

#[test]
fn battery_report() {
    let output = Command::new(env!("CARGO_BIN_EXE_s03-l03-references-borrowing"))
        .output()
        .expect("the program should start");
    assert!(
        output.status.success(),
        "\n\nThe program stopped with an error. Run it with `cargo run -p s03-l03-references-borrowing` to see what happened.\n"
    );

    let stdout = String::from_utf8(output.stdout).expect("the output should be text");
    let lines: Vec<&str> = stdout.lines().collect();
    for (number, expected) in EXPECTED.iter().enumerate() {
        let actual = lines.get(number).copied().unwrap_or("(nothing)");
        assert_eq!(
            actual,
            *expected,
            "\n\nLine {} of the output is wrong.\n  expected: {}\n  got:      {}\n",
            number + 1,
            expected,
            actual
        );
    }
    assert_eq!(
        lines.len(),
        EXPECTED.len(),
        "\n\nThe program printed {} lines; it should print {}.\n",
        lines.len(),
        EXPECTED.len()
    );
}
