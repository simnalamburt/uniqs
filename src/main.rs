use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Result, Write};

use clap::Parser;

/// uniq(1) alternative with streaming support
///
/// The uniqs utility reads the specified INPUT, and writes only the unique lines that appear in it
/// to OUTPUT. It ignores lines that have already appeared before. If INPUT is a single dash ('-')
/// or absent, standard input is read. If OUTPUT is absent, the standard output is used for output.
#[derive(Parser)]
#[command(version, author)]
struct Args {
    /// Path of the input file (default: stdin)
    input: Option<String>,
    /// Path of the output file (default: stdout)
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut input: Box<dyn BufRead> = match args.input {
        Some(path) if path != "-" => Box::new(BufReader::new(File::open(path)?)),
        _ => Box::new(stdin().lock()),
    };

    let mut output: Box<dyn Write> = match args.output {
        Some(path) => Box::new(File::create(path)?),
        _ => Box::new(stdout().lock()),
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
    fn test_abcabc() {
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
