use crate::cpu::Instruction;
use crate::cpu::RegIndex;
use crate::cpu::Cpu;
use crate::memory::AccessSize;

#[derive(PartialEq, Eq)]
pub struct DecInstruction {
    opcode: u8,
    f3: u8,
    f7: u8
}

struct OpCodes;
impl OpCodes {
    // RV32I
    const RTYPE:   u8 = 0b0110011;
    const ITYPE:   u8 = 0b0010011;
    const STYPE:   u8 = 0b0100011;
    const BTYPE:   u8 = 0b1100011;
    const LOAD:    u8 = 0b0000011;
    const LUI:     u8 = 0b0110111;
    const AUIPC:   u8 = 0b0010111;
    const JAL:     u8 = 0b1101111;
    const JALR:    u8 = 0b1100111;
    const FENCE:   u8 = 0b0001111;
    const EXCEP:   u8 = 0b1110011;
    // RV64I
    const RTYPE64: u8 = 0b0111011;
    const ITYPE64: u8 = 0b0011011;
}

pub fn decode(instr: Instruction, curcpu: &mut Cpu) {
    // opcode = instr[6:0]
    let opcode = (instr & 0x7f) as u8;
    // f3 = instr[14:12]
    let f3 = ((instr >> 12) & 0x7) as u8;
    // f7 = instr[31:25]
    let f7 = ((instr >> 25) & 0x7f) as u8;

    // rd = instr[11:7]
    let rd:  RegIndex = ((instr >>  7) & 0x1f) as RegIndex;
    // rs1 = instr[19:15]
    let rs1: RegIndex = ((instr >> 15) & 0x1f) as RegIndex;
    // rs2 = instr[24:20]
    let rs2: RegIndex = ((instr >> 20) & 0x1f) as RegIndex;
    // 5 bits long immediate takes the place of rd instr[11:7]
    let imm5:  u32 = ((instr >>  7) & 0x1f) as u32;
    // 12 bits long immediate is instr[31:20]
    let imm12: u32 = (instr as i32 >> 20) as u32;
    // 20 bits long immediate is instr[31:12]
    // cast to signed integer to do sign extension as we shift right
    let imm20: u32 = (instr as i32 >> 12) as u32;

    let dec_instr: DecInstruction = DecInstruction { opcode, f3, f7 };

    match dec_instr {
        // RV32I Base Instruction Set
        // LUI
        DecInstruction { opcode: OpCodes::LUI,   f3: _,     f7: _         } => lui(curcpu, rd, imm20),
        // AUIPC
        DecInstruction { opcode: OpCodes::AUIPC, f3: _,     f7: _         } => auipc(curcpu, rd, imm20),
        // JAL
        DecInstruction { opcode: OpCodes::JAL,   f3: _,     f7: _         } => jal(curcpu, rd, imm20),
        // JALR
        DecInstruction { opcode: OpCodes::JALR,  f3: 0b000, f7: _         } => jalr(curcpu, rs1, rd, imm12),
        // BEQ
        DecInstruction { opcode: OpCodes::BTYPE, f3: 0b000, f7: _         } => beq(curcpu, rs1, rs2, imm5, imm12),
        // BNE
        DecInstruction { opcode: OpCodes::BTYPE, f3: 0b001, f7: _         } => bne(curcpu, rs1, rs2, imm5, imm12),
        // BLT
        DecInstruction { opcode: OpCodes::BTYPE, f3: 0b100, f7: _         } => blt(curcpu, rs1, rs2, imm5, imm12),
        // BGE
        DecInstruction { opcode: OpCodes::BTYPE, f3: 0b101, f7: _         } => bge(curcpu, rs1, rs2, imm5, imm12),
        // BLTU
        DecInstruction { opcode: OpCodes::BTYPE, f3: 0b110, f7: _         } => bltu(curcpu, rs1, rs2, imm5, imm12),
        // BGEU
        DecInstruction { opcode: OpCodes::BTYPE, f3: 0b111, f7: _         } => bgeu(curcpu, rs1, rs2, imm5, imm12),
        // LB
        DecInstruction { opcode: OpCodes::LOAD,  f3: 0b000, f7: _         } => lb(curcpu, rs1, rd, imm12),
        // LH
        DecInstruction { opcode: OpCodes::LOAD,  f3: 0b001, f7: _         } => lh(curcpu, rs1, rd, imm12),
        // LW
        DecInstruction { opcode: OpCodes::LOAD,  f3: 0b010, f7: _         } => lw(curcpu, rs1, rd, imm12),
        // LBU
        DecInstruction { opcode: OpCodes::LOAD,  f3: 0b100, f7: _         } => lbu(curcpu, rs1, rd, imm12),
        // LHU
        DecInstruction { opcode: OpCodes::LOAD,  f3: 0b101, f7: _         } => lhu(curcpu, rs1, rd, imm12),
        // SB
        DecInstruction { opcode: OpCodes::STYPE, f3: 0b000, f7: _         } => sb(curcpu, rs1, imm12, imm5),
        // SH
        DecInstruction { opcode: OpCodes::STYPE, f3: 0b001, f7: _         } => sh(curcpu, rs1, imm12, imm5),
        // SW
        DecInstruction { opcode: OpCodes::STYPE, f3: 0b010, f7: _         } => sw(curcpu, rs1, imm12, imm5),
        // ADDI
        DecInstruction { opcode: OpCodes::ITYPE, f3: 0b000, f7: _         } => addi(curcpu, rs1, rd, imm12),
        // SLTI
        DecInstruction { opcode: OpCodes::ITYPE, f3: 0b010, f7: _         } => slti(curcpu, rs1, rd, imm12),
        // SLTIU
        DecInstruction { opcode: OpCodes::ITYPE, f3: 0b011, f7: _         } => sltiu(curcpu, rs1, rd, imm12),
        // XORI
        DecInstruction { opcode: OpCodes::ITYPE, f3: 0b100, f7: _         } => xori(curcpu, rs1, rd, imm12),
        // ORI
        DecInstruction { opcode: OpCodes::ITYPE, f3: 0b110, f7: _         } => ori(curcpu, rs1, rd, imm12),
        // ANDI
        DecInstruction { opcode: OpCodes::ITYPE, f3: 0b111, f7: _         } => andi(curcpu, rs1, rd, imm12),
        // SLLI
        DecInstruction { opcode: OpCodes::ITYPE, f3: 0b001, f7: _         } => slli(curcpu, rs1, rd, imm12),
        // SRLI and SRAI
        DecInstruction { opcode: OpCodes::ITYPE, f3: 0b101, f7: _         } => srli_srai(curcpu, rs1, rd, imm12),
        // ADD
        DecInstruction { opcode: OpCodes::RTYPE, f3: 0b000, f7: 0b0000000 } => add(curcpu, rs1, rs2, rd),
        // SUB
        DecInstruction { opcode: OpCodes::RTYPE, f3: 0b000, f7: 0b0100000 } => sub(curcpu, rs1, rs2, rd),
        // SLL
        DecInstruction { opcode: OpCodes::RTYPE, f3: 0b001, f7: 0b0000000 } => sll(curcpu, rs1, rs2, rd),
        // SLT
        DecInstruction { opcode: OpCodes::RTYPE, f3: 0b010, f7: 0b0000000 } => slt(curcpu, rs1, rs2, rd),
        // SLTU
        DecInstruction { opcode: OpCodes::RTYPE, f3: 0b011, f7: 0b0000000 } => sltu(curcpu, rs1, rs2, rd),
        // XOR
        DecInstruction { opcode: OpCodes::RTYPE, f3: 0b100, f7: 0b0000000 } => xor(curcpu, rs1, rs2, rd),
        // SRL
        DecInstruction { opcode: OpCodes::RTYPE, f3: 0b101, f7: 0b0000000 } => srl(curcpu, rs1, rs2, rd),
        // SRA
        DecInstruction { opcode: OpCodes::RTYPE, f3: 0b101, f7: 0b0100000 } => sra(curcpu, rs1, rs2, rd),
        // OR
        DecInstruction { opcode: OpCodes::RTYPE, f3: 0b110, f7: 0b0000000 } => or(curcpu, rs1, rs2, rd),
        // AND
        DecInstruction { opcode: OpCodes::RTYPE, f3: 0b111, f7: 0b0000000 } => and(curcpu, rs1, rs2, rd),
        // FENCE
        DecInstruction { opcode: OpCodes::FENCE, f3: 0b000, f7: _         } => fence(),
        // FENCEI
        DecInstruction { opcode: OpCodes::FENCE, f3: 0b001, f7: _         } => fencei(),
        // ECALL
        DecInstruction { opcode: OpCodes::EXCEP, f3: 0b000, f7: 0b0000000 } => ecall_ebreak(imm12),
        // CSRRW
        DecInstruction { opcode: OpCodes::EXCEP, f3: 0b001, f7: _         } => csrrw(curcpu, rs1, rd, imm12),
        // CSRRS
        DecInstruction { opcode: OpCodes::EXCEP, f3: 0b010, f7: _         } => csrrs(curcpu, rs1, rd, imm12),
        // CSRRC
        DecInstruction { opcode: OpCodes::EXCEP, f3: 0b011, f7: _         } => csrrc(curcpu, rs1, rd, imm12),
        // CSRRWI
        DecInstruction { opcode: OpCodes::EXCEP, f3: 0b101, f7: _         } => csrrwi(curcpu, rs1, rd, imm12),
        // CSRRS
        DecInstruction { opcode: OpCodes::EXCEP, f3: 0b110, f7: _         } => csrrsi(curcpu, rs1, rd, imm12),
        // CSRRCI
        DecInstruction { opcode: OpCodes::EXCEP, f3: 0b111, f7: _         } => csrrci(curcpu, rs1, rd, imm12),

        // RV64I Base Instruction Set
        // LWU
        DecInstruction { opcode: OpCodes::LOAD,    f3: 0b110, f7: _         } => lwu(curcpu, rs1, rd, imm12),
        // LD
        DecInstruction { opcode: OpCodes::LOAD,    f3: 0b011, f7: _         } => ld(curcpu, rs1, rd, imm12),
        // SD
        DecInstruction { opcode: OpCodes::STYPE,   f3: 0b011, f7: _         } => sd(curcpu, rs1, imm12, imm5),
        // ADDIW
        DecInstruction { opcode: OpCodes::ITYPE64, f3: 0b000, f7: _         } => addiw(curcpu, rs1, rd, imm12),
        // SLLIW
        DecInstruction { opcode: OpCodes::ITYPE64, f3: 0b001, f7: 0b0000000 } => slliw(curcpu, rs1, rd, imm12),
        // SRLIW and SRAIW
        DecInstruction { opcode: OpCodes::ITYPE64, f3: 0b101, f7: _         } => srliw_sraiw(curcpu, rs1, rd, imm12),
        // ADDW
        DecInstruction { opcode: OpCodes::RTYPE64, f3: 0b000, f7: 0b0000000 } => addw(curcpu, rs1, rs2, rd),
        // SUBW
        DecInstruction { opcode: OpCodes::RTYPE64, f3: 0b000, f7: 0b0100000 } => subw(curcpu, rs1, rs2, rd),
        // SLLW
        DecInstruction { opcode: OpCodes::RTYPE64, f3: 0b001, f7: 0b0000000 } => sllw(curcpu, rs1, rs2, rd),
        // SRLW
        DecInstruction { opcode: OpCodes::RTYPE64, f3: 0b101, f7: 0b0000000 } => srlw(curcpu, rs1, rs2, rd),
        // SRAW
        DecInstruction { opcode: OpCodes::RTYPE64, f3: 0b101, f7: 0b0100000 } => sraw(curcpu, rs1, rs2, rd),
        _ => panic!("Not recognized")
    };
}

