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
        }
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];

            self.program_counter += 1;

            match opscode {
                // LDA
                // A,Z,N = M
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.lda(param);
                }

                // TAX
                // X = A
                0xAA => self.tax(),

                0x00 => return,

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
}
