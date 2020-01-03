use std::process::Command;
use std::str::from_utf8;

fn main() {
    match execute() {
        Ok(ret) => {
            dbg!(from_utf8(&ret));
        }
        Err(err) => panic!(err),
    };
}

fn execute() -> Result<Vec<u8>, String> {
    let stdout = std::process::Stdio::piped();
    let result = Command::new("ls")
        .stdout(stdout)
        .args(&["-z"])
        .output()
        .unwrap();
    if result.status.success() {
        Ok(result.stdout)
    } else {
        Err(from_utf8(&result.stderr).unwrap().to_string())
    }
}
