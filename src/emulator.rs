use std::time::Duration;
use colored::Colorize;
use crate::cpu::Cpu;
use crate::elf::{Elf, AddressSpace};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub struct Emulator {
    cpu: Cpu,
}

impl Emulator {
    pub fn new(memsize: Option<usize>) -> Emulator {
        Emulator {
            cpu: Cpu::new(memsize)
        }
    }

    pub fn load_program(&mut self, filename: &str) {
        let filepath: &Path = Path::new(filename);
        let display = filepath.display();
        let mut filebuffer: Vec<u8> = Vec::new();
        let mut elf_file = Elf::new();

        let mut file = match File::open(&filepath) {
            Err(why) => panic!("Could not open {}: {}", display, why),
            Ok(file) => file,
        };

        match file.read_to_end(&mut filebuffer) {
            Err(why) => panic!("Could not read {}: {}", display, why),
            Ok(_) => ()
        }

        let entry: u64 = elf_file.read_header(&filebuffer);

        elf_file.read_progheaders(&filebuffer);
        let addr_space: AddressSpace = elf_file.get_addrspace();

        self.cpu.set_read_only_segment(addr_space.read_execute_segment as u64);
        self.cpu.set_read_write_segment(addr_space.read_write_segment as u64);

        self.cpu.store_from_buffer(&filebuffer[addr_space.read_execute_offset..
                                                    addr_space.read_execute_offset
                                                    + addr_space.read_execute_size],
                                   addr_space.read_execute_segment as u64);

        self.cpu.store_from_buffer(&filebuffer[addr_space.read_write_offset..
                                                    addr_space.read_write_offset
                                                    + addr_space.read_write_size],
                              addr_space.read_write_segment as u64);

        self.cpu.set_pc(entry);

        // Load sentinel value in RA. If a program executes the "ret" instruction and there is no
        // nowhere else to return but this value then the emulator will stop executing instructions
        self.cpu.write_reg(Cpu::RETURN_REGISTER, Cpu::SENTINEL_RETURN_ADDRESS);

        // Set SP to the last address in the DRAM
        self.cpu.set_stack_pointer(addr_space.read_write_segment as u64 + self.cpu.get_read_write_memsize() as u64);

        // Set GP to the middle address in the DRAM
        // TODO: check if this is correct? Seems like it is, but not 100% sure
        self.cpu.write_reg(Cpu::GLOBAL_POINTER,
                     addr_space.read_write_segment as u64 + (self.cpu.get_read_write_memsize() as u64)/2);

    }

    pub fn run(&mut self) -> (Duration, u64) {
        let now = std::time::Instant::now();
        let instruction_count: u64;

        instruction_count = self.cpu.cpu_loop();
        (now.elapsed(), instruction_count)
    }

    pub fn interactive_run(&mut self) -> (Duration, u64) {

        let mut command_tokens;
        let mut instruction_count: u64 = 0;

        let now = std::time::Instant::now();
        self.cpu.set_debug_mode();
        loop {
            let mut command_string: String = String::new();
            print!("> ");
            let _ = std::io::stdout().flush();
            std::io::stdin().read_line(&mut command_string).expect("could not read from stdin");
            command_tokens = command_string.split(" ");
            let command_char: &str = command_tokens.next().expect("could not get token");
            match command_char.trim() {
                "s" =>
                {
                    let second_arg: Option<&str> = command_tokens.next();
                    match second_arg {
                        Some(num_steps) =>
                        {
                            match num_steps.trim().parse() {
                                Ok(num_steps) => instruction_count += self.cpu.cpu_loop_interactive(num_steps),
                                Err(err) => println!("Error: {}", err)
                            }

                        },
                        None => instruction_count += self.cpu.cpu_loop_interactive(1)
                    }
                },
                "r" => self.cpu.dump_regs(),
                "c" => { self.cpu.clear_debug_mode(); instruction_count += self.cpu.cpu_loop()},
                "d" =>
                {
                    let second_arg: Option<&str> = command_tokens.next();
                    match second_arg {
                        Some(filename) => self.dump_memory_to_file(filename.trim()),
                        None => println!("File name expected")
                    }
                }
                "q" => break,
                "h" => self.interactive_usage(),
                _   => println!("Command not recognized: type h for help"),
            }
        }
        (now.elapsed(), instruction_count)

    }

    fn interactive_usage(&self) {
        println!("Commands:");
        println!("{}: step by <n> instuctions (if omitted, execute next instruction)", "s [<n>]".bold());
        println!("{}: continue until executable is over", "c".bold());
        println!("{}: dump registers", "r".bold());
        println!("{}: quit interactive mode", "q".bold());
    }

    pub fn dump_memory_to_file(&self, filename: &str) {
        self.cpu.get_memory().dump_to_file(filename)
    }
}