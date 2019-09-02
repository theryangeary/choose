# Choose

This is `choose`, a human-friendly alternative to `awk` and `cut`

## Rationale

The AWK programming language is designed for text processing and is extremely
capable in this endeavor. However, the `awk` command is not ideal for rapid
shell use, with its requisite quoting of a line wrapped in curly braces, even
for the simplest of programs:

```bash
awk '{print $1}'
```

Likewise, `cut` is far from ideal for rapid shell use, because it is difficult
to get the confusing syntax correct on the first attempt. Field separators and
ranges are just plain difficult to use.

It is for these reasons that I present to you `choose`. It is not meant to be a
drop-in or complete replacement for either of the aforementioned tools, but
rather a simple and intuitive tool to reach for when the basics of `awk` or
`cut` will do, but the overhead of getting them to behave should not be
necessary.

## Usage

`choose [OPTIONS] <FIELDS> FILE`

### Options

```
-f <NUM>        Field separator
-i              Inclusive ranges
```

### Examples

```bash
choose 5                # print the 5th item from a line (zero indexed)
choose -f ':' 0 3 5     # print the 0th, 3rd, and 5th item from a line, where
                        # items are separated by ':' instead of whitespace
choose 2:5              # print everything from the 2nd to 5th item on the line,
                        # exclusive of the 5th
choose -i 2:5           # print everything from the 2nd to 5th item on the line,
                        # inclusive of the 5th
choose :3               # print the beginning of the line to the 3rd item,
                        # exclusive
choose 3:               # print the third item to the end of the line
```
