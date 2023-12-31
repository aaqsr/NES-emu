use crate::memory::Mem;
use crate::CPU::CPU;

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
#[allow(dead_code)]
pub enum AddressingMode {
    // actual values are used
    // For example,
    // LDX #$01 loads the value $01 into the X register in Immediate
    // different to the zero page instruction LDX $01 which loads the
    // value at memory location $01 into the X register
    Immediate,

    // has a size of 2 bytes, one for opcode itself, and one for a parameter
    // the parameter is read in from the memory pointed to by the second byte
    // can't reference memory above the first 255 bytes
    // faster, as only one byte needs to be looked up
    ZeroPage,

    // a zero page address is given, and then the value of the X register is added
    // if the result of the addition is larger than a single byte, the address wraps around
    // i.e: the second byte is de-referenceed and then the val of the X register is added to it
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
    pub(super) fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.mem_read(self.program_counter).into(),

            AddressingMode::ZeroPage_X => {
                let zero_page_addr = self.mem_read(self.program_counter);
                zero_page_addr.wrapping_add(self.register_x) as u16
            }

            AddressingMode::ZeroPage_Y => {
                let zero_page_addr = self.mem_read(self.program_counter);
                zero_page_addr.wrapping_add(self.register_y) as u16
            }

            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

            AddressingMode::Absolute_X => {
                let abs_addr = self.mem_read_u16(self.program_counter);
                abs_addr.wrapping_add(self.register_x as u16)
            }

            AddressingMode::Absolute_Y => {
                let abs_addr = self.mem_read_u16(self.program_counter);
                abs_addr.wrapping_add(self.register_y as u16)
            }

            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                // note: we have to do this due to little endian
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }

            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                deref_base.wrapping_add(self.register_y as u16)
            }

            AddressingMode::NoneAddressing => {
                panic!("Invalid addressing mode! Mode: {:?} is not supported", mode)
            }
        }
    }
}
