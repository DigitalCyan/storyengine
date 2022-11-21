use std::io::{stdin, BufRead};

pub fn get_usize_from_console() -> usize {
    loop {
        let mut buf = String::new();
        stdin().lock().read_line(&mut buf).unwrap();

        buf = buf.trim().to_string();

        let result = buf.parse::<usize>();

        if let Ok(n) = result {
            return n;
        }
    }
}