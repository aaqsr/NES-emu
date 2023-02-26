/*
 *  DATA:
 *
 *      The CPU consists of the following pieces of data
 *
 *      Three primary registers:
 *
 *      A : Accumulator - 8 bit
 *      X : Register - 8 Bit
 *      Y : Register - 8 Bit
 *
 *
 *      stkp : The stack pointer
 *          8 bit number pointing to somewhere in memory
 *          incremented/decremented as things pushed/pulled from stack
 *
 *      pc : The program counter
 *          16 bit counter
 *          stores bit of the next program byte to be read/exec by CPU
 *
 *      status: The status register
 *          stores bits that let us interrogate the state of the CPU (packaged into an 8 bit word)
 *          represents 7 status flags that can be set or unset depending on the result of the last executed instruction
 *
 *  How it works:
 *
 *      Naive idea:
 *
 *          Each time the CPU is clocked, it outputs the pc onto the bus, recieves 1 byte of
 *          instruction, executes it, and outputs some data
 *
 *      However,
 *
 *          Not all instructions are the same length
 *              they may be 1, 2, or 3 bytes
 *
 *          thus the pc isn't simply incremented per instruction
 *
 *          since multiple things done per instruction, we need several clock cycles, and not
 *          just one
 *
 *              i.e. different instructions take different number of clock cycles to execute
 *
 *      Therefore we need to implement an instruction's function, it's address mode, and it's
 *      cycles
 *      so, per instruction, we need to consider the size of the instruction, and its duration as
 *      well. The first byte of instruction provides us with this info
 *
 *      eg:
 *
 *          CLC (clear the carry bit) is just 1-Byte
 *          LDA $41 (load the Accumulator with hex 41) is 2 Bytes, 1 Byte for the instruction, the
 *          other for the number
 *          LDA $0105 (load the Accumulator with a value from memory) is 3 Bytes, 1 for
 *          instruction, 2 for the 16 bit address
 *
 *
 *      We have 256 instructions
 *
 */

/*
 *  How our CPU class works
 *
 *      -> Read Byte @ program counter
 *
 *  The CPU works in a constant cycle:
 *
 *  Fetch next execution instruction from the instruction memory
 *  Decode the instruction
 *  Execute the Instruction
 *  Repeat the cycle
 *
 */

use crate::bus;

pub struct CPU<'a> {
    pub register_a: u8,

    pub status: u8,
    pub program_counter: u16,

    bus: &'a bus::Bus<'a>,
}

impl CPU<'_> {
    pub fn connect_bus(&mut self, n: &bus::Bus) {
        self.bus = n;
    }

    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
            bus: None,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];

            match opscode {
                // BRK
                // break. Stop interpretting the program
                0x00 => return,

                _ => todo!(),
            }
        }
    }
}
