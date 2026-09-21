use crate::word::Word;


pub struct ALUResult {
    pub out: Word,
    pub zr: bool,
    pub ng: bool
}

pub fn alu(
    x: &Word, 
    y: &Word, 
    zx: bool, 
    nx: bool, 
    zy: bool, 
    ny: bool,
    f: bool,
    no: bool
) -> ALUResult {
    let mut x = *x;
    let mut y = *y;
    if zx { x = Word::new(); }
    if nx { x = bitwise_not(x); }
    if zy { y = Word::new(); }
    if ny { y = bitwise_not(y); }
    let mut out = if f {
        adder(&x, &y)
    } else {
        bitwise_and(&x, &y)
    };
    if no { out = bitwise_not(out); }

    let zr = out.bits == Word::new().bits;
    let ng = out.bits[0];

    ALUResult{out: out, zr: zr, ng: ng}
}

struct AdditionResult {
    sum: bool,
    carry: bool
}

fn full_adder(a: bool, b: bool, c: bool) -> AdditionResult {
    AdditionResult {
        sum: a ^ b ^ c,
        carry: (a&&b) || (b&&c) || (c&&a)
    }
}

fn adder(a: &Word, b: &Word) -> Word {
    let mut carry = false;
    let mut out = Word { bits: [false; 16] };

    for i in (0..=15).rev() {
        let addition = full_adder(a.bits[i], b.bits[i], carry);
        out.bits[i] = addition.sum;
        carry = addition.carry;
    }
    out
}

fn bitwise_and(a: &Word, b: &Word) -> Word {
    let mut out = Word { bits: [false; 16] };
    for i in 0..=15 {
        out.bits[i] = a.bits[i] && b.bits[i];
    }
    out
}

fn bitwise_not(mut a: Word) -> Word {
    for bit in &mut a.bits {
        *bit = !*bit;
    }
    a
}
