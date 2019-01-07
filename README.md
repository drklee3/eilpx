# eilpx

[![crates.io](https://img.shields.io/crates/v/eilpx.svg)](https://crates.io/crates/eilpx)
[![Travis Build Status](https://travis-ci.org/drklee3/eilpx.svg?branch=master)](https://travis-ci.org/drklee3/eilpx)
[![Azure Build Status](https://dev.azure.com/dlee3/dlee3/_apis/build/status/drklee3.eilpx?branchName=master)](https://dev.azure.com/dlee3/dlee3/_build/latest?definitionId=2?branchName=master)

A command line pixel sorter.  Sorts pixels in images based on red, green, blue, alpha, or luma values in given directions based on configurable threshold values.

## Installation

Precompiled binaries for Windows, macOS and Linux can be downloaded via [releases](https://github.com/drklee3/eilpx/releases).  You can also download artifacts on [Azure Pipelines](https://dev.azure.com/dlee3/dlee3/_build?definitionId=2) for builds on the master branch.

Eilpx can also be installed via [crates.io](https://crates.io/crates/eilpx).  Requires [Rust 2018](https://www.rust-lang.org/tools/install).

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
