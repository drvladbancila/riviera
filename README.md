# riviera

*riviera* stands for: **RI**SC-**V** **I**mprobable **E**mulator, **R**ust **A**ssisted.

As the name suggests, it is a RISC-V emulator written in Rust.

It is currently a work in progress in the early stages and supports RV32I only (although support for RV64I is planned).

## Building and running

To get the emulator running, first clone the repo and then build it with cargo:

```
git clone https://github.com/drvladbancila/riviera
cargo build
```

## Running executables

## Testing

```
riscv64-unknown-linux-gnu-gcc -march=rv64g -s -nostdlib <assembly_file.s> -o <output_file>
```

## TODOs

- [-] Add RV64I Instruction Set
- [ ] Add FENCE and CSR instructions in standard RV32I Instruction Set
- [ ] Deal with gp and sp initialization
- [ ] Compressed instructions
- [ ] Refactor ELF parser code and its interaction with the CPU struct
  