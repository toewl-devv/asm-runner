
// use this: https://medium.com/@saehwanpark/diving-deeper-into-lc-3-from-opcodes-to-machine-code-4637cf00c878
use crate::{instruction::Instruction, word::Word, alu};
use std::collections::HashMap;

pub struct Cpu {
    registers: [Word; 8],
    pc: usize,
    memory: Vec<Word>,
    labels: HashMap<String, usize>
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            registers: [Word::new(); 8],
            pc: 0,
            memory: vec![Word::new(); 2048],
            labels: HashMap::new()
        }
    }

    fn run(&mut self) {
        loop {
            if self.pc as usize >= self.memory.len() {
                println!("PC out of bounds, halting");
                break;
            }

            let instruction = Instruction::from_word(&self.memory[self.pc]);

            self.pc += 1;

            let opcode = &instruction.bits[0..=3];

            match bin_to_u16(opcode) {
                0 => {},
                1 => {
                    let steer = instruction.bits[10];
                    let dest_adr = bin_to_u16(&instruction.bits[4..=6]);
                    let sr1 = self.registers[bin_to_u16(&instruction.bits[7..=9]) as usize];

                    if steer {
                        let arr_to_add: [bool; 5] = instruction.bits[11..=15].try_into().unwrap();
                        let to_add = Word::from_array(arr_to_add);
                        self.registers[dest_adr as usize] = alu::alu(&sr1, &to_add,false, false, false, false, true, false).out;
                    } else {
                        let sr2 = self.registers[bin_to_u16(&instruction.bits[13..=15]) as usize];

                        self.registers[dest_adr as usize] = alu::alu(&sr1, &sr2, false, false, false, false, true, false).out;
                    }
                },
                2 => { // LOAD or LD
                    let dest_adr = bin_to_u16(&instruction.bits[4..=6]);
                    let pcoffset9_bits = &instruction.bits[7..=15];
                    let pcoffset = bits_to_signed(pcoffset9_bits);
                    let address = self.pc as i32 + pcoffset as i32;
                    let value = self.memory[address as usize];

                    self.registers[dest_adr as usize] = value;
                },
                3 => { // STORE or ST
                    let source_adr = bin_to_u16(&instruction.bits[4..=6]);
                    let pcoffset9_bits = &instruction.bits[7..=15];
                    let pcoffset = bits_to_signed(pcoffset9_bits);
                    let address = self.pc as i32 + pcoffset as i32;
                    let value = self.registers[source_adr as usize];

                    self.memory[address as usize] = value;
                },
                4 => {},
                5 => {
                    let steer = instruction.bits[10];
                    let dest_adr = bin_to_u16(&instruction.bits[4..=6]);
                    let sr1 = self.registers[bin_to_u16(&instruction.bits[7..=9]) as usize];

                    if steer {
                        let arr_to_add: [bool; 5] = instruction.bits[11..=15].try_into().unwrap();
                        let to_add = Word::from_array(arr_to_add);
                        self.registers[dest_adr as usize] = alu::alu(&sr1, &to_add,false, false, false, false, false, false).out;
                    } else {
                        let sr2 = self.registers[bin_to_u16(&instruction.bits[13..=15]) as usize];

                        self.registers[dest_adr as usize] = alu::alu(&sr1, &sr2, false, false, false, false, false, false).out;
                    }
                },
                6 => {},
                7 => {},
                8 => {},
                9 => { // NOT
                    let dest_adr = bin_to_u16(&instruction.bits[4..=6]);
                    let sr1 = self.registers[bin_to_u16(&instruction.bits[7..=9]) as usize];
                    self.registers[dest_adr as usize] = alu::alu(&sr1, &sr1, false, false, true, true, false, true).out;
                },
                10 => {},
                11 => {},
                12 => {},
                13 => {},
                14 => { // LEA (Load Effective Address
                    let dest_adr = bin_to_u16(&instruction.bits[4..=6]);
                    let pcoffset9_bits = &instruction.bits[7..=15];
                    let pcoffset = bits_to_signed(pcoffset9_bits);
                    let address = (self.pc as i32 + pcoffset as i32) as u16;

                    self.registers[dest_adr as usize] = Word::from_u16(address);
                },
                _ => todo!()
            }
        }
    }
}

fn bin_to_u16(input: &[bool]) -> u16 {
    let mut value = 0;
    for bit in input {
        value = (value << 1) | *bit as u16;
    }
    value
}

fn bits_to_signed(bits: &[bool]) -> i16 {
    let mut value: i16 = 0;

    for &bit in bits {
        value = (value << 1) | bit as i16;
    }

    if bits[0] {
        value - 512
    } else {
        value
    }
}
