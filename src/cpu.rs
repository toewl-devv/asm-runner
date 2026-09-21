// use this: https://medium.com/@saehwanpark/diving-deeper-into-lc-3-from-opcodes-to-machine-code-4637cf00c878
use crate::{instruction::Instruction, word::Word, alu};

pub struct Cpu {
    registers: [Word; 8],
    pc: usize,
    memory: Vec<Word>
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            registers: [Word::new(); 8],
            pc: 0,
            memory: vec![Word::new(); 2048]
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

            let opcode = instruction.opcode;

            match bin_to_u16(opcode) {
                0 => break,
                1 => {
                    if instruction.steer {
                        todo!()
                    } else {
                        let dest_adr = bin_to_u16(instruction.dest);
                        let sr1 = self.registers[bin_to_u16(instruction.source1) as usize];
                        let sr2 = self.registers[bin_to_u16(instruction.source2) as usize];

                        self.registers[dest_adr as usize] = alu::alu(&sr1, &sr2, false, false, false, false, true, false).out;
                    }
                },
                2 => {},
                3 => {},
                _ => todo!()
            }
        }
    }
}

fn bin_to_u16<const N: usize>(input: [bool; N]) -> u16 {
    let mut value = 0;
    for bit in input {
        value = (value << 1) | bit as u16;
    }
    value
}
