use systemstat::{saturating_sub_bytes, Platform, System};
use std::process::Command;
use std::{thread, time};

pub fn memory() -> String {
    let sys = System::new();
    match sys.memory() {
        Ok(mem) => format!(
            "{}",
            saturating_sub_bytes(mem.total, mem.free)
        ),
        Err(x) => format!("Memory: error: {}", x),
    }
}

pub fn lang() -> String {
    let lang = Command::new("setxkbmap")
        .arg("-query")
        .output()
        .expect("Failed to execute command!");
    let lang = String::from_utf8_lossy(&lang.stdout);
    let lang = lang.split_whitespace();
    let lang: Vec<&str> = lang.collect();
    format!("{}", &lang[5])
}

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

pub fn date_time() -> String {
    chrono::Local::now().format("%a, %d %h %R").to_string()
}

pub fn status() -> String {
    format!(
        " {}  {}  {}  {}",
        memory(),
        lang(),
        volume(),
        date_time(),
    )
}


pub fn run() {
    loop {
        Command::new("xsetroot")
            .arg("-name")
            .arg(status())
            .spawn()
            .expect("Failed to execute command!");
	thread::sleep(time::Duration::from_secs(3));
    }
}
