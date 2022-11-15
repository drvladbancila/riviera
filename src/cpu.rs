use crate::bus;
use crate::rv;
use crate::memory;
use crate::memory::AccessSize;
use colored::Colorize;

const REG_FILE_SIZE: usize = 32;
const CS_REG_FILE_SIZE: usize = 4096;
const PC_INITIAL_VALUE: u64 = 0x0;

pub const REG_FILE_NAMES: [&str; REG_FILE_SIZE] = [
 "zero", "ra", "sp",  "gp",  "tp", "t0", "t1", "t2",
 "s0",   "s1", "a0",  "a1",  "a2", "a3", "a4", "a5",
 "a6",   "a7", "s2",  "s3",  "s4", "s5", "s6", "s7",
 "s8",   "s9", "s10", "s11", "t3", "t4", "t5", "t6"
];

pub type Instruction = u32;
pub type RegIndex    = u8;
pub type CSRegIndex  = u16;

// CPU structure: it represents a RISC-V processing element
// Attributes:
// regs    -> array of 64 bits elements representing the reg. file
// pc      -> program counter
// next_pc -> value of the next PC that will be assigned to PC at
//            the end of the current cycle
// bus     -> bus object that allows to interface with memory 
//            and peripherals 
pub struct Cpu {
    regs: [u64; REG_FILE_SIZE],
    csregs: [u64; CS_REG_FILE_SIZE],
    pc: u64,
    next_pc: u64,
    bus: bus::Bus
}

// Cpu struct methods implementation
impl Cpu {
    pub const ZERO_REGISTER: RegIndex = 0x0;
    pub const RETURN_REGISTER: RegIndex = 0x1;
    pub const STACK_POINTER: RegIndex = 0x2;
    pub const GLOBAL_POINTER: RegIndex = 0x3;
    //pub const THREAD_POINTER: RegIndex = 0x4;

    pub const SENTINEL_RETURN_ADDRESS: u64 = 0xfffffffffffffffe;

    // Constructor
    pub fn new(memsize: Option<usize>) -> Cpu {
        Cpu {
            regs: [0; REG_FILE_SIZE],
            csregs: [0; CS_REG_FILE_SIZE],
            pc: PC_INITIAL_VALUE,
            next_pc: PC_INITIAL_VALUE,
            bus: bus::Bus::new(memsize),
        }
    }

    // Function that writes to a Cpu register
    #[inline(always)]
    pub fn write_reg(&mut self, regi: RegIndex, data: u64) {
        self.regs[regi as usize] = data;
    }

    // Function that reads data from a Cpu register
    #[inline(always)]
    pub fn read_reg(&self, regi: RegIndex) -> u64 {
        self.regs[regi as usize]
    }

    // Function that writes data to a Cpu CS register
    #[inline(always)]
    pub fn write_csreg(&mut self, csregi: CSRegIndex, data: u64) {
        match self.csregs.get_mut(csregi as usize) {
            Some(val) => *val = data,
            None => panic!("Invalid CSR address")
        }
    }

    // Function that reads data from a Cpu CS register
    #[inline(always)]
    pub fn read_csreg(&self, csregi: CSRegIndex) -> u64 {
        match self.csregs.get(csregi as usize) {
            Some(val) => *val,
            None => panic!("Invalid CSR address")
        }
    }

    // Function that displays the contents of all the registers
    pub fn dump_regs(&self) {
        let mut i: usize = 0;
        println!("{}", "Register values".red());
        println!("{}  : 0x{:0>16x}", "pc".purple(), self.pc);
        for r in self.regs {
            let rn: &str = REG_FILE_NAMES[i];
            print!("{:4}: 0x{:0>16x}\t", rn.green(), r);
            i += 1;
            if i % 2 == 0 {
                println!("");
            }
        }
        println!("");
    }

    // Get the current Program Counter
    #[inline(always)]
    pub fn get_pc(&self) -> u64{
        self.pc
    }

