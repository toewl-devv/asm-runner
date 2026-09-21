#[derive(Clone, Copy)]
pub struct Word {
    pub bits: [bool; 16]
}

impl Word {
    pub fn new() -> Self {
        Word {bits: [false; 16]}
    }

    fn from_u16(mut value: u16) -> Self {
        
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

    fn to_string(&self) -> String {
        let mut out: String = String::new();
        for x in self.bits {
            out.push(if x {'1'} else {'0'});
        }
        out
    }
}
