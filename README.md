uniqs
========
[`uniq`] alternative with streaming support.

```console
$ uniq -h
uniq(1) alternative with streaming support

Usage: uniqs [OPTIONS] [INPUT] [OUTPUT]

Arguments:
  [INPUT]   Path of the input file (default: stdin)
  [OUTPUT]  Path of the output file (default: stdout)

Options:
  -c, --count    Prefix lines by the number of occurrences
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

### Installation
Using Homebrew in macOS:
```bash
brew install simnalamburt/x/uniqs
```

Using Cargo:
```bash
cargo install uniqs
```

&nbsp;

--------
*uniqs* is primarily distributed under the terms of both the [Apache License
(Version 2.0)] and the [MIT license]. See [COPYRIGHT] for details.

[`uniq`]: https://www.gnu.org/software/coreutils/manual/html_node/uniq-invocation.html
[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
