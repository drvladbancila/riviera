# riviera

*riviera* stands for: **RI**SC-**V** **I**mprobable **E**mulator, **R**ust **A**ssisted.

As the name suggests, it is a RISC-V emulator written in Rust. It is currently a work in progress in the early stages and supports RV32I and RV64I.

[![asciicast](https://asciinema.org/a/538760.svg)](https://asciinema.org/a/538760)

## Features

- it supports the RV32I and RV64I instructions set
- it can execute a 64 bit RISC-V compiled ELF at peak speed of about 60 MIPS
- supports interactive mode: step manually through the instructions and dump content of the register file
- interactive mode highlights the last register that was updated
- it can dump the content of the data memory to a binary file
- set the RAM size by command line arguments

## Building and running

To get the emulator running, first clone the repository and then build it with cargo:

```
git clone https://github.com/drvladbancila/riviera
cargo build --release
```

In order to get it running
```
cargo run --release  -- <arguments>
```

## Compiling and running executables

Programs need to be compiled without standard C library and (for now) with the `-march=rv64g` flag as this instructs the compiler to use only __non-compressed__ instructions (support may be added in the future).
Like this:
```
riscv64-unknown-linux-gnu-gcc -march=rv64g -nostdlib <files.c> -o <output_file>
```

To run an ELF file and obtain execution time, number of instruction and MIPS:
```
cargo run -- <ELF executable>
```

Other parameters are:

    - d <file>: dump DRAM content to binary file
    - r <n>: dump register contents on screen every <n> executed instructions
    - m <size>: set the DRAM size to <size>

For other usage parameters run with the `--help` flag.

## Testing

Some programs that can be run and used to test the emulator are put in the `tests` folder.
To automatically compile the tests:

```
cd tests
chmod +x compile.sh
./compile.sh
cd ..
```

Executables are stored in the `tests/compiled` folder.

To execute a test, run:

```
cargo run --release -- tests/compiled/<testname> <other params...>
```


## TODOs

- [ ] Support for compressed instructions
- [ ] Framebuffer for displaying user output
- [ ] Module to extract statistics from running code
