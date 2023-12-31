use crate::addressing_modes::AddressingMode;
use crate::memory::Mem;
use crate::opcodes;

use std::collections::HashMap;

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
  // CPU has only 2 KiB of RAM, and everything else is reserved for memory mapping
  pub(super) memory: [u8; 0xFFFF],
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
      status: 0,
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
    self.status = 0;

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
    let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

    loop {
      let code = self.mem_read(self.program_counter);
      self.program_counter += 1;
      let program_counter_state = self.program_counter;

      let opcode = opcodes
        .get(&code)
        .expect(&format!("OpCode {:x} is not recognized", code));

      match code {
        0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => {
          self.lda(&opcode.mode);
        }

        /* STA */
        0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
          self.sta(&opcode.mode);
        }

        0xAA => self.tax(),
        0xe8 => self.inx(),
        0x00 => return,
        _ => todo!(),
      }

      if program_counter_state == self.program_counter {
        self.program_counter += (opcode.len - 1) as u16;
      }
    }
  }
}
