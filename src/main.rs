use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Result, Write};

use clap::Parser;

/// uniq(1) alternative with streaming support
#[derive(Parser, Debug)]
#[command(version, author)]
struct Args {
    /// Path of the input file (default: stdin)
    input: Option<String>,
    /// Path of the output file (default: stdout)
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut input: Box<dyn BufRead> = if let Some(path) = args.input {
        Box::new(BufReader::new(File::open(path)?))
    } else {
        Box::new(stdin().lock())
    };

    let mut output: Box<dyn Write> = if let Some(path) = args.output {
        Box::new(File::create(path)?)
    } else {
        Box::new(stdout().lock())
    };

    program(&mut input, &mut output)
}

fn program(input: &mut dyn BufRead, output: &mut dyn Write) -> std::io::Result<()> {
    let mut set = HashMap::new();

    for line in input.lines() {
        let line = line?;

        match set.entry(line) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(e) => {
                output.write_all(e.key().as_bytes())?;
                output.write_all(b"\n")?;
                e.insert(());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::program;

    fn test(mut input: &[u8], expected: &[u8]) {
        let mut actual = Vec::new();
        program(&mut input, &mut actual).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_abc() {
        test(
            b"\
aaa
aaa
aaa
bbb
bbb
bbb
ccc
ccc
ccc
",
            b"\
aaa
bbb
ccc
",
        )
    }

    #[test]
    fn test_add() {
        test(
            b"\
a
b
c
a
b
c
a
b
c
",
            b"\
a
b
c
",
        )
    }
}
