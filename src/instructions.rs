use crate::addressing_modes::AddressingMode;
use crate::memory::Mem;
use crate::CPU::CPUFlags;
use crate::CPU::CPU;

impl CPU {
    // Command Helpers

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        // If 0 then set zero
        self.status.set(CPUFlags::ZERO, result == 0);

        // if negative then set negative
        self.status.set(CPUFlags::NEGATIV, result >> 7 == 1);
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
        let condition = (arg1 ^ result) & (arg2 ^ result) & 0b1000_0000 != 0;

        self.status.set(CPUFlags::OVERFLOW, condition)
    }

    fn add_to_reg_a(&mut self, arg: u8) {
        // Add in a bigger container
        let bigres: u16 = (arg as u16)
            + (self.register_a as u16)
            + (self.status.contains(CPUFlags::CARRY) as u16);

        // So we can check for carry by comparing with largest u8
        self.status.set(CPUFlags::CARRY, bigres > 0xff);

        // truncating conversion
        let res = bigres as u8;

        self.update_overflow_flag(self.register_a, arg, res);

        self.update_zero_and_negative_flags(res);
        self.register_a = res;
    }

    // Commands

    // Adds the contents of a memory location to the accumulator together with the carry bit.
    // If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.
    pub(super) fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.add_to_reg_a(value);
    }

    // logical AND on the accumulator contents using the contents of a byte of memory
    pub(super) fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value & self.register_a;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn bit_shift_left_and_set_flags(&mut self, value: u8) -> u8 {
        // Set carry flag
        self.status.set(CPUFlags::CARRY, value >> 7 == 1);
        let res = value << 1;
        self.update_zero_and_negative_flags(res);
        res
    }

    // shifts all the bits of the accumulator or memory contents one bit left
    // Bit 0 is set to 0 and bit 7 is placed in the carry flag
    pub(super) fn asl(&mut self, mode: &AddressingMode) {
        if *mode == AddressingMode::NoneAddressing {
            // we have to deal with the accumulator
            self.register_a = self.bit_shift_left_and_set_flags(self.register_a);
        } else {
            // Read from memory
            let addr = self.get_operand_address(mode);
            let val = self.mem_read(addr);
            let res = self.bit_shift_left_and_set_flags(val);
            // println!("writing at {addr}: val was {val}, res is {res}");
            self.mem_write(addr, res);
        };
    }

    // if predicate if true then add the relative displacement to the program counter
    // to cause a branch to a new location
    fn add_next_val_to_pc_if(&mut self, predicate: bool) {
        if predicate {
            let jmp = self.mem_read(self.program_counter);
            self.program_counter = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(jmp as u16);
        }
    }

    // branch if the carry flag is clear
    pub(super) fn bcc(&mut self) {
        self.add_next_val_to_pc_if(!self.status.contains(CPUFlags::CARRY));
    }

    // branch if the carry flag is set
    pub(super) fn bcs(&mut self) {
        self.add_next_val_to_pc_if(self.status.contains(CPUFlags::CARRY));
    }

    // branch if the zero flag is set
    pub(super) fn beq(&mut self) {
        self.add_next_val_to_pc_if(self.status.contains(CPUFlags::ZERO));
    }

    // branch if the negative flag is set
    pub(super) fn bmi(&mut self) {
        self.add_next_val_to_pc_if(self.status.contains(CPUFlags::NEGATIV));
    }

    // branch if the zero flag is clear
    pub(super) fn bne(&mut self) {
        self.add_next_val_to_pc_if(!self.status.contains(CPUFlags::ZERO));
    }

    // branch if the negative flag is clear
    pub(super) fn bpl(&mut self) {
        self.add_next_val_to_pc_if(!self.status.contains(CPUFlags::NEGATIV));
    }

    // branch if overflow flag is clear
    pub(super) fn bvc(&mut self) {
        self.add_next_val_to_pc_if(!self.status.contains(CPUFlags::OVERFLOW));
    }

    // branch if overflow flag is set
    pub(super) fn bvs(&mut self) {
        self.add_next_val_to_pc_if(self.status.contains(CPUFlags::OVERFLOW));
    }

    // Test if one or more bits are set in a target memory location
    // Mask pattern in A is ANDed with the value in memory to set or clear the zero flag,
    // but the result is not kept
    // Bits 6 and 7 of the value from memory are copied into the V and N flags
    pub(super) fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        if (self.register_a & value) == 0 {
            self.status.insert(CPUFlags::ZERO);
        } else {
            self.status.remove(CPUFlags::ZERO);
        }

        self.status.set(CPUFlags::OVERFLOW, value >> 6 == 1);
        self.status.set(CPUFlags::NEGATIV, value >> 7 == 1);
    }

    // Forces the generation of an interrupt request
    // Program counter and processor status are pushed on the stack
    // IRQ interrupt vector at $FFFE/F is loaded into PC
    // and the break flag is set to one
    pub(super) fn brk(&mut self) {
        todo!();
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
