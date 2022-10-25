use std::fs::File;
use std::io::Write;
use std::path::Path;
pub enum AccessSize {
    BYTE,
    HALFWORD,
    WORD,
    DOUBLEWORD
}

pub struct Memory {
    memory: Vec<u8>
}

impl Memory {
    pub const DRAM_DEFAULT_SIZE: usize = 4 * 1024;
    pub const ROM_DEFAULT_SIZE:  usize = 1 * 1024;

    pub fn new(size: Option<usize>) -> Memory {
            match size {
                Some(size) => Self { memory: vec![0; size]},
                None => Self { memory: Vec::new() },
            }
    }

    pub fn load(&self, paddr: u64, size: AccessSize) -> u64 {
        match size {
            AccessSize::BYTE => self.load8(paddr as usize) as u64,
            AccessSize::HALFWORD => self.load16(paddr as usize) as u64,
            AccessSize::WORD => self.load32(paddr as usize) as u64,
            AccessSize::DOUBLEWORD => self.load64(paddr as usize)
        }
    }

    pub fn get_size(&self) -> usize {
        self.memory.len()
    }

    pub fn store(&mut self, data: u64, paddr: u64, size: AccessSize) {
        match size {
            AccessSize::BYTE => self.store8(data as u8, paddr as usize),
            AccessSize::HALFWORD => self.store16(data as u16, paddr as usize),
            AccessSize::WORD => self.store32(data as u32, paddr as usize),
            AccessSize::DOUBLEWORD => self.store64(data as u64, paddr as usize)
        };
    }

    pub fn dump_to_file(&self, filename: &str) {
        let filepath: &Path = Path::new(filename);
        let display = filepath.display();

        let mut file = match File::create(&filepath) {
            Err(why) => panic!("Could not create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write(&self.memory) {
            Err(why) => panic!("Could not write memory buffer to {}: {}", display, why),
            Ok(_) => ()
        }
    }

    pub fn store_n_bytes(&mut self, data: &[u8], paddr: u64, size: usize) {
        if (paddr as usize + size)  <= self.memory.len() {
            self.memory[paddr as usize..paddr as usize+size].clone_from_slice(data);
        } else {
            self.memory.extend_from_slice(data).try_into().expect("Could not allocate enough memory")
        }
    }

    fn load8(&self, paddr: usize) -> u8 {
        self.memory[paddr]
    }

    fn load16(&self, paddr: usize) -> u16 {
        u16::from_le_bytes(self.memory[paddr..paddr + 2].try_into().unwrap())
    }

    fn load32(&self, paddr: usize) -> u32 {
        u32::from_le_bytes(self.memory[paddr..paddr + 4].try_into().unwrap())
    }

    fn load64(&self, paddr: usize) -> u64 {
        u64::from_le_bytes(self.memory[paddr..paddr + 8].try_into().unwrap())
    }

    fn store8(&mut self, data: u8, paddr: usize) {
        self.memory[paddr] = data;
    }

    fn store16(&mut self, data: u16, paddr: usize) {
        self.memory[paddr..paddr + 2].copy_from_slice(&data.to_le_bytes());
    }

    fn store32(&mut self, data: u32, paddr: usize) {
        self.memory[paddr..paddr + 4].copy_from_slice(&data.to_le_bytes());
    }

    fn store64(&mut self, data: u64, paddr: usize) {
        self.memory[paddr..paddr + 8].copy_from_slice(&data.to_le_bytes());
    }
}
