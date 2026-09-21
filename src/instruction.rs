use crate::word::Word;

pub struct Instruction {
    pub opcode: [bool; 4],
    pub dest: [bool; 3],
    pub source1: [bool; 3],
    pub steer: bool,
    pub padding: [bool; 2],
    pub source2: [bool; 3]
}

impl Instruction {
    pub fn from_word(word: &Word) -> Self {
        let bits = word.bits;
        Self { 
            opcode: [bits[0], bits[1], bits[2], bits[3]],
            dest: [bits[4], bits[5], bits[6]],
            source1: [bits[7], bits[8], bits[9]], 
            steer: bits[10], 
            padding: [bits[11], bits[12]], 
            source2: [bits[13], bits[14], bits[15]] 
        }
    }
}

