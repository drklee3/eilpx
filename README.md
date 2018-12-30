# pixel-sorter

A command line tool to pixel sort images based on red, green, blue, alpha, or luma values.

## Usage

```text
USAGE:
    pixel-sorter [FLAGS] [OPTIONS] -i <input> -o <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               Sets the level of verbosity

OPTIONS:
    -d <direction>        Sets direction of sorting [default: horizontal]  [possible values: horizontal, vertical]
    -i <input>            Sets the input file
    -m <mode>             Sets mode of sorting [default: luma]  [possible values: red, green, blue, alpha, luma]
    -o <output>           Sets the output file
    -t <threshold>        Sets threshold of sorting
```
