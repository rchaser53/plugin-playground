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

fn execute() -> Result<Vec<u8>, std::io::Error> {
    let stdout = std::process::Stdio::piped();
    let result = Command::new("ls").stdout(stdout).args(&["-a"]).output()?;
    Ok(result.stdout)
}
