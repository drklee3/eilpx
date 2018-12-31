# eilpx

A command line tool to pixel sort images based on red, green, blue, alpha, or luma values.

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
    -d, --direction <direction>    Sets direction of sorting [default: horizontal]  [possible values: horizontal,
                                   vertical]
    -i, --input <input>            Sets the input file
    -m, --mode <mode>              Sets mode of sorting [default: luma]  [possible values: red, green, blue, alpha,
                                   luma]
    -o, --output <output>          Sets the output file
    -t, --threshold <threshold>    Sets threshold of sorting
```

## Installation

Requires [Rust 2018](https://www.rust-lang.org/tools/install).

```bash
cargo install --git https://github.com/drklee3/eilpx
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
eilpx -i img/lighthouse.jpg -o img/lighthouse_sorted.jpg
```

Original                        | Sorted                               |
------------------------------- | ------------------------------------ |
![Original](img/lighthouse.jpg) | ![Sorted](img/lighthouse_sorted.jpg) |
