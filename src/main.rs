use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, IsTerminal, Result, Write};

use clap::Parser;

/// uniq(1) alternative with streaming support
///
/// The uniqs utility reads the specified INPUT, and writes only the unique lines that appear in it
/// to OUTPUT. It ignores lines that have already appeared before. If INPUT is a single dash ('-')
/// or absent, standard input is read. If OUTPUT is absent, the standard output is used for output.
#[derive(Parser)]
#[command(version, author)]
struct Args {
    /// Prefix lines by the number of occurrences
    #[arg(short, long)]
    count: bool,

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

    trait WriteAndIsTerminal: Write + IsTerminal {}
    impl<T: Write + IsTerminal> WriteAndIsTerminal for T {}
    let mut output: Box<dyn WriteAndIsTerminal> = match args.output {
        Some(path) => Box::new(File::create(path)?),
        _ => Box::new(stdout().lock()),
    };

    match (args.count, output.is_terminal()) {
        (false, _) => program(&mut input, &mut output),
        (true, true) => count_interactive(&mut input, &mut output),
        (true, false) => count(&mut input, &mut output),
    }
}

fn program(input: &mut dyn BufRead, output: &mut dyn Write) -> Result<()> {
    let mut set = HashMap::new();

    for line in input.lines() {
        use std::collections::hash_map::Entry;
        match set.entry(line?) {
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

fn count_interactive(input: &mut dyn BufRead, output: &mut dyn Write) -> Result<()> {
    use crossterm::{
        cursor::MoveTo,
        style::Print,
        terminal::{size, DisableLineWrap, EnterAlternateScreen, LeaveAlternateScreen},
        QueueableCommand,
    };

    output.queue(EnterAlternateScreen)?.queue(DisableLineWrap)?;

    let mut print = |row: usize, count: u64, key: &str| -> Result<()> {
        // Do nothing if screen is not big enough for this print()
        if row >= size()?.1 as usize {
            return Ok(());
        }

        output
            .queue(MoveTo(0, row as u16))?
            .queue(Print(format!("{:>7} {}", count, key)))?;
        output.flush()?;
        Ok(())
    };

    // id == order of appearance == row number on the screen
    // index : line -> id
    let mut index = HashMap::new();
    // table : id -> (count of occurrences, line)
    let mut table = Vec::new();

    for line in input.lines() {
        use std::collections::hash_map::Entry;
        match index.entry(line?) {
            Entry::Vacant(e) => {
                let id = table.len();
                print(id, 1, e.key())?;

                table.push((1u64, e.key().clone())); // TODO: clone
                e.insert(id);
            }
            Entry::Occupied(e) => {
                let id = *e.get();
                let (ref mut count, ref line) = table[id];
                *count += 1;

                print(id, *count, &line)?;
            }
        }
    }

    output.queue(LeaveAlternateScreen)?;
    for (count, line) in table {
        writeln!(output, "{count:>7} {line}")?;
    }

    Ok(())
}

fn count(input: &mut dyn BufRead, output: &mut dyn Write) -> Result<()> {
    let mut set = BTreeMap::new();

    for line in input.lines() {
        use std::collections::btree_map::Entry;
        match set.entry(line?) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1u64;
            }
            Entry::Vacant(e) => {
                e.insert(1u64);
            }
        }
    }

    for (line, count) in set {
        writeln!(output, "{count:>7} {line}")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{count, program};

    fn t(mut input: &[u8], expected: &str) {
        let mut actual = Vec::new();
        program(&mut input, &mut actual).unwrap();
        assert_eq!(String::from_utf8(actual).unwrap(), expected);
    }

    #[test]
    fn test_abc() {
        t(
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
            "\
aaa
bbb
ccc
",
        )
    }

    #[test]
    fn test_abcabc() {
        t(
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
            "\
a
b
c
",
        )
    }

    fn c(mut input: &[u8], expected: &str) {
        let mut actual = Vec::new();
        count(&mut input, &mut actual).unwrap();
        assert_eq!(String::from_utf8(actual).unwrap(), expected);
    }

    #[test]
    fn test_count() {
        c(
            b"\
a
a
a
b
b
c
c
a
a
b
b
c
",
            r#"      5 a
      4 b
      3 c
"#,
        )
    }
}