// Decode J-Type Immediates
#[inline(always)]
fn decode_immediate_jtype(imm20: u32) -> i64 {
    let imm_32_20: u32 = imm20 & 0xfff80000;
    let imm_19_12: u32 = (imm20 & 0xff) << 11;
    let imm_11:    u32 = (imm20 & 0x100) << 2;
    let imm_10_0:  u32 = (imm20 & 0x7fe00) >> 8;

    ((imm_32_20 | imm_19_12 | imm_11 | imm_10_0)) as i32 as i64
}

// Decode B-Type Immediates
#[inline(always)]
fn decode_immediate_btype(imm5: u32, imm12: u32) -> i64 {
    let imm_32_12: u32 = imm12 & 0xfffff800;
    let imm_11:    u32 = (imm5 & 0x1) << 10;
    let imm_10_5:  u32 = imm12 & 0x7e0;
    let imm_4_0:   u32 = imm5 & !0x3;

    (imm_32_12 | imm_11 | imm_10_5 | imm_4_0) as i32 as i64
}

// Decode S-Type Immediates
#[inline(always)]
fn decode_immediate_stype(imm5: u32, imm12: u32) -> i64 {
    ((imm12 & 0xffffffe0) | imm5) as i32 as i64
}

// LUI instruction
// rd <- signed'imm[32:12] << 12
#[inline(always)]
fn lui(curcpu: &mut Cpu, rd: RegIndex, imm: u32) {
    curcpu.write_reg(rd, (imm << 12) as u64);
}

