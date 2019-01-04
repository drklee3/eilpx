# eilpx

[![crates.io](https://img.shields.io/crates/v/eilpx.svg)](https://crates.io/crates/eilpx) [![Build Status](https://travis-ci.org/drklee3/eilpx.svg?branch=master)](https://travis-ci.org/drklee3/eilpx)

A command line pixel sorter.  Sorts pixels in images based on red, green, blue, alpha, or luma values in given directions based on configurable threshold values.

## Installation

Requires [Rust 2018](https://www.rust-lang.org/tools/install).  Eilpx can be installed via [crates.io](https://crates.io/crates/eilpx) with the following command.

```bash
cargo install eilpx
```

## Usage

```text
USAGE:
    eilpx [FLAGS] [OPTIONS] --input <input> --output <output>

FLAGS:
    -h, --help       Prints help information
    -y               Overwrite output files without asking
    -n               Do not overwrite output files, exit immediately if output file already exists
    -V, --version    Prints version information
    -v               Sets the level of verbosity

OPTIONS:
    -b, --bound <bound>            Sets threshold to be max or min [default: min]  [possible values: min, max]
    -d, --direction <direction>    Sets direction of sorting [default: right]  [possible values: up, right, down, left]
    -i, --input <input>            Sets the input file
    -m, --mode <mode>              Sets mode of sorting [default: luma]  [possible values: red, green, blue, alpha,
                                   luma]
    -o, --output <output>          Sets the output file
    -t, --threshold <threshold>    Sets threshold of sorting
```

## Examples

Command

```bash
eilpx -i img/tamsui.jpg -o img/tamsui_sorted.jpg -b max -t 175
#  -i img/tamsui.jpg           input file
#  -o img/tamsui_sorted.jpg    output file
#  -b max -t 175               sort only pixels with a max luma value of 175
```

Original                    | Sorted                           |
--------------------------- | -------------------------------- |
![Original](img/tamsui.jpg) | ![Sorted](img/tamsui_sorted.jpg) |

Command

```bash
# sorts leftwards based on luma with min bound of 50
eilpx -i img/lighthouse.jpg -o img/lighthouse_sorted.jpg -d left
```

Original                        | Sorted                               |
------------------------------- | ------------------------------------ |
![Original](img/lighthouse.jpg) | ![Sorted](img/lighthouse_sorted.jpg) |
