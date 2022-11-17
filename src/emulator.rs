use std::time::Duration;

use crate::cpu::Cpu;
use crate::elf::{Elf, AddressSpace};
use std::fs::File;
use std::io::Read;
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

    pub fn run(&mut self, reg_dump_interval: u64) -> (Duration, u64) {
        let now = std::time::Instant::now();
        let instruction_count: u64;
        if reg_dump_interval > 0 {
            instruction_count = self.cpu.cpu_loop_with_reg_dump(reg_dump_interval)
        } else {
            instruction_count = self.cpu.cpu_loop();
        }
        (now.elapsed(), instruction_count)
    }

    pub fn dump_memory_to_file(&self, filename: &str) {
        self.cpu.get_memory().dump_to_file(filename)
    }
}