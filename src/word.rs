use crate::instruction::Instruction;

#[derive(Clone, Copy)]
pub struct Word {
    pub bits: [bool; 16]
}

impl Word {
    pub fn new() -> Self {
        Word {bits: [false; 16]}
    }

    pub fn from_u16(mut value: u16) -> Self {
        
        let mut out = Word::new();

        for i in 0..=15 {
            let power = 2_u16.pow(15 - i);

            if value >= power {
                out.bits[i as usize] = true;
                value -= power;
            }
        }
        out
    }

    pub fn to_string(&self) -> String {
        let mut out: String = String::new();
        for x in self.bits {
            out.push(if x {'1'} else {'0'});
        }
        out
    }

    pub fn from_array<const N: usize>(input: [bool;N]) -> Self {
        let mut out = Word::new();
        for i in 0..= input.len() {
            if input.len()-i <= input.len() { 
                out.bits[i] = false; 
            } else {
                out.bits[i] = input[input.len()-i];
            }
        }
        out
    }

    pub fn from_instruction(instruction: &Instruction) -> Self {
        Self {
            bits: instruction.bits
        }
    }
}
