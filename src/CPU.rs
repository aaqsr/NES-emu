pub struct CPU {
    // the accumulator
    // stores the results of arithmetic, logic, and memory access operations
    // used as an input parameter for some operations
    pub register_a: u8,

    // stack pointer
    // memory space [0x0100 .. 0x1FF] is used for stack
    // holds the address of the top of that space
    // pub stack: u8;

    // index register x
    // used as an offset in specific memory addressing modes
    // can be used for auxiliary storage needs
    pub register_x: u8,

    // index register y
    // similar to x
    pub register_y: u8,

    // processor status
    // 8-bit register represents 7 status flags that can be
    // set or unset depending on the result of the last executed instruction
    //
    // In order from right to left,
    // Carry Flag, Zero Flag, Interrupt Disable, Decimal Mode Flag
    // Break Command, Overflow Flag, Negative Flag
    pub status: u8,

    // program counter
    // holds the address for the next machine language instruction
    pub program_counter: u16,

    // temporary ram
    memory: [u8; 0xFFFF],
}

// CPU works in a constant cycle:

// Fetch next execution instruction from the instruction memory
// Decode the instruction
// Execute the Instruction
// Repeat the cycle

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x8000;
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    // Loads a byte of memory (value) into the accumulator
    // and sets the zero and negative flags as appropriate
    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    // Copies the current contents of the accumulator into the X register
    // and sets the zero and negative flags as appropriate
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    // Adds one to the X register
    // and sets the zero and negative flags as appropriate
    fn inx(&mut self) {
        if self.register_x == 0xFF {
            self.register_x = 0;
            return;
        }

        self.register_x += 1;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn run(&mut self) {
        loop {
            let opscode = self.mem_read(self.program_counter);

            self.program_counter += 1;

            match opscode {
                // LDA
                // A,Z,N = M
                0xA9 => {
                    let param = self.memory[self.program_counter as usize];
                    self.program_counter += 1;
                    self.lda(param);
                }

                // TAX
                // X = A
                0xAA => self.tax(),

                // INX
                // X,Z,N = X+1
                0xE8 => self.inx(),

                // BRK
                // stop execution
                0x00 => {
                    self.status |= 0b0010_0000;
                    return;
                }

                _ => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
    #[test]
    fn break_sets_break_register() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0x00]);
        assert_ne!(cpu.status & 0b0010_0000, 0);
    }
}
