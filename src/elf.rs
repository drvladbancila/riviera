pub struct AddressSpace {
    pub read_execute_segment: usize,
    pub read_execute_size: usize,
    pub read_execute_offset: usize,
    pub read_write_segment: usize,
    pub read_write_size: usize,
    pub read_write_offset: usize
}

impl AddressSpace {
    const TEXT_START_DEFAULT: usize = 0x00000000;
    const DATA_START_DEFAULT: usize = 0x00020000;
    pub fn new() -> AddressSpace {
        AddressSpace {
            read_execute_segment: AddressSpace::TEXT_START_DEFAULT,
            read_execute_size: 0,
            read_execute_offset: 0,
            read_write_segment: AddressSpace::DATA_START_DEFAULT,
            read_write_size: 0,
            read_write_offset: 0
        }
    }
}

#[repr(C)]
struct ElfHeader {
    e_ident    : [u8; ElfHeader::EI_NIDENT],
    e_type     : u16,
    e_machine  : u16,
    e_version  : u32,
    e_entry    : u64,
    e_phoff    : u64,
    e_shoff    : u64,
    e_flags    : u32,
    e_ehsize   : u16,
    e_phentsize: u16,
    e_phnum    : u16,
    e_shentsize: u16,
    e_shnum    : u16,
    e_shstrndx : u16,
}

impl ElfHeader {
    // Size of e_ident field at the beginning of the ELF header
    const EI_NIDENT: usize = 16;
    // Elf Header fields offset from beginning of file

    // e_ident: this arrays specifies how to interpret the ELF file,
    // it contains magic numbers and infos like endianness, abi, architecture...
    const EIDENT_OFF:     usize = 0x00;
    // e_type: object file type (is it an executable? relocatable file?)
    const ETYPE_OFF:      usize = 0x10;
    // e_machine: required architecture to be executed (RISC-V)
    const EMACHINE_OFF:   usize = 0x12;
    // e_version: version of ELF standard
    const EVERSION_OFF:   usize = 0x14;
    // e_entry: entry point from where the CPU starts executing
    const EENTRY_OFF:     usize = 0x18;
    // e_phoff: offset to the program header table in the ELF file
    const EPHOFF_OFF:     usize = 0x20;
    // e_shoff: offset to the section header table in the ELF file
    const ESHOFF_OFF:     usize = 0x28;
    // e_flags: processor-specific flags
    const EFLAGS_OFF:     usize = 0x30;
    // e_ehsize: ELF header's size
    const EEHSIZE_OFF:    usize = 0x34;
    // e_phentsize: size in bytes of one entry in the program header table
    const EPHENTSIZE_OFF: usize = 0x36;
    // e_phnum: number of entries in the program header table
    const EPHNUM_OFF:     usize = 0x38;
    // e_shentsize: size in bytes of one entry in the section header table
    const ESHENTSIZE_OFF: usize = 0x3A;
    // e_shnum: number of entries in bytes in the section header table
    const ESHNUM_OFF:     usize = 0x3C;
    // e_shstrndx: section header table index of the table with section name table
    const ESHSTRNDX_OFF:  usize = 0x3E;

    /// Create new ELF Header
    fn new() -> ElfHeader {
        ElfHeader { e_ident: [0; ElfHeader::EI_NIDENT],
            e_type:  0, e_machine:   0, e_version:   0,
            e_entry: 0, e_phoff:     0, e_shoff:     0,
            e_flags: 0, e_ehsize:    0, e_phentsize: 0,
            e_phnum: 0, e_shentsize: 0, e_shnum:     0,
            e_shstrndx: 0
        }
    }

    /// Fill ELF header from byte buffer
    fn from_buffer(&mut self, buf: &[u8]) {
        self.e_ident.clone_from_slice(&buf[ElfHeader::EIDENT_OFF..ElfHeader::EIDENT_OFF + ElfHeader::EI_NIDENT]);
        self.e_type =      u16::from_le_bytes(buf[ElfHeader::ETYPE_OFF..ElfHeader::ETYPE_OFF + 2].try_into().unwrap());
        self.e_machine =   u16::from_le_bytes(buf[ElfHeader::EMACHINE_OFF..ElfHeader::EMACHINE_OFF + 2].try_into().unwrap());
        self.e_version =   u32::from_le_bytes(buf[ElfHeader::EVERSION_OFF..ElfHeader::EVERSION_OFF + 4].try_into().unwrap());
        self.e_entry =     u64::from_le_bytes(buf[ElfHeader::EENTRY_OFF..ElfHeader::EENTRY_OFF + 8].try_into().unwrap());
        self.e_phoff =     u64::from_le_bytes(buf[ElfHeader::EPHOFF_OFF..ElfHeader::EPHOFF_OFF + 8].try_into().unwrap());
        self.e_shoff =     u64::from_le_bytes(buf[ElfHeader::ESHOFF_OFF..ElfHeader::ESHOFF_OFF + 8].try_into().unwrap());
        self.e_flags =     u32::from_le_bytes(buf[ElfHeader::EFLAGS_OFF..ElfHeader::EFLAGS_OFF + 4].try_into().unwrap());
        self.e_ehsize =    u16::from_le_bytes(buf[ElfHeader::EEHSIZE_OFF..ElfHeader::EEHSIZE_OFF + 2].try_into().unwrap());
        self.e_phentsize = u16::from_le_bytes(buf[ElfHeader::EPHENTSIZE_OFF..ElfHeader::EPHENTSIZE_OFF + 2].try_into().unwrap());
        self.e_phnum =     u16::from_le_bytes(buf[ElfHeader::EPHNUM_OFF..ElfHeader::EPHNUM_OFF + 2].try_into().unwrap());
        self.e_shentsize = u16::from_le_bytes(buf[ElfHeader::ESHENTSIZE_OFF..ElfHeader::ESHENTSIZE_OFF + 2].try_into().unwrap());
        self.e_shnum =     u16::from_le_bytes(buf[ElfHeader::ESHNUM_OFF..ElfHeader::ESHNUM_OFF + 2].try_into().unwrap());
        self.e_shstrndx =  u16::from_le_bytes(buf[ElfHeader::ESHSTRNDX_OFF..ElfHeader::ESHSTRNDX_OFF + 2].try_into().unwrap());
    }
}

