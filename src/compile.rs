use crate::instruction::Instruction;
use crate::word::Word;

// this is for the second pass through the code :)
pub fn asm_to_instruction(line: String) -> Option<Instruction> {
    let mut out = Instruction{bits: [false;16]};
    let line = line.replace(',', "");
    let args: Vec<&str> = line.split_whitespace().collect();
    if args.len() > 0 {
        match args[0] {
            "ADD" => {
                println!("ADD RAN");
                if args.len() != 4 {
                    return None
                }
                for i in 1..=2 {
                    if !args[i].starts_with('R') {
                        return None
                    }
                    let number: u8 = args[i][1..].parse().ok()?;
                    if number > 7 {
                        return None
                    }
                    let reg_bin = regnum_to_bin(number);
                    out.bits[3 * (i-1) + 4] = reg_bin[0];
                    out.bits[3 * (i-1) + 5] = reg_bin[1];
                    out.bits[3 * (i-1) + 6] = reg_bin[2];
                }
                // last few bits:
                if args[3].starts_with('R') {
                    // add 0 0 0 and regnum_to_bin
                    out.bits[10] = false;
                    out.bits[11] = false;
                    out.bits[12] = false;
                    let number: u8 = args[3][1..].parse().ok()?;
                    if number > 7 {
                        return None
                    }
                    let reg_bin = regnum_to_bin(number);
                    out.bits[13] = reg_bin[0];
                    out.bits[14] = reg_bin[1];
                    out.bits[15] = reg_bin[2];

                } else if args[3].starts_with('#') {
                    println!("found #");
                    // add 1 and number to add (it's signed i think)
                    out.bits[10] = true;
                    let number: u16 = args[3][1..].parse().ok()?;
                    if number > 31 {
                        return None
                    }
                    println!("{}", number);
                    //only does unsigned numbers rn, no subtraction lol
                    let num_in_bits: Word = Word::from_u16(number);
                    out.bits[11] = num_in_bits.bits[3];
                    out.bits[12] = num_in_bits.bits[4];
                    out.bits[13] = num_in_bits.bits[5];
                    out.bits[14] = num_in_bits.bits[6];
                    out.bits[15] = num_in_bits.bits[7];
                }
                println!("{:?}", out.bits);
            },
            "AND" => {},
            "JMP" => {},
            "JSR" => {},
            "JSRR" => {},
            "LD" => {},
            "LDI" => {},
            "LDR" => {},
            "LEA" => {},
            "NOT" => {},
            "RET" => {},
            "RTI" => {},
            "ST" => {},
            "STI" => {},
            "STR" => {},
            "TRAP" => {},
            _ => {
                // check if BR, if not then it's a LABEL
            }
        }
    } else {
        todo!(); // smth went wrong??
    }
    Some(out)
}

fn regnum_to_bin(mut input: u8) -> [bool;3] {
    let mut out = [false;3];
    for i in 0..=2 {
        if input >= 2_u8.pow(i) {
            out[i as usize] = true;
            input -= 2_u8.pow(i);
        }
    }
    out
}
