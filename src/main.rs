use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut read_set = HashSet::new();

    for line in stdin.lock().lines().map(|l| l.unwrap() + "\n") {
        if read_set.contains(&line) {
            continue;
        };

        match io::stdout().write(line.as_bytes()) {
            Err(ref e) if e.kind() == io::ErrorKind::BrokenPipe => return,
            Err(e) => panic!(e),
            Ok(_) => {
                read_set.insert(line);
            }
        };
    }
}
