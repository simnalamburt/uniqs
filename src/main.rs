use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{stdin, stdout, BufRead, Result, Write};

fn main() -> Result<()> {
    program(stdin().lock(), stdout().lock())
}

fn program<R: BufRead, W: Write>(input: R, mut output: W) -> std::io::Result<()> {
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

    fn test(input: &[u8], expected: &[u8]) {
        let mut actual = Vec::new();
        program(&input[..], &mut actual).unwrap();
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
