# Choose

This is `choose`, a human-friendly and fast alternative to `awk` and `cut`

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

## Usage

```
$ choose --help
choose 0.1.4
`choose` sections from each line of files

USAGE:
    choose [FLAGS] [OPTIONS] <choice>...

FLAGS:
    -x, --exclusive    Use exclusive ranges, similar to array indexing in many programming languages
    -h, --help         Prints help information
    -V, --version      Prints version information

OPTIONS:
    -f, --field-separator <field-separator>    Specify field separator other than whitespace, using Rust `regex` syntax
    -i, --input <input>                        Input file

ARGS:
    <choice>...    Fields to print. Either x, x:, :y, or x:y, where x and y are integers, colons indicate a range,
                   and an empty field on either side of the colon continues to the beginning or end of the line.
```

### Examples

```bash
choose 5                # print the 5th item from a line (zero indexed)

choose -f ':' 0 3 5     # print the 0th, 3rd, and 5th item from a line, where
                        # items are separated by ':' instead of whitespace

choose 2:5              # print everything from the 2nd to 5th item on the line,
                        # exclusive of the 5th

choose -x 2:5           # print everything from the 2nd to 5th item on the line,
                        # exclusive of the 5th

choose :3               # print the beginning of the line to the 3rd item,
                        # exclusive

choose 3:               # print the third item to the end of the line

choose -1               # print the last item from a line

choose -3:-1            # print the last three items from a line
```

## Compilation and Installation

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

### Benchmarking

Benchmarking is performed using the [`bench` utility](https://github.com/Gabriel439/bench).

Benchmarking is based on the assumption that there are five files in `test/`
that match the glob "long*txt". GitHub doesn't support files big enough in
normal repos, but for reference the files I'm working with have lengths like
these:

```
     1000 test/long.txt
    19272 test/long_long.txt
    96360 test/long_long_long.txt
   963600 test/long_long_long_long.txt
 10599600 test/long_long_long_long_long.txt
```

and content generally like this:

```
Those an equal point no years do. Depend warmth fat but her but played. Shy and
subjects wondered trifling pleasant. Prudent cordial comfort do no on colonel as
assured chicken. Smart mrs day which begin. Snug do sold mr it if such.
Terminated uncommonly at at estimating. Man behaviour met moonlight extremity
acuteness direction.

Ignorant branched humanity led now marianne too strongly entrance. Rose to shew
bore no ye of paid rent form. Old design are dinner better nearer silent excuse.
She which are maids boy sense her shade. Considered reasonable we affronting on
expression in. So cordial anxious mr delight. Shot his has must wish from sell
```
