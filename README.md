# ccuniq
## Overview

ccuniq is a Rust-based implementation of the Unix uniq command-line utility. It reads input from a specified file or standard input, compares adjacent lines, and writes a copy of each unique input line to the output. The tool can also count occurrences of lines, display repeated lines, and ignore case sensitivity.

## Features

- Reads from a specified input file or standard input.
- Writes unique lines to the output.
- Supports counting occurrences of lines.
- Can display repeated lines.
- Case insensitive comparison option.
- Configurable options for line parsing.

## Installation

To install ccuniq,ensure you have Rust and Cargo installed on your system. You can install Rust using rustup.

Once Rust is installed, clone the repository and build the project:

```
git clone https://github.com/yourusername/ccuniq.git
cd ccuniq
cargo build --release
```
The binary will be located in the target/release directory.


## Usage
After building the project, you can use the ccuniq command as follows:

```bash
./target/release/ccuniq [options] [input_file] [output_file]
```

## Options

-c: Precede each output line with the count of occurrences.

-d: Output a single copy of each line that is repeated in the input.

-D <type>: Control how to separate groups of repeated lines. Options include none, prepend, and separate.
-f: Ignore the first num fields in each input line when doing comparisons.
-i: Perform case insensitive comparison of lines.
-s: Ignore the first chars characters in each input line when doing comparisons.

-u: Only output lines that are not repeated in the input.