// AUIPC instruction
// rd <- pc + (signed'imm[32:12] << 12)
#[inline(always)]
fn auipc(curcpu: &mut Cpu, rd: RegIndex, imm: u32) {
    // AUIPC adds an immediate to the current PC (the one that points to 
    // this instruction)
    let first_operand: i64 = (curcpu.get_pc()) as i64;
    // immediate is sign-extended to 64 bits and shifted left
    let second_operand: i64 = (imm as i32 as i64) << 12; 
    curcpu.write_reg(rd, (first_operand + second_operand) as u64);
}

// JAL instruction
// rd <- pc + 4
// pc <- pc + signed'immediate
#[inline(always)]
fn jal(curcpu: &mut Cpu, rd: RegIndex, imm: u32) {
    // Next PC needs to be saved in rd
    if rd != Cpu::ZERO_REGISTER {
        curcpu.write_reg(rd, curcpu.get_next_pc());
    }
    // The immediate - instead - needs to be added to this PC
    let imm64: i64 = decode_immediate_jtype(imm);
    curcpu.set_next_pc_rel(imm64);
}

// JALR instruction
// rd <- pc + 4
// pc <- (pc + signed'immediate) & !0x1
#[inline(always)]
fn jalr(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm: u32) {
    if rd != Cpu::ZERO_REGISTER {
        curcpu.write_reg(rd, curcpu.get_next_pc());
    }
    let first_operand: i64 = curcpu.read_reg(rs1) as i64;
    let second_operand: i64 = imm as i32 as i64;
    // Mask the resulting PC with 0xfff...ffe so that it is always an even number
    curcpu.set_next_pc_abs(((first_operand + second_operand) & !0x1) as u64);
}