    // Set the Program Counter to a certain value
    #[inline(always)]
    pub fn set_pc(&mut self, value: u64) {
        self.pc = value;
    }

    // Get the next Program Counter
    #[inline(always)]
    pub fn get_next_pc(&self) -> u64 {
        self.next_pc
    }

    // Set the next PC = PC + signed constant
    #[inline(always)]
    pub fn set_next_pc_rel(&mut self, value: i64) {
        self.next_pc = (self.pc as i64 + value) as u64;
    }

    // Set the next PC = unsigned constant
    #[inline(always)]
    pub fn set_next_pc_abs(&mut self, value: u64) {
        self.next_pc = value;
    }

    #[inline(always)]
    pub fn set_stack_pointer(&mut self, value: u64) {
        self.regs[Cpu::STACK_POINTER as usize] = value;
    }

    // Cpu load from address (control is given to the Bus)
    // Since I/O is memory mapped it could be a load from DRAM, ROM or
    // any peripheral
    #[inline(always)]
    pub fn load(&self, addr: u64, size: AccessSize) -> u64 {
        self.bus.read(addr, size)
    }

    // Cpu store at address (control is given to the Bus)
    #[inline(always)]
    pub fn store(&mut self, data: u64, addr: u64, size: AccessSize) {
        self.bus.write(data, addr, size);
    }

    pub fn store_from_buffer(&mut self, data: &[u8], addr: u64) {
        self.bus.write_from_buf(addr, data)
    }

    pub fn set_read_only_segment(&mut self, offset: u64) {
        self.bus.set_rom_offset(offset);
    }

    #[allow(dead_code)]
    pub fn get_read_only_memsize(&self) -> usize {
        self.bus.get_rom_size()
    }

    pub fn get_read_write_memsize(&self) -> usize {
        self.bus.get_dram_size()
    }

    pub fn set_read_write_segment(&mut self, offset: u64) {
        self.bus.set_dram_offset(offset)
    }

    pub fn get_memory(&self) -> &memory::Memory {
        self.bus.get_device()
    }

    // Good ol' Fetch, Decode and Execute loop
    pub fn cpu_loop(&mut self) -> u64 {
        let mut count_instructions: u64 = 0;
        loop {
            if self.pc == Cpu::SENTINEL_RETURN_ADDRESS {
                break count_instructions;
            }
            // Fetch and instruction
            let fetched_instruction: Instruction = self.fetch();
            // Set the next PC assuming we continue the flow of execution
            self.next_pc = self.pc + 4;
            // Decode the instruction and call the function that implements
            // that instruction
            self.decode_and_execute(fetched_instruction);

            // The executed instruction might have changed the next PC
            // from the PC + 4 value, now assign next PC to PC
            self.pc = self.next_pc;
            count_instructions += 1;
        }
    }

    pub fn cpu_loop_with_reg_dump(&mut self, dump_period: u64) -> u64 {
        let mut count_instructions: u64 = 0;
        loop {
            if self.pc == Cpu::SENTINEL_RETURN_ADDRESS {
                break count_instructions;
            }
            // Fetch and instruction
            let fetched_instruction: Instruction = self.fetch();
            // Set the next PC assuming we continue the flow of execution
            self.next_pc = self.pc + 4;
            // Decode the instruction and call the function that implements
            // that instruction
            self.decode_and_execute(fetched_instruction);

            count_instructions += 1;

            if count_instructions % dump_period == 0 {
                self.dump_regs();
            }

            // The executed instruction might have changed the next PC
            // from the PC + 4 value, now assign next PC to PC
            self.pc = self.next_pc;
        }
    }

    // Fetch function to read the next instruction to be executed
    fn fetch(&self) -> Instruction {
        self.bus.read(self.pc, AccessSize::WORD) as Instruction
    }

    // Call the decoder to decode the instruction. The decoder will call
    // the function that handles the execution of the decoded instruction
    fn decode_and_execute(&mut self, instr: Instruction) {
        rv::decode(instr, self);
    }

}
