use std::collections::HashMap;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, IsTerminal, Result, Write};

use clap::{builder::ValueHint, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use indexmap::IndexMap;

/// uniq(1) alternative with streaming support
///
/// The uniqs utility reads the specified INPUT, and writes only the unique lines that appear in it
/// to OUTPUT. It ignores lines that have already appeared before. If INPUT is a single dash ('-')
/// or absent, standard input is read. If OUTPUT is absent, the standard output is used for output.
#[derive(Parser)]
#[command(version, author, args_conflicts_with_subcommands = true)]
struct Args {
    /// Prefix lines by the number of occurrences
    #[arg(short, long)]
    count: bool,

    /// Path of the input file (default: stdin)
    #[arg(value_hint = ValueHint::FilePath, allow_hyphen_values = true)]
    input: Option<String>,
    /// Path of the output file (default: stdout)
    #[arg(value_hint = ValueHint::FilePath)]
    output: Option<String>,

    /// Subcommands
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate shell completion script for specified shell
    #[command(arg_required_else_help = true)]
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle shell completion
    if let Some(Commands::Completion { shell }) = args.command {
        let mut cmd = Args::command();
        let bin_name = env!("CARGO_PKG_NAME");
        generate(shell, &mut cmd, bin_name, &mut stdout());
        return Ok(());
    }

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
    use std::time::{Duration, Instant};

    use crossterm::{
        cursor::MoveTo,
        style::Print,
        terminal::{size, DisableLineWrap, EnterAlternateScreen, LeaveAlternateScreen},
        QueueableCommand,
    };

    output.queue(EnterAlternateScreen)?.queue(DisableLineWrap)?;

    // line -> count of occurrences
    let mut map = IndexMap::new();

    let mut last_rendered_time = Instant::now();
    let mut last_rendered_size = 0;
    let inverse_fps = Duration::from_millis(33); // 30 FPS

    for line in input.lines() {
        use indexmap::map::Entry;
        match map.entry(line?) {
            Entry::Vacant(e) => {
                e.insert(1);
            }
            Entry::Occupied(mut e) => {
                let slot = e.get_mut();
                let count = *slot + 1;
                *slot = count;
            }
        }

        let now = Instant::now();
        if now - last_rendered_time > inverse_fps {
            for (row, (key, count)) in map.iter().enumerate() {
                // Do nothing if screen is not big enough for this print()
                if row >= size()?.1 as usize {
                    break;
                }

                output.queue(MoveTo(0, row as u16))?.queue(Print(
                    if row >= last_rendered_size {
                        format!("{count:>7} {key}")
                    } else {
                        format!("{count:>7}")
                    },
                ))?;
            }

            last_rendered_time = now;
            last_rendered_size = map.len();
            output.flush()?;
        }
    }

    output.queue(LeaveAlternateScreen)?;
    for (line, count) in map {
        writeln!(output, "{count:>7} {line}")?;
    }

    Ok(())
}

fn count(input: &mut dyn BufRead, output: &mut dyn Write) -> Result<()> {
    // line -> count of occurrences
    let mut map = IndexMap::new();

    for line in input.lines() {
        use indexmap::map::Entry;
        match map.entry(line?) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1u64;
            }
            Entry::Vacant(e) => {
                e.insert(1u64);
            }
        }
    }

    for (line, count) in map {
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
