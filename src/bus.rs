use crate::memory;

// Bus is an object that contains everything
// that is connected to the CPU through a bus
// such as: DRAM, ROM and other peripherals
pub struct Bus {
    dram: memory::Memory,
    dram_offset: u64,
    rom: memory::Memory,
    rom_offset: u64
}

impl Bus {

    const TEXT_START_DEFAULT: u64 = 0x00000000;
    const DATA_START_DEFAULT: u64 = 0x00020000;

    // Constructor, initialize DRAM to a certain size
    // while the ROM is only constructed, its size depends
    // on the ELF file that is loaded into it
    pub fn new(memsize: Option<usize>) -> Bus {
        Self {
            dram: memory::Memory::new(memsize),
            dram_offset: Bus::DATA_START_DEFAULT,
            rom:  memory::Memory::new(Some(memory::Memory::ROM_DEFAULT_SIZE)),
            rom_offset: Bus::TEXT_START_DEFAULT,
        }
    }

    // Read from any devide through the bus, this function (depending
    // on the memory boundaries) will dispatch the operation to the
    // appropriate device
    pub fn read(&self, addr: u64, size: memory::AccessSize) -> u64 {
        if addr < self.dram_offset  {
            self.rom.load(addr - self.rom_offset, size)
        } else {
            self.dram.load(addr - self.dram_offset, size)
        }
    }

    // Write to any devide through the bus, this function (depending
    // on the memory boundaries) will dispatch the operation to the
    // appropriate device
    pub fn write(&mut self, data: u64, addr: u64, size: memory::AccessSize) {
        if addr < self.dram_offset {
            self.rom.store(data, addr - self.rom_offset, size);
        } else {
            self.dram.store(data, addr - self.dram_offset, size);
        }
    }

    pub fn set_dram_offset(&mut self, offset: u64) {
        self.dram_offset = offset;
    }

    pub fn set_rom_offset(&mut self, offset: u64) {
        self.rom_offset = offset;
    }

    pub fn get_dram_size(&self) -> usize {
        self.dram.get_size()
    }

    pub fn get_rom_size(&self) -> usize {
        self.rom.get_size()
    }

    pub fn write_from_buf(&mut self, addr: u64, buf: &[u8]) {
        if addr < self.dram_offset {
            self.rom.store_n_bytes(buf, addr - self.rom_offset, buf.len());
        } else {
            self.dram.store_n_bytes(buf, addr - self.dram_offset, buf.len());
        }
    }

    pub fn get_device(&self) -> &memory::Memory {
        &self.dram
    }
}