struct ProgHeader {
    p_type:   u32,
    p_flags:  u32,
    p_offset: u64,
    p_vaddr:  u64,
    p_paddr:  u64,
    p_filesz: u64,
    p_memsz:  u64,
    p_align:  u64
}

impl ProgHeader {
    const PTYPE_OFF:   usize = 0x00;
    const PFLAGS_OFF:  usize = 0x04;
    const POFFSET_OFF: usize = 0x08;
    const PVADDR_OFF:  usize = 0x10;
    const PPADDR_OFF:  usize = 0x18;
    const PFILESZ_OFF: usize = 0x20;
    const PMEMSZ_OFF:  usize = 0x28;
    const PALIGN_OFF:  usize = 0x30;

    const PTYPE_LOAD:   u32 = 0x1;
    const PFLAGS_READ:  u32 = 0x4;
    const PFLAGS_WRITE: u32 = 0x2;
    const PFLAGS_EXEC:  u32 = 0x1;

    /// Create new Program Header
    fn new() -> ProgHeader {
        ProgHeader {
            p_type:  0, p_flags: 0, p_offset: 0,
            p_vaddr: 0, p_paddr: 0, p_filesz: 0,
            p_memsz: 0, p_align: 0 }
    }

    /// Fill program header from byte buffer
    fn from_buffer(&mut self, buf: &[u8]) {
        self.p_type =   u32::from_le_bytes(buf[ProgHeader::PTYPE_OFF..ProgHeader::PTYPE_OFF + 4].try_into().unwrap());
        self.p_flags =  u32::from_le_bytes(buf[ProgHeader::PFLAGS_OFF..ProgHeader::PFLAGS_OFF + 4].try_into().unwrap());
        self.p_offset = u64::from_le_bytes(buf[ProgHeader::POFFSET_OFF..ProgHeader::POFFSET_OFF + 8].try_into().unwrap());
        self.p_vaddr =  u64::from_le_bytes(buf[ProgHeader::PVADDR_OFF..ProgHeader::PVADDR_OFF + 8].try_into().unwrap());
        self.p_paddr =  u64::from_le_bytes(buf[ProgHeader::PPADDR_OFF..ProgHeader::PPADDR_OFF + 8].try_into().unwrap());
        self.p_filesz = u64::from_le_bytes(buf[ProgHeader::PFILESZ_OFF..ProgHeader::PFILESZ_OFF + 8].try_into().unwrap());
        self.p_memsz =  u64::from_le_bytes(buf[ProgHeader::PMEMSZ_OFF..ProgHeader::PMEMSZ_OFF + 8].try_into().unwrap());
        self.p_align =  u64::from_le_bytes(buf[ProgHeader::PALIGN_OFF..ProgHeader::PALIGN_OFF + 8].try_into().unwrap());
    }
}

pub struct Elf {
    elf_header: ElfHeader,
    program_headers: Vec<ProgHeader>
}

impl Elf {
    /// Create new ELF
    pub fn new() -> Elf {
        Elf {
            elf_header: ElfHeader::new(),
            program_headers: Vec::new()
        }
    }

    pub fn read_header(&mut self, buf: &[u8]) -> u64 {
        self.elf_header.from_buffer(buf);
        self.elf_header.e_entry
    }

    pub fn read_progheaders(&mut self, buf: &[u8]) {
        for i in 0..self.elf_header.e_phnum as usize {
            let mut program_header_i = ProgHeader::new();
            let hdr_offset_byte: usize = self.elf_header.e_phoff as usize;
            let hdr_size_bytes: usize = self.elf_header.e_phentsize as usize;
            let hdr_start_byte: usize = hdr_offset_byte + hdr_size_bytes*i;

            program_header_i.from_buffer(&buf[hdr_start_byte..hdr_start_byte + hdr_size_bytes]);
            if program_header_i.p_type == ProgHeader::PTYPE_LOAD {
                self.program_headers.push(program_header_i);
            }
        }
    }

    pub fn get_addrspace(&self) -> AddressSpace {
        let mut addr_space: AddressSpace = AddressSpace::new();
        for hdr in &self.program_headers {
            let segment_start: usize = hdr.p_offset as usize;
            let segment_size: usize = hdr.p_filesz as usize;
            if hdr.p_flags == (ProgHeader::PFLAGS_READ | ProgHeader::PFLAGS_EXEC) {
                addr_space.read_execute_segment = hdr.p_paddr as usize;
                addr_space.read_execute_offset = segment_start;
                addr_space.read_execute_size = segment_size;
            }
            if hdr.p_flags == (ProgHeader::PFLAGS_READ | ProgHeader::PFLAGS_WRITE) {
                addr_space.read_write_segment = hdr.p_paddr as usize;
                addr_space.read_write_offset = segment_start;
                addr_space.read_write_size = segment_size;
            }
        }
        addr_space
    }

}