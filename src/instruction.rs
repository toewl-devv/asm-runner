use crate::word::Word;

pub struct Instruction {
    pub bits: [bool;16]
}

impl Instruction {
    pub fn from_word(word: &Word) -> Self {
        Self { 
             bits: word.bits
        }
    }
}