// BEQ instruction
// if (rs1 == rs2) { pc = pc + signed'immediate }
#[inline(always)]
fn beq(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, imm5: u32, imm12: u32) {
    let imm64: i64 = decode_immediate_btype(imm5, imm12);

    if curcpu.read_reg(rs1) == curcpu.read_reg(rs2) {
        curcpu.set_next_pc_rel(imm64);
    }
}

// BNE instruction
// if (rs1 != rs2) { pc = pc + signed'immediate }
#[inline(always)]
fn bne(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, imm5: u32, imm12: u32) {
    let imm64: i64 = decode_immediate_btype(imm5, imm12);

    if curcpu.read_reg(rs1) != curcpu.read_reg(rs2) {
        curcpu.set_next_pc_rel(imm64);
    }
}

// BLT instruction
// if (singed'rs1 < signed'rs2) { pc = pc + signed'immediate }
#[inline(always)]
fn blt(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, imm5: u32, imm12: u32) {
    let imm64: i64 = decode_immediate_btype(imm5, imm12);

    if (curcpu.read_reg(rs1) as i64) < curcpu.read_reg(rs2) as i64 {
        curcpu.set_next_pc_rel(imm64);
    }
}

// BGE instruction
// if (signed'rs1 >= signed'rs2) { pc = pc + signed'immediate }
#[inline(always)]
fn bge(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, imm5: u32, imm12: u32) {
    let imm64: i64 = decode_immediate_btype(imm5, imm12);

    if curcpu.read_reg(rs1) as i64 >= curcpu.read_reg(rs2) as i64 {
        curcpu.set_next_pc_rel(imm64);
    }
}

// BLTU instruction
// if (unsigned'rs1 < unsigned'rs2) { pc = pc + signed'immediate }
#[inline(always)]
fn bltu(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, imm5: u32, imm12: u32) {
    let imm64: i64 = decode_immediate_btype(imm5, imm12);

    if curcpu.read_reg(rs1) < curcpu.read_reg(rs2) {
        curcpu.set_next_pc_rel(imm64);
    }
}

// BGEU instruction
// if (unsigned'rs1 >= unsigned'rs2) { pc = pc + signed'immediate }
#[inline(always)]
fn bgeu(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, imm5: u32, imm12: u32) {
    let imm64: i64 = decode_immediate_btype(imm5, imm12);

    if curcpu.read_reg(rs1) >= curcpu.read_reg(rs2) {
        curcpu.set_next_pc_rel(imm64);
    }
}

// LB instruction
// rd <- memory[signed'rs1 + signed'imm][7:0]
#[inline(always)]
fn lb(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let addr: u64 = (curcpu.read_reg(rs1) as i64 + imm12 as i32 as i64) as u64;
    let data: i64 = curcpu.load(addr, AccessSize::BYTE) as i8 as i64;
    curcpu.write_reg(rd, data as u64);
}

// LH instruction
// rd <- memory[signed'rs1 + signed'imm][15:0]
#[inline(always)]
fn lh(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let addr: u64 = (curcpu.read_reg(rs1) as i64 + imm12 as i32 as i64) as u64;
    let data: i64 = curcpu.load(addr, AccessSize::HALFWORD) as i16 as i64;
    curcpu.write_reg(rd, data as u64);
}

// LW instruction
// rd <- memory[signed'rs1 + signed'imm][31:0]
#[inline(always)]
fn lw(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let addr: u64 = (curcpu.read_reg(rs1) as i64 + imm12 as i32 as i64) as u64;
    let data: i64 = curcpu.load(addr, AccessSize::WORD) as i32 as i64;
    curcpu.write_reg(rd, data as u64);
}

// LD instruction
// rd <- memory[signed'rs1 + signed'imm][63:0]
#[inline(always)]
fn ld(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let addr: u64 = (curcpu.read_reg(rs1) as i64 + imm12 as i32 as i64) as u64;
    let data: u64 = curcpu.load(addr, AccessSize::DOUBLEWORD);
    curcpu.write_reg(rd, data);
}

