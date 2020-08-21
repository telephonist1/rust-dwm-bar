use systemstat::{saturating_sub_bytes, Platform, System};

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
