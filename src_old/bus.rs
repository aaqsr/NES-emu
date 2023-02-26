use crate::CPU;

pub struct Bus<'a> {
    // devices
    cpu: CPU::CPU<'a>,

    // fake RAM for now
    ram: [u8; 64 * 1024],
}

impl Bus<'_> {
    pub fn write(&mut self, addr: u16, data: u8) {
        if addr >= 0x0000 && addr <= 0xFFFF {
            // this looks useless ^ , but is useful later
            self.ram[addr as usize] = data;
        }
    }

    pub fn read(&mut self, addr: u16, read_only: bool) -> Option<u8> {
        if addr >= 0x0000 && addr <= 0xFFFF {
            // this looks useless ^ , but is useful later
            return Some(self.ram[addr as usize]);
        }

        // if reading out of the range
        None
    }
}
