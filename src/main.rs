use colored::Colorize;
use clap::Parser;
use crate::emulator::Emulator;

mod cpu;
mod bus;
mod memory;
mod rv;
mod elf;
mod emulator;

const BANNER: &str = "
        d8b          d8b
        Y8P          Y8P

888d888 888 888  888 888  .d88b.  888d888 8888b.
888P    888 888  888 888 d8P  Y8b 888P       88b
888     888 Y88  88P 888 88888888 888    .d888888
888     888  Y8bd8P  888 Y8b.     888    888  888
888     888   Y88P   888   Y8888  888     Y888888";

#[derive(Parser)]
#[command(author, version, about)]
struct CLIArguments {
    /// Executable to be run on emulator
    #[arg()]
    elf: String,

    /// File for memory dumping
    #[arg(short, long)]
    dump: Option<String>,

    /// Show registers every N instructions
    #[arg(short, long)]
    registers: Option<u64>,

    #[arg(short, long)]
    interactive: Option<bool>,

    /// RAM size for the emulator
    #[arg(short, long)]
    memsize: Option<u64>
}

/// Print welcome banner
fn welcome() {
    println!("{}\n", BANNER.bright_cyan());
    println!("{} is a RISC-V emulator", "riviera".bright_cyan());
    println!("Developed by Vlad George Bancila {}\n",
             "<https://github.com/drvladbancila>".green());
}
fn main() {
    welcome();

    // Parse arguments thanks to clap crate
    let args: CLIArguments = CLIArguments::parse();
    // Variable to store execution time for running the executable
    let execution_time: std::time::Duration;
    // Executed instructions counter
    let instr_count: u64;
    let mips: f64;
    let mut emu: Emulator;

    // If a memory size was specified with the -m flag, allocate a
    // DRAM vector with that size, otherwise the default value is taken
    if let Some(memsize) = args.memsize {
        emu = Emulator::new(Some(memsize as usize));
    } else {
        emu = Emulator::new(Some(memory::Memory::DRAM_DEFAULT_SIZE));
    }

    // Load ELF file into memory
    emu.load_program(args.elf.as_str());
    println!("{} ELF loaded correctly", "[*]".green());


    if let Some(interactive_mode) = args.interactive {
        if interactive_mode == true {
            (execution_time, instr_count) = emu.interactive_run()
        } else {
            (execution_time, instr_count) = emu.run();
        }
    } else {
        (execution_time, instr_count) = emu.run();
    }

    // If execution is over, print the total runtime
    mips = (instr_count as f64/1e6)/execution_time.as_secs_f64();
    println!("{} Execution is over ({:.2?})", "[*]".green(), execution_time);
    println!("{} T = {:.2?}, IC = {} ({:.6?} MIPS)",
             "[*]".green(), execution_time, instr_count, mips);

    // If the -d flag was used, dump all the DRAM in a binary file
    if let Some(dump_file) = args.dump.as_deref() {
        emu.dump_memory_to_file(dump_file);
        println!("{} Memory dump to file successful", "[*]".green());
    }
}
