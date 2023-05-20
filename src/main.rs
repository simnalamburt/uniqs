use std::io::BufRead;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
    let mut set = HashMap::new();

    // for each lines of stdin
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();

        match set.entry(line) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(e) => {
                println!("{}", e.key());
                e.insert(());
            }
        }
    }
}
