use std::io;
use std::io::Write;
use std::collections::HashSet;

fn main() {
    let mut read_set = HashSet::new();

    loop {
        let mut line = String::new();

        match io::stdin().read_line(&mut line) {
            Err(e) => panic!(e),
            Ok(n) => {
                if n == 0 {
                    return; // EOF
                }
            }
        };

        if !read_set.contains(&line) {
            match io::stdout().write(line.as_bytes()) {
                Err(ref e) if e.kind() == io::ErrorKind::BrokenPipe => return,
                Err(e) => panic!(e),
                Ok(_) => {
                    read_set.insert(line);
                }
            };
        }
    }
}
