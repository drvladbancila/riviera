use std::time::Duration;
use colored::Colorize;
use crate::cpu::Cpu;
use crate::elf::{Elf, AddressSpace};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

/// Emulator is just a wrapper for a CPU
/// It might contain a cluster of CPU in the future?
pub struct Emulator {
    cpu: Cpu,
}

impl Emulator {

    /// Create a new emulator with a certain memory size (DRAM)
    pub fn new(memsize: Option<usize>) -> Emulator {
        Emulator {
            cpu: Cpu::new(memsize)
        }
    }

    /// Load ELF, parse it and setup the CPU for execution from a given
    /// file path
    pub fn load_program(&mut self, filename: &str) -> Result<(), String> {
        let filepath: &Path = Path::new(filename);
        let display = filepath.display();
        let mut filebuffer: Vec<u8> = Vec::new();
        let mut elf_file = Elf::new();

        // Try to open the file
        let mut file = match File::open(&filepath) {
            Err(why) => panic!("Could not open {}: {}", display, why),
            Ok(file) => file,
        };

        // Try to read the file to the end and copy it into a heap-allocated buffer
        match file.read_to_end(&mut filebuffer) {
            Err(why) => panic!("Could not read {}: {}", display, why),
            Ok(_) => ()
        }

        // Read ELF header and obtain entry point
        let entry_point: u64;
        match elf_file.read_header(&filebuffer) {
            Ok(entry) => entry_point = entry,
            Err(err_string) => return Err(err_string),
        }

        // Read all the program headers to set the address space
        elf_file.read_progheaders(&filebuffer);
        // Get the address space
        let addr_space: AddressSpace = elf_file.get_addrspace();

        // Set the read-only memory offset (address at which the read only memory starts)
        self.cpu.set_read_only_segment(addr_space.read_execute_segment as u64);
        // Set the read-write memory offset
        self.cpu.set_read_write_segment(addr_space.read_write_segment as u64);
        // Copy the read-execute segment in the file into the read only memory of the CPU
        self.cpu.store_from_buffer(&filebuffer[addr_space.read_execute_offset..
                                                    addr_space.read_execute_offset
                                                    + addr_space.read_execute_size],
                                   addr_space.read_execute_segment as u64);

        // Copy the read-write segment from the file into the DRAM of the CPU
        self.cpu.store_from_buffer(&filebuffer[addr_space.read_write_offset..
                                                    addr_space.read_write_offset
                                                    + addr_space.read_write_size],
                              addr_space.read_write_segment as u64);

        // Set initial value of the PC
        self.cpu.set_pc(entry_point);

        // Load sentinel value in RA. If a program executes the "ret" instruction and there is no
        // nowhere else to return but this value then the emulator will stop executing instructions
        self.cpu.write_reg(Cpu::RETURN_REGISTER, Cpu::SENTINEL_RETURN_ADDRESS);

        // Set SP to the last address in the DRAM
        self.cpu.set_stack_pointer(addr_space.read_write_segment as u64 + self.cpu.get_read_write_memsize() as u64);

        // Set GP to the middle address in the DRAM
        // TODO: check if this is correct? Seems like it is, but not 100% sure
        self.cpu.write_reg(Cpu::GLOBAL_POINTER,
                     addr_space.read_write_segment as u64 + (self.cpu.get_read_write_memsize() as u64)/2);
        Ok(())

    }

    // Let the emulator run the CPU and execute all instructions
    // It returns the duration of the exectuion and the number of exectued instructions
    pub fn run(&mut self) -> (Duration, u64) {
        // Start the execution time counter
        let now = std::time::Instant::now();
        let instruction_count: u64;

        // Run CPU loop, this will return the number of executed instructions
        instruction_count = self.cpu.cpu_loop();
        (now.elapsed(), instruction_count)
    }

    // Let the emulator run in interactive mode: the user is asked
    // to move forward the program by stepping through the instructions
    // It returns the duration of the execution and the number of executed instructions
    pub fn interactive_run(&mut self) -> (Duration, u64) {
        let mut command_tokens: core::str::Split<&str>;
        let mut instruction_count: u64 = 0;
        // Start the execution time counter
        let now: std::time::Instant = std::time::Instant::now();
        // Set the debug mode of the CPU
        self.cpu.set_debug_mode();
        loop {
            let mut command_string: String = String::new();
            // Write command prompt
            print!("> ");
            let _ = std::io::stdout().flush();
            // Ask for user command
            std::io::stdin().read_line(&mut command_string).expect("could not read from stdin");
            // Split the command into tokens by using a whitespace as a delimiter
            command_tokens = command_string.split(" ");
            // Get the first item from the iterator returned by the split() method
            let command_char: &str = command_tokens.next().expect("could not get token");
            // Trim delimiting whitespaces and match the token with available commands
            match command_char.trim() {
                // s: step execution of N steps
                "s" =>
                {
                    // Try to get the number of steps as the following element from the iterator
                    let second_arg: Option<&str> = command_tokens.next();
                    match second_arg {
                        // If there is a second element...
                        Some(num_steps) =>
                        {
                            // Remove trailing whitespaces and try to parse the string into a u64
                            match num_steps.trim().parse() {
                                Ok(num_steps) => instruction_count += self.cpu.cpu_loop_interactive(num_steps),
                                Err(err) => println!("Error: {}", err)
                            }

                        },
                        // If there is not second element, just step by 1 instruction
                        None => instruction_count += self.cpu.cpu_loop_interactive(1)
                    }
                },
                // r: dump register content
                "r" => self.cpu.dump_regs(),
                // c: disable debug mode and run CPU loop until the end is reached
                "c" => { self.cpu.clear_debug_mode(); instruction_count += self.cpu.cpu_loop()},
                // d: dump the content of the DRAM into a binary file
                "d" =>
                {
                    let second_arg: Option<&str> = command_tokens.next();
                    match second_arg {
                        Some(filename) => {
                            match self.dump_memory_to_file(filename.trim()) {
                                Ok(res_string) => println!("{}", res_string),
                                Err(res_string) => println!("{}", res_string)
                            }
                        }
                        None => println!("Expected file name")
                    }
                }
                // q: quit interactive mode
                "q" => break,
                // h: show help
                "h" => self.interactive_usage(),
                // unrecognized command
                _   => println!("Command not recognized: type h for help"),
            }
        }
        (now.elapsed(), instruction_count)

    }

    /// This function shows the usage of the interactive mode
    fn interactive_usage(&self) {
        println!("Commands:");
        println!("{}: step by <n> instructions (if omitted, execute next instruction)", "s [<n>]".bold());
        println!("{}: continue until all code is executed", "c".bold());
        println!("{}: dump registers", "r".bold());
        println!("{}: dump memory content to binary file", "d <filename>".bold());
        println!("{}: quit interactive mode", "q".bold());
    }

    /// Dump the memory associated to the CPU to a file specified as a string
    pub fn dump_memory_to_file(&self, filename: &str) -> Result<String, String> {
        self.cpu.get_memory().dump_to_file(filename)
    }
}