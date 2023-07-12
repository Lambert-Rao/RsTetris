use std::io::Write;

pub fn log(s: &str) {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("log.txt")
        .unwrap();
    f.write(s.as_bytes()).unwrap();
}