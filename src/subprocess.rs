use dialoguer::console::style;
use std::process::{Child, Command, Stdio};

/// Spawns a new subprocess and returns a process handle.
pub fn exec<T: std::convert::AsRef<std::path::Path>>(
    cmd: &str,
    cwd: T,
    stdout: Stdio,
    stderr: Stdio,
) -> Child {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir(cwd)
            .args(&["/C", cmd])
            .spawn()
            .unwrap_or_else(|_| {
                panic!(
                    "{} Failed to run command `{}`",
                    style("SubprocessError: ").red().bold(),
                    cmd
                )
            })
    } else {
        Command::new("sh")
            .current_dir(cwd)
            .stdout(stdout)
            .stderr(stderr)
            .args(&["-c", cmd])
            .spawn()
            .unwrap_or_else(|_| {
                panic!(
                    "{} Failed to run command `{}`",
                    style("SubprocessError: ").red().bold(),
                    cmd
                )
            })
    }
}
