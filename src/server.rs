use std::process::{Child, Command, Stdio};

pub fn start_dev_server() -> Child {
    println!("Starting dev server for frontend...");
    let process = Command::new("sh")
        .current_dir("web/")
        .stdout(Stdio::piped())
        .arg("-c")
        .arg("yarn run dev")
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to start dev server");

    println!("server started");
    process
}
