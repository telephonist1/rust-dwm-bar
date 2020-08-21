mod date_time;
mod lang;
mod memory;
mod volume;
use std::thread;
use std::time;
use std::process::Command;

fn status() -> String {
    format!(
        " {}  {}  {}  {}",
        memory::memory(),
        lang::lang(),
        volume::volume(),
        date_time::date_time()
    )
}

fn main() {
    loop {
        Command::new("xsetroot")
            .arg("-name")
            .arg(status())
            .spawn()
            .expect("Failed to execute command!");
	thread::sleep(time::Duration::from_secs(3));
    }
}
