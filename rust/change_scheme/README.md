# Change Scheme Cli APP

A standalone version cil app of the `change_scheme.py` python script. Functionality and usage are as same as the original script.

## Usage

Here is the help info of the app, for more detailed description please refer to the [README](../../README.md) in root folder.

```
A cli app to change dictionary scheme of TextGrid files.

Usage: change_scheme [OPTIONS] [TextGrids]

Arguments:
  [TextGrids]  The directory of textgrid files, non recursive.

Options:
  -s, --scheme <PATH>  The path of scheme file. [default: ./configs/cantonese-two-seg.csv]
  -o, --out <DIR>      The path of output directory. [default: ./out]
  -d, --debug          Turn on debug mode
  -h, --help           Print help
  -V, --version        Print version
```

## Build

To build this app you need to install rust and cargo. Please refer to the official [Install Rust](https://www.rust-lang.org/tools/install) instruction.

Once you install rust, run following command to double check if everything is working:

```bash
cargo --version
```

It should show something like this:
```
cargo 1.81.0 (2dbb1af80 2024-08-20)
```

Then you could run this to build:
```bash
cargo build --release
```

Notice that your current working directory should be in the root of this repository, not in the `/rust/change_scheme/` folder.

The compiled app should locate in `/rust/change_scheme/target/release/change_scheme`

## Credit

[min-sized-rust](https://github.com/johnthagen/min-sized-rust)