// LBU instruction
// rd <- memory[rs1 + unsigned'(signed'imm)][7:0]
#[inline(always)]
fn lbu(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let addr: u64 = (curcpu.read_reg(rs1) as i64 + imm12 as i32 as i64) as u64;
    let data: u64 = curcpu.load(addr, AccessSize::BYTE);
    curcpu.write_reg(rd, data);
}

// LHU instruction
// rd <- memory[rs1 + unsigned'(signed'imm)][15:0]
#[inline(always)]
fn lhu(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let addr: u64 = (curcpu.read_reg(rs1) as i64 + imm12 as i32 as i64) as u64;
    let data: u64 = curcpu.load(addr, AccessSize::HALFWORD);
    curcpu.write_reg(rd, data);
}

// LWU instruction
// rd <- memory[signed'rs1 + signed'imm][63:0]
#[inline(always)]
fn lwu(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let addr: u64 = (curcpu.read_reg(rs1) as i64 + imm12 as i32 as i64) as u64;
    let data: u64 = curcpu.load(addr, AccessSize::WORD);
    curcpu.write_reg(rd, data);
}

// SB instruction
// memory[signed'rs1 + imm] = rs2[7:0]
#[inline(always)]
fn sb(curcpu: &mut Cpu, rs1: RegIndex, imm12: u32, imm5: u32) {
    let rs2: RegIndex = (imm12 & 0x1f) as RegIndex;
    let data: u64 = curcpu.read_reg(rs2);
    let imm: i64 = decode_immediate_stype(imm5, imm12); 
    let addr: u64 = (curcpu.read_reg(rs1) as i64 + imm) as u64;
    curcpu.store(data, addr, AccessSize::BYTE);
}

// SH instruction
// memory[signed'rs1 + imm] = rs2[15:0]
#[inline(always)]
fn sh(curcpu: &mut Cpu, rs1: RegIndex, imm12: u32, imm5: u32) {
    let rs2: RegIndex = (imm12 & 0x1f) as RegIndex;
    let data: u64 = curcpu.read_reg(rs2);
    let imm: i64 = decode_immediate_stype(imm5, imm12); 
    let addr: u64 = (curcpu.read_reg(rs1) as i64 + imm) as u64;
    curcpu.store(data, addr, AccessSize::HALFWORD);
}

// SW instruction
// memory[signed'rs1 + imm] = rs2[31:0]
#[inline(always)]
fn sw(curcpu: &mut Cpu, rs1: RegIndex, imm12: u32, imm5: u32) {
    let rs2: RegIndex = (imm12 & 0x1f) as RegIndex;
    let data: u64 = curcpu.read_reg(rs2);
    let imm: i64 = decode_immediate_stype(imm5, imm12); 
    let addr: u64 = (curcpu.read_reg(rs1) as i64 + imm) as u64;
    curcpu.store(data, addr, AccessSize::WORD);
}

// SD instruction
// memory[signed'rs1 + imm] = rs2[63:0]
#[inline(always)]
fn sd(curcpu: &mut Cpu, rs1: RegIndex, imm12: u32, imm5: u32) {
    let rs2: RegIndex = (imm12 & 0x1f) as RegIndex;
    let data: u64 = curcpu.read_reg(rs2);
    let imm: i64 = decode_immediate_stype(imm5, imm12); 
    let addr: u64 = (curcpu.read_reg(rs1) as i64 + imm) as u64;
    curcpu.store(data, addr, AccessSize::DOUBLEWORD);
}

// ADDI instruction
// rd <- rs1 + imm
#[inline(always)]
fn addi(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let first_operand: i64 = curcpu.read_reg(rs1) as i64;
    let second_operand: i64 = imm12 as i32 as i64;
    curcpu.write_reg(rd, (first_operand + second_operand) as u64);
}

// SLTI instruction
// rd <- (rs1 < imm) ? 1 : 0
#[inline(always)]
fn slti(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let first_operand: i64 = curcpu.read_reg(rs1) as i64;
    let second_operand: i64 = imm12 as i32 as i64;
    if first_operand < second_operand {
        curcpu.write_reg(rd, 0x1);
    } else {
        curcpu.write_reg(rd, 0x0);
    }
}

// SLTIU instruction
// rd <- (unsigned'rs1 < unsigned'imm) ? 1 : 0
#[inline(always)]
fn sltiu(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let first_operand: u64 = curcpu.read_reg(rs1);
    let second_operand: u64 = imm12 as i32 as i64 as u64;
    if first_operand < second_operand {
        curcpu.write_reg(rd, 0x1);
    } else {
        curcpu.write_reg(rd, 0x0);
    }
}

// XORI instruction
// rd <- rs1 ^ imm
#[inline(always)]
fn xori(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let first_operand: i64 = curcpu.read_reg(rs1) as i64;
    let second_operand: i64 = imm12 as i32 as i64;
    curcpu.write_reg(rd, (first_operand ^ second_operand) as u64);
}

