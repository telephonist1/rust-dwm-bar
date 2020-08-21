use std::process::Command;

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
