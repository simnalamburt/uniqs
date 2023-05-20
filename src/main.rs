use std::io::{stdin, stdout, BufRead, Write};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() -> std::io::Result<()> {
    let mut set = HashMap::new();

    // for each lines of stdin
    let mut stdout = stdout().lock();
    for line in stdin().lock().lines() {
        let line = line?;

        match set.entry(line) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(e) => {
                stdout.write_all(e.key().as_bytes())?;
                stdout.write_all(b"\n")?;
                e.insert(());
            }
        }
    }

    Ok(())
}