// ORI instruction
// rd <- rs1 | imm
#[inline(always)]
fn ori(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let first_operand: i64 = curcpu.read_reg(rs1) as i64;
    let second_operand: i64 = imm12 as i32 as i64;
    curcpu.write_reg(rd, (first_operand | second_operand) as u64);
}

// SLLI instruction
// rd <- unsigned'rs1 << imm
#[inline(always)]
fn slli(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let first_operand: u64 = curcpu.read_reg(rs1);
    let second_operand: u8 = (imm12 & 0x3f) as u8;
    curcpu.write_reg(rd, first_operand << second_operand);
}

// SLLIW instruction
// rd <- unsigned'rs1 << imm
#[inline(always)]
fn slliw(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let first_operand: u64 = curcpu.read_reg(rs1);
    let second_operand: u8 = (imm12 & 0x1f) as u8;
    curcpu.write_reg(rd, first_operand << second_operand);
}

// SRLI and SRAI instruction
// rd <- unsigned'rs1 >> imm (SRLI)
// rd <- signed'rs1 | imm    (SRAI)
#[inline(always)]
fn srli_srai(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let first_operand: u64 = curcpu.read_reg(rs1);
    let second_operand: u8 = (imm12 & 0x3f) as u8;
    // if the 11th bit of the immediate is 0b1 -> SRAI, otherwise SRLI
    if imm12 >> 10 == 0b1 {
        curcpu.write_reg(rd, (first_operand >> second_operand) as u64);
    } else {
        curcpu.write_reg(rd, first_operand >> second_operand);
    }
}

// SRLIW and SRAIW instruction
// rd <- unsigned'rs1 >> imm (SRLI)
// rd <- signed'rs1 | imm    (SRAI)
#[inline(always)]
fn srliw_sraiw(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let first_operand: u64 = curcpu.read_reg(rs1);
    let second_operand: u8 = (imm12 & 0x1f) as u8;
    // if the 11th bit of the immediate is 0b1 -> SRAIW, otherwise SRLIW
    if imm12 >> 10 == 0b1 {
        curcpu.write_reg(rd, (first_operand >> second_operand) as u64);
    } else {
        curcpu.write_reg(rd, first_operand >> second_operand);
    }
}

// ANDI instruction
// rd <- rs1 | imm
#[inline(always)]
fn andi(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let first_operand: i64 = curcpu.read_reg(rs1) as i64;
    let second_operand: i64 = imm12 as i32 as i64;
    curcpu.write_reg(rd, (first_operand & second_operand) as u64);
}

// ADD instruction
// rd <- rs1 + rs2
#[inline(always)]
fn add(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    let first_operand: i64 = curcpu.read_reg(rs1) as i64;
    let second_operand: i64 = curcpu.read_reg(rs2) as i64;
    curcpu.write_reg(rd, (first_operand + second_operand) as u64);
}

// ADDW instruction
// rd <- signed'(rs1[31:0] + rs2[31:0])
#[inline(always)]
fn addw(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    let first_operand: i32 = curcpu.read_reg(rs1) as i32;
    let second_operand: i32 = curcpu.read_reg(rs2) as i32;
    curcpu.write_reg(rd, (first_operand + second_operand) as i64 as u64);
}

// SUB instruction
// rd <- rs1 - rs2
#[inline(always)]
fn sub(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    let first_operand: i64 = curcpu.read_reg(rs1) as i64;
    let second_operand: i64 = curcpu.read_reg(rs2) as i64;
    curcpu.write_reg(rd, (first_operand - second_operand) as u64);
}

// SUBW instruction
// rd <- signed'(rs1[31:0] - rs2[31:0])
#[inline(always)]
fn subw(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    let first_operand: i32 = curcpu.read_reg(rs1) as i32;
    let second_operand: i32 = curcpu.read_reg(rs2) as i32;
    curcpu.write_reg(rd, (first_operand - second_operand) as i64 as u64);
}

// SLL instruction
// rd <- rs1 << rs2[4:0]
#[inline(always)]
fn sll(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    curcpu.write_reg(rd, curcpu.read_reg(rs1) << (curcpu.read_reg(rs2) & 0x3f));
}

// SLLW instruction
// rd <- rs1 << rs2[4:0]
#[inline(always)]
fn sllw(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    let first_operand: u32 = curcpu.read_reg(rs1) as u32;
    let second_operand: u64= curcpu.read_reg(rs2) & 0x1f;
    curcpu.write_reg(rd, (first_operand << second_operand) as u64);
}

