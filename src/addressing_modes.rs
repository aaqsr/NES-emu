use super::*;

// The NES was nice enough to use different addressing modes
// i.e different ways to get a parameter for an instruction
// based on the instruction

// a property of an instruction that defines how the CPU should interpret
// the next 1 or 2 bytes in the instruction stream

// different addressing modes have different instruction sizes

// CPU instruction size can be either 1, 2, or 3 bytes.
// no opcodes that occupy more than 3 bytes

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    // actual values are used
    // For example,
    // LDX #$01 loads the value $01 into the X register in Immediate
    // different to the zero page instruction LDX $01 which loads the
    // value at memory location $01 into the X register
    Immediate,

    // has a size of 2 bytes, one for opcode itself, and one for a parameter
    // can't reference memory above the first 255 bytes
    // faster, as only one byte needs to be looked up
    ZeroPage,

    // a zero page address is given, and then the value of the X register is added
    // if the result of the addition is larger than a single byte, the address wraps around
    ZeroPage_X,

    // same but for Y register
    // can only be used with LDX and STX
    ZeroPage_Y,

    // the full memory location is used as the argument to the instruction
    // has 3 bytes, the Address occupies 2 bytes making it possible to
    // reference all 65536 memory cells
    Absolute,

    // absolute addressing versions of zero page,X and zero page,Y
    Absolute_X,
    Absolute_Y,

    // uses an absolute address to look up another address.
    // first address gives the least significant byte of the address, the
    // following byte gives the most significant byte
    Indirect_X,
    Indirect_Y,

    // none
    NoneAddressing,
}

impl CPU {
    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            _ => todo!(),
        }
    }
}
