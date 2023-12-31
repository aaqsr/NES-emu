use crate::addressing_modes::AddressingMode;
use crate::memory::Mem;
use crate::CPU::CPUFlags;
use crate::CPU::CPU;

impl CPU {
    // Command Helpers

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status.insert(CPUFlags::ZERO);
        } else {
            self.status.remove(CPUFlags::ZERO);
        }

        if result & 0b1000_0000 != 0 {
            self.status.insert(CPUFlags::NEGATIV);
        } else {
            self.status.remove(CPUFlags::NEGATIV);
        }
    }

    // set if the result has yielded an invalid 2's complement result
    // (e.g. adding to positive numbers and ending up with a negative result: 64 + 64 => -128)
    fn update_overflow_flag(&mut self, arg1: u8, arg2: u8, result: u8) {
        // This if statement is hard to explain, but it works if you do the math
        //
        // For proof consider the cases in which overflow may occur:
        //  1. The two numbers were positive and we got a negative number
        //  2. The two numbers were negative and we got a positive number
        //
        // Let the numbers X, Y, and the result R be sequences of bits xi, yi, and ri such that
        //  X = x7x6...x0, Y = y7y6...y0 and R = r7r6...r0
        //
        // If X, Y are positive then x7 = 0 and y7 = 0.
        // Then if we have overflowed, r7 = 1, and then x7 XOR r7 = 1 and y7 XOR r7 = 1
        // so regardless of the values of the other bits we get, 1 AND 1 AND 1 = 1 and
        // the overflow flag is set.
        //
        // And then if we did not overflow then r7 = 0, and x7 XOR r7 = 0, y7 XOR r7 = 0,
        // and 0 AND 0 AND 1 = 0.
        //
        //
        // Now if X, Y are negative then x7 = 1, and y7 = 1.
        // Then if we have underflowed, r7 = 0, and then x7 XOR r7 = 1 and y7 XOR r7 = 1
        // and a similar argument follows.
        //
        // And then if we did not overflow then r7 = 1, and x7 XOR r7 = 0, y7 XOR r7 = 10
        // and a similar argument follows.
        if (arg1 ^ result) & (arg2 ^ result) & 0b1000_0000 != 0 {
            self.status.insert(CPUFlags::OVERFLOW)
        } else {
            self.status.remove(CPUFlags::OVERFLOW)
        }
    }

    fn add_to_reg_a(&mut self, arg: u8) {
        // Add in a bigger container
        let bigres: u16 = (arg as u16)
            + (self.register_a as u16)
            + (self.status.contains(CPUFlags::CARRY) as u16);

        // So we can check for carry by comparing with largest u8
        if bigres > 0xff {
            self.status.insert(CPUFlags::CARRY)
        } else {
            self.status.remove(CPUFlags::CARRY)
        }

        // truncating conversion
        let res = bigres as u8;

        self.update_overflow_flag(self.register_a, arg, res);

        self.update_zero_and_negative_flags(res);
        self.register_a = res;
    }

    // Commands

    // adds the contents of a memory location to the accumulator together with the carry bit. If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.
    pub(super) fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.add_to_reg_a(value);
    }

    // Loads a byte of memory (value) into the accumulator
    // and sets the zero and negative flags as appropriate
    pub(super) fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    // Store address into register A
    pub(super) fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    // Copies the current contents of the accumulator into the X register
    // and sets the zero and negative flags as appropriate
    pub(super) fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    // Adds one to the X register
    // and sets the zero and negative flags as appropriate
    pub(super) fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }
}