// SLT instruction
// rd <- (rs1 < rs2) ? 1 : 0
#[inline(always)]
fn slt(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {

    if (curcpu.read_reg(rs1) as i64) < (curcpu.read_reg(rs2) as i64) {
        curcpu.write_reg(rd, 0b1);
    } else {
        curcpu.write_reg(rd, 0b0);
    }
}

// SLTU instruction
// rd <- (rs1 < rs2) ? 1 : 0
#[inline(always)]
fn sltu(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    if curcpu.read_reg(rs1) < curcpu.read_reg(rs2) {
        curcpu.write_reg(rd, 0b1);
    } else {
        curcpu.write_reg(rd, 0b0);
    }
}

// XOR instruction
// rd <- rs1 xor rs2
#[inline(always)]
fn xor(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    curcpu.write_reg(rd, curcpu.read_reg(rs1) ^ (curcpu.read_reg(rs2)));
}

// OR instruction
// rd <- rs1 xor rs2
#[inline(always)]
fn or(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    curcpu.write_reg(rd, curcpu.read_reg(rs1) | (curcpu.read_reg(rs2)));
}

// AND instruction
// rd <- rs1 xor rs2
#[inline(always)]
fn and(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    curcpu.write_reg(rd, curcpu.read_reg(rs1) & (curcpu.read_reg(rs2)));
}

// FENCE instruction
// Does not do anything because the CPU executes memory accesses in the program order anyway
#[inline(always)]
fn fence() {
    // Placeholder, just in case I have the crazy idea to support OoO execution
}

// FENCEI instruction
// Does not do anything because the CPU executes memory accesses in the program order anyway
#[inline(always)]
fn fencei() {
    // Placeholder, just in case I have the crazy idea to support OoO execution
}

// ECALL and EBREAK instruction
// Not implemented yet
fn ecall_ebreak(imm12: u32) {
    if imm12 & 0x1 == 0x1 {
        // EBREAK
    } else {
        // ECALL
    }
}

// CSRRW instruction
// rd <- csr[imm]
// csr[imm] <- rs1
#[inline(always)]
fn csrrw(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    if rd != Cpu::ZERO_REGISTER {
        curcpu.write_reg(rd, curcpu.read_csreg(imm12 as u16));
    }
    curcpu.write_csreg(imm12 as u16, curcpu.read_reg(rs1));
}

// CSRRS instruction
// rd <- csr[imm]
// csr[imm] <- csr[imm] | rs1
#[inline(always)]
fn csrrs(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let csr_data: u64 = curcpu.read_csreg(imm12 as u16);
    if rd != Cpu::ZERO_REGISTER {
        curcpu.write_reg(rd, csr_data);
    }
    curcpu.write_csreg(imm12 as u16, curcpu.read_reg(rs1) | csr_data);
}

// CSRRC instruction
// rd <- csr[imm]
// csr[imm] <- !csr[imm] & rs1 (clear bits in CSR where rs1 = 1)
#[inline(always)]
fn csrrc(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let csr_data: u64 = curcpu.read_csreg(imm12 as u16);
    if rd != Cpu::ZERO_REGISTER {
        curcpu.write_reg(rd, csr_data);
    }
    curcpu.write_csreg(imm12 as u16, !curcpu.read_reg(rs1) & csr_data);
}

// CSRRWI instruction
// rd <- csr[imm]
// csr[imm] <- unsigned'rs1[4:0]
#[inline(always)]
fn csrrwi(curcpu: &mut Cpu, rs1: u8, rd: RegIndex, imm12: u32) {
    if rd != Cpu::ZERO_REGISTER {
        curcpu.write_reg(rd, curcpu.read_csreg(imm12 as u16));
    }
    curcpu.write_csreg(imm12 as u16, (rs1 & 0x1f) as u64);
}

// CSRRSI instruction
// rd <- csr[imm]
// csr[imm] <- csr[imm] | unsigned'rs1[4:0]
#[inline(always)]
fn csrrsi(curcpu: &mut Cpu, rs1: u8, rd: RegIndex, imm12: u32) {
    let csr_data: u64 = curcpu.read_csreg(imm12 as u16);
    if rd != Cpu::ZERO_REGISTER {
        curcpu.write_reg(rd, csr_data);
    }
    curcpu.write_csreg(imm12 as u16, (rs1 & 0x1f) as u64 | csr_data);
}

// CSRRCI instruction
// rd <- csr[imm]
// csr[imm] <- !csr[imm] & unsigned'rs1[4:0] (clear bits in CSR where rs1 = 1)
#[inline(always)]
fn csrrci(curcpu: &mut Cpu, rs1: u8, rd: RegIndex, imm12: u32) {
    let csr_data: u64 = curcpu.read_csreg(imm12 as u16);
    if rd != Cpu::ZERO_REGISTER {
        curcpu.write_reg(rd, csr_data);
    }
    curcpu.write_csreg(imm12 as u16, !((rs1 & 0x1f) as u64) & csr_data);
}

// SRL instruction
// rd <- rs1 >> rs2[5:0]
#[inline(always)]
fn srl(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    curcpu.write_reg(rd, curcpu.read_reg(rs1) >> (curcpu.read_reg(rs2) & 0x3f));
}

// SRLW instruction
// rd <- rs1 >> rs2[4:0]
#[inline(always)]
fn srlw(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    let first_operand: u32 = curcpu.read_reg(rs1) as u32;
    let second_operand: u64= curcpu.read_reg(rs2) & 0x1f;
    curcpu.write_reg(rd, (first_operand >> second_operand) as u64);
}

// SRA instruction
// rd <- rs1 >> rs2[4:0]
#[inline(always)]
fn sra(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    let first_operand: i64 = curcpu.read_reg(rs1) as i64;
    let second_operand: u64= curcpu.read_reg(rs2) & 0x3f;
    curcpu.write_reg(rd, (first_operand >> second_operand) as u64);
}

// SRAW instruction
// rd <- rs1 >> rs2[4:0]
#[inline(always)]
fn sraw(curcpu: &mut Cpu, rs1: RegIndex, rs2: RegIndex, rd: RegIndex) {
    let first_operand: i32 = curcpu.read_reg(rs1) as i32;
    let second_operand: u64= curcpu.read_reg(rs2) & 0x1f;
    curcpu.write_reg(rd, (first_operand >> second_operand) as u64);
}

// ADDI instruction
// rd <- rs1 + imm
#[inline(always)]
fn addiw(curcpu: &mut Cpu, rs1: RegIndex, rd: RegIndex, imm12: u32) {
    let first_operand: i32 = (curcpu.read_reg(rs1) & 0xffffffff) as i32;
    let second_operand: i32 = imm12 as i32;
    curcpu.write_reg(rd, (first_operand + second_operand) as i64 as u64);
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;
    use crate::rv::*;
    #[test]
    fn add_test() {
        let mut cpu: Cpu = Cpu::new(None);
        let first_op: u64 = 45;
        let second_op: u64 = 50;
        let result = first_op.wrapping_add(second_op);
        cpu.write_reg(1, first_op);
        cpu.write_reg(2, second_op);
        add(&mut cpu, 0x1, 0x2, 0x3);
        assert_eq!(cpu.read_reg(3), result);
    }

    #[test]
    fn sub_test() {
        let mut cpu: Cpu = Cpu::new(None);
        let first_op: u64 = 40;
        let second_op: u64 = 45;
        let result = first_op.wrapping_sub(second_op);
        cpu.write_reg(1, first_op);
        cpu.write_reg(2, second_op);
        sub(&mut cpu, 0x1, 0x2, 0x3);
        assert_eq!(cpu.read_reg(3), result);
    }

    #[test]
    fn jal_test() {
        let mut cpu: Cpu = Cpu::new(None);
        let result = cpu.get_pc().wrapping_sub(5);
        let imm_minus_five: u32 = 0b111111111111_1_1111111011_1_11111111;
        jal(&mut cpu, 0x1, imm_minus_five);
        assert_eq!(cpu.get_next_pc(), result);
    }

    #[test]
    fn beq_test() {
        let mut cpu: Cpu = Cpu::new(None);
        cpu.set_pc(6);
        let result: u64 = cpu.get_pc().wrapping_sub(6);
        let imm12: u32 = 0b11111111111111111111111111100000 as u32;
        let imm5: u32 = 0b10101;
        cpu.write_reg(1, 3);
        cpu.write_reg(2, 3);
        beq(&mut cpu, 0x1, 0x2, imm5, imm12);
        assert_eq!(cpu.get_next_pc(), result);
    }

    #[test]
    fn bne_test() {
        let mut cpu: Cpu = Cpu::new(None);
        cpu.set_pc(6);
        let result: u64 = cpu.get_pc().wrapping_sub(6);
        let imm12: u32 = 0b11111111111111111111111111100000 as u32;
        let imm5: u32 = 0b10101;
        cpu.write_reg(1, 4);
        cpu.write_reg(2, 3);
        bne(&mut cpu, 0x1, 0x2, imm5, imm12);
        assert_eq!(cpu.get_next_pc(), result);
    }

    #[test]
    fn load_test() {
        let mut cpu: Cpu = Cpu::new(None);
        cpu.store(0xdeadbeef, 0x2, AccessSize::WORD);
        lh(&mut cpu, 0x1, 0x2, 0x4);
        assert_eq!(cpu.read_reg(0x2), 0xffffffffffffdead);
    }

    #[test]
    fn store_test() {
        let mut cpu: Cpu = Cpu::new(None);
        cpu.write_reg(0x1, 0xef);
        sb(&mut cpu, 0x0, 0x1, 0x4);
        lbu(&mut cpu, 0x0, 0x2, 0x4);
        assert_eq!(cpu.read_reg(0x1), cpu.read_reg(0x2));
    }


}
