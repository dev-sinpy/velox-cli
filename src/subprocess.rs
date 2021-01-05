use crate::VeloxError;
use std::process::{Child, Command, Stdio};

/// Spawns a new subprocess and returns a process handle.
pub fn exec(cmd: &str, stdout: Stdio, stderr: Stdio) -> Child {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", cmd])
            .spawn()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .stdout(stdout)
            .stderr(stderr)
            .args(&["-c", cmd])
            .spawn()
            .expect("failed to execute process")
    }
}
