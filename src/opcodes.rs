use crate::addressing_modes::AddressingMode;

pub struct Opcode {
    pub code: u8,
    pub assembly: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl Opcode {
    fn new(code: u8, assembly: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        Opcode {
            code,
            assembly,
            len,
            cycles,
            mode,
        }
    }
}
