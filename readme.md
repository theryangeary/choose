# Choose

This is `choose`, a human-friendly and fast alternative to `cut` and (sometimes) `awk`

[![`choose` demo](https://asciinema.org/a/315932.png)](https://asciinema.org/a/315932?autoplay=1)

## Features

- terse field selection syntax similar to Python's list slices
- negative indexing from end of line
- optional start/end index
- zero-indexed
- reverse ranges
- slightly faster than `cut` for sufficiently long inputs, much faster than
  `awk`
- regular expression field separators using Rust's regex syntax

## Rationale

The AWK programming language is designed for text processing and is extremely
capable in this endeavor. However, the `awk` command is not ideal for rapid
shell use, with its requisite quoting of a line wrapped in curly braces, even
for the simplest of programs:

```bash
awk '{print $1}'
```

Likewise, `cut` is far from ideal for rapid shell use, because of its confusing
syntax. Field separators and ranges are just plain difficult to get right on the
first try.

It is for these reasons that I present to you `choose`. It is not meant to be a
drop-in or complete replacement for either of the aforementioned tools, but
rather a simple and intuitive tool to reach for when the basics of `awk` or
`cut` will do, but the overhead of getting them to behave should not be
necessary.

## Contributing

Please see our guidelines in [contributing.md](contributing.md).

## Usage

```
$ choose --help
choose 1.2.0
`choose` sections from each line of files

USAGE:
    choose [FLAGS] [OPTIONS] <choices>...

FLAGS:
    -c, --character-wise    Choose fields by character number
    -d, --debug             Activate debug mode
    -x, --exclusive         Use exclusive ranges, similar to array indexing in many programming languages
    -h, --help              Prints help information
    -n, --non-greedy        Use non-greedy field separators
    -V, --version           Prints version information

OPTIONS:
    -f, --field-separator <field-separator>
            Specify field separator other than whitespace, using Rust `regex` syntax

    -i, --input <input>                                      Input file
    -o, --output-field-separator <output-field-separator>    Specify output field separator

ARGS:
    <choices>...    Fields to print. Either a, a:b, a..b, or a..=b, where a and b are integers. The beginning or end
                    of a range can be omitted, resulting in including the beginning or end of the line,
                    respectively. a:b is inclusive of b (unless overridden by -x). a..b is exclusive of b and a..=b
                    is inclusive of b
```

### Examples

```bash
choose 5                # print the 5th item from a line (zero indexed)

choose -f ':' 0 3 5     # print the 0th, 3rd, and 5th item from a line, where
                        # items are separated by ':' instead of whitespace

choose 2:5              # print everything from the 2nd to 5th item on the line,
                        # inclusive of the 5th

choose -x 2:5           # print everything from the 2nd to 5th item on the line,
                        # exclusive of the 5th

choose :3               # print the beginning of the line to the 3rd item

choose -x :3            # print the beginning of the line to the 3rd item,
                        # exclusive

choose 3:               # print the third item to the end of the line

choose -1               # print the last item from a line

choose -3:-1            # print the last three items from a line
```

## Compilation and Installation

### Installing From Source

In order to build `choose` you will need the rust toolchain installed. You can
find instructions [here](https://www.rust-lang.org/tools/install).

Then, to install:

```bash
git clone https://github.com/theryangeary/choose.git
cd choose
cargo build --release
install target/release/choose <DESTDIR>
```

Just make sure DESTDIR is in your path.

### Installing From Package Managers

Cargo:

```sh
cargo install choose
```

Arch Linux:

```sh
yay -S choose-rust-git
```

Fedora/CentOS [COPR](https://copr.fedorainfracloud.org/coprs/atim/choose/):

```sh
dnf copr enable atim/choose
dnf install choose
```

Homebrew:

```sh
brew install choose-rust
```

MacPorts:

```sh
sudo port install choose
```

### Benchmarking

See [benchmarking](./benchmarking.md)

