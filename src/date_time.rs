extern crate chrono;

pub fn date_time() -> String {
    chrono::Local::now().format("%a, %d %h %R").to_string()
}
