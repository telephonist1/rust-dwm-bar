use std::process::Command;

pub fn volume() -> String {
    let volume = Command::new("pulsemixer")
        .arg("--get-volume")
        .output()
        .expect("Failed to execute command");
    let volume = String::from_utf8_lossy(&volume.stdout);
    let volume = volume.split_whitespace();
    let volume: Vec<&str> = volume.collect();
    format!("{}%", &volume[0])
}
