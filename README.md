# ASCII Plots 

Make snazzy plots in your terminal!

[![Build Status](https://travis-ci.org/mooreryan/ascii_plots.svg?branch=master)](https://travis-ci.org/mooreryan/ascii_plots)

## Install

### Binaries

Check the [releases](https://github.com/mooreryan/ascii_plots/releases) page to download the latest release for Mac, Linux, and Windows!

### From source

From GitHub, assuming you have [Rust installed](https://www.rust-lang.org/tools/install):

```
$ git clone https://github.com/mooreryan/ascii_plots.git
$ cd ascii_plots
$ cargo build --release
```

Then move or symlink the binary (`./target/release/asciiplot`) somewhere on your path:

```
$ ln -s $(pwd)/target/release/asciiplot $HOME/bin/asciiplot
```

## Usage

See the options.

```
$ asciiplot --help

asciiplot 0.1.0
Draw neat plots in your terminal!

USAGE:
    asciiplot <SUBCOMMAND>

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


SUBCOMMANDS:
    help           Prints this message or the help of the given subcommand(s)
    histogram      Draw histograms
    scatterplot    Draw scatterplots
```

### Histograms

Get help....

```
$ asciiplot histogram --help 
```

Make a histogram! 

(Data from the `test_files` directory in this repository.)

```
$ asciiplot histogram < test_files/rbeta_n_1000_s1_10_s2_1.txt 
min: 0.54, max: 1.00

| *
|
| **
| *
|
| ***
| **
| ***
| ***
| **
| ***
| ****
| ****
| *******
| ********
| *****
| ************
| ************
| ************
| *******************
| ***********************
| *************************
| **************************
| *****************************
| *******************************************************
| ******************************************
| *************************************************************
| *******************************************************
| ********************************************************************
| ********************************************************************************
```

### Scatterplots

Get help....

```
$ asciiplot scatterplot --help 
```

Make a histogram!

(Data from the `test_files` directory in this repository.)

```
$ asciiplot scatterplot < test_files/scatterplot_big.txt
x range -- min: -2.48, max: 2.47, range: 4.95
y range -- min: -4.36, max: 3.20, range: 7.56

|--------------------------------------------------------------------------------|
|                                               *                                |
|                                            *                         *         |
|                                                   *    *   *   *              *|
|                                     *               *       *                  |
|                              *                      *         *     *  *       |
|                            * *   *         *   *              *                |
|                          *                     ** **       *                   |
|                                ** *    * ***      * *         *                |
|                     *         **   * * *   * *       *   *                     |
|                 **  * *  *    *   **    * *  * *    *                          |
|              *      *  *    * *                 *                              |
|                      *  *  * *  *     * ****       *                           |
|                       **    *    *          *                                  |
|        *          *    *             *                                         |
|                        *                                                       |
|                                                                                |
| * *                                                                            |
|                                                                                |
|                                                                                |
|*                                                                               |
|--------------------------------------------------------------------------------| 
```

## License

Dual-licensed to be compatible with the Rust project.

Licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or the [MIT license](http://opensource.org/licenses/MIT), at your option. This program may not be copied, modified, or distributed except according to those terms.