use crate::CPU::CPUFlags;
#[cfg(test)]
use crate::CPU::CPU;

// Pro tip: Use the mac os calculator in programmer mode by going to View > Programmer

#[test]
fn test_add_with_carry_overflow() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, // lda
        0x40, // 64
        0x69, // adc
        0x40, // 64
        0x00, // brk
    ]);

    assert_eq!(cpu.register_a, 0x80);
    assert!(cpu.status.contains(CPUFlags::OVERFLOW));
    assert!(!cpu.status.contains(CPUFlags::CARRY));
}

#[test]
fn test_asl_adc_carry() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, // lda
        0x60, 0x0A, // asl of acc
        0x69, // adc
        0xC0, 0x00, // brk
    ]);

    assert_eq!(cpu.register_a, 0x80); // 0x180
    assert!(cpu.status.contains(CPUFlags::CARRY));
    assert!(!cpu.status.contains(CPUFlags::OVERFLOW));
}

#[test]
fn test_asl_adc_carry_2() {
    let mut cpu = CPU::new();
    // NES CPU uses Little-Endian addressing!
    cpu.load_and_run(vec![
        0xA9, // lda
        0xFE, // value
        0x0E, // asl in absolute
        0x09, // memory address of val bottom
        0x80, // memory address of val top
        0x6D, // adc in absolute
        0x09, // memory address of val bottom
        0x80, // mem address of val top
        0x00, // brk
        0x7F, // 127
    ]);

    assert_eq!(cpu.register_a, 0xFC); // should be 0x1FC truncated
    assert!(cpu.status.contains(CPUFlags::CARRY));
    assert!(!cpu.status.contains(CPUFlags::OVERFLOW));
}

#[test]
fn test_and() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, // lda
        0xFF, // all 1s
        0x29, // AND
        0xA0, // 1010 0000
        0x00
    ]);

    assert_eq!(cpu.register_a, 0xA0);
    assert!(cpu.status.contains(CPUFlags::NEGATIV));
}

#[test]
fn test_0xa9_lda_immidiate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x05, 0x00]);
    assert_eq!(cpu.register_a, 0x05);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
    assert!(cpu.status.bits() & 0b1000_0000 == 0);
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x00, 0x00]);
    assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
}

#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x0A, 0xAA, 0x00]);

    assert_eq!(cpu.register_x, 10)
}

#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}

#[test]
fn test_inx_overflow() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0xFF, 0xAA, 0xE8, 0xE8, 0x00]);

    assert_eq!(cpu.register_x, 1)
}

// Invalid test
// #[test]
// fn break_sets_break_register() {
//     let mut cpu = CPU::new();
//     cpu.load_and_run(vec![0x00]);
//     assert_ne!(cpu.status & 0b0010_0000, 0);
// }
