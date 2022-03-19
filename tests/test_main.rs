mod util;
use assert_cmd::Command;

#[test]
fn test_main() {
    let in_files = vec!["tests/data/1_hello.lox"];

    for in_file in in_files {
        let mut run_main = Command::cargo_bin("rlox").unwrap();
        run_main.arg(in_file).assert().success();
    }
}
