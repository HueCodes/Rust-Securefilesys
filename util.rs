use std::time::Instant;

pub fn time_it<T, F: FnOnce() -> T>(label: &str, f: F) -> T {
    let start = Instant::now();
    let result = f();
    println!("{label} took {:?}", start.elapsed());
    result
}