mod addressing_modes;
mod instructions;
mod memory;
mod opcodes;

#[allow(unused_imports)]
use crate::CPU::{
    addressing_modes::AddressingMode, instructions::*, memory::Mem, opcodes::OPCODES_MAP,
};

use bitflags::bitflags;

use std::collections::HashMap;

// Very cool crate!
bitflags! {
    // processor status
    // 8-bit register represents 7 status flags that can be
    // set or unset depending on the result of the last executed instruction
    //
    // In order from right to left,
    // Carry Flag, Zero Flag, Interrupt Disable, Decimal Mode Flag
    // Break Command, Overflow Flag, Negative Flag
    //
    //  7 6 5 4 3 2 1 0
    //  N V _ B D I Z C
    //  | |   | | | | +--- Carry Flag
    //  | |   | | | +----- Zero Flag
    //  | |   | | +------- Interrupt Disable
    //  | |   | +--------- Decimal Mode (not used on NES)
    //  | |   +----------- Break Command
    //  | +--------------- Overflow Flag
    //  +----------------- Negative Flag

    pub struct CPUFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK2            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIV           = 0b10000000;
    }
}

#[allow(non_snake_case)]
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
    pub status: CPUFlags,

    // program counter
    // holds the address for the next machine language instruction
    pub program_counter: u16,

    // temporary ram
    // CPU has only 2 KiB of RAM, and everything else is reserved for memory mapping
    memory: [u8; 0xFFFF],
    // pub super so that memory trait can be implemented elsewhere
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
            status: CPUFlags::from_bits_truncate(0b100100),
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    // Device operations

    // inserting a new cartridge -> CPU receives a special signal called "Reset interrupt"
    // instructs CPU to:
    // - reset the state (registers and flags)
    // - set program_counter to the 16-bit address that is stored at 0xFFFC
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = CPUFlags::from_bits_truncate(0b100100);

        self.program_counter = self.mem_read_u16(0xFFFC);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>) {
        // [0x8000 .. 0xFFFF] is reserved for Program ROM
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn run(&mut self) {
        let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *OPCODES_MAP;

        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            let opcode = opcodes
                .get(&code)
                .expect(&format!("OpCode {:x} is not recognized", code));

            let mode = &opcode.mode;

            match code {
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => self.adc(mode),

                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(mode),

                0x0A | 0x06 | 0x16 | 0x0E | 0x1E => self.asl(mode),

                // Branching
                0x90 => self.bcc(),
                0xB0 => self.bcs(),
                0xF0 => self.beq(),
                0x30 => self.bmi(),
                0xD0 => self.bne(),
                0x10 => self.bpl(),
                0x50 => self.bvc(),
                0x70 => self.bvs(),

                0x24 | 0x2C => self.bit(mode),

                // Break but wrong
                0x00 => return,

                0x18 => self.clc(),
                0xD8 => self.cld(),
                0x58 => self.cli(),
                0xB8 => self.clv(),

                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => self.lda(mode),

                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => self.sta(mode),

                0xAA => self.tax(),
                0xe8 => self.inx(),
                _ => todo!(),
            }

            // Update the PC accordingly
            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.len - 1) as u16;
            }
        }
    }
}
