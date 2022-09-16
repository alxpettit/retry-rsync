use std::io;
use std::process::{Command, ExitStatus, Stdio};

pub fn shell(program: &str, args: &Vec<String>) -> io::Result<ExitStatus> {
    Command::new(program)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
}
