use std::time::SystemTime;

#[allow(dead_code)]
pub fn execution_seconds<F>(f: F) -> u64
where
    F: Fn() -> u32,
{
    let start = SystemTime::now();
    (f());
    let end = SystemTime::now();
    end.duration_since(start).unwrap().as_secs()
}
