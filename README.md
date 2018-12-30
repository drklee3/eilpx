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

## Building

Requires [Rust 2018](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/drklee3/pixel-sorter.git
cd pixel-sorter

cargo build --release
```

## Examples

```bash
pixel-sorter -i img/tamsui.jpg -o img/tamsui_sorted.jpg -b max -t 175
#  -i img/tamsui.jpg           input file
#  -o img/tamsui_sorted.jpg    output file
#  -b max -t 175               sort only pixels with a max luma value of 175
```

Original Image | Sorted Image |
-------------- | ------------ |
![Original Image](img/tamsui.jpg) | ![Sorted Image](img/tamsui_sorted.jpg) |


```bash
pixel-sorter -i img/lighthouse.jpg -o img/lighthouse_sorted.jpg
```

Original Image | Sorted Image |
-------------- | ------------ |
![Original Image](img/lighthouse.jpg) | ![Sorted Image](img/lighthouse_sorted.jpg) |
