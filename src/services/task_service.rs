use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};

#[cfg(target_os = "windows")]
pub fn task_exists(task_name: &str) -> bool {
    let output = Command::new("schtasks")
        .arg("/query")
        .arg("/tn")
        .arg(task_name)
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute schtasks command");

    let output_text = String::from_utf8_lossy(&output.stdout);

    output_text.contains(task_name)
}

#[cfg(target_os = "windows")]
pub fn task_delete(task_name: &str) {
    let _ = Command::new("schtasks")
        .arg("/delete")
        .arg("/tn")
        .arg(task_name)
        .arg("-f")
        .stdout(Stdio::null())
        .spawn();
}

#[cfg(target_os = "windows")]
pub fn task_execute(path: &str) {
    let _ = Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg("/b")
        .arg("watcher_backup.exe")
        .arg("-p")
        .arg(path)
        .arg("-f")
        .arg("n")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(0x08000000)
        .spawn();
}

#[cfg(target_os = "windows")]
pub fn task_init(path: &str) {
    let _ = Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg("/b")
        .arg("watcher_backup.exe")
        .arg("-p")
        .arg(path)
        .arg("-f")
        .arg("y")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(0x08000000)
        .spawn();
}