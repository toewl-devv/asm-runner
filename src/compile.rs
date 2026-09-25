use crate::instruction::Instruction;

fn asm_to_instruction(line: String) -> Option<Instruction> {
    let mut out = Instruction{bits: [false;16]};
    let line = line.replace(',', "");
    let args: Vec<&str> = line.split_whitespace().collect();
    if args.len() > 0 {
        match args[0] {
            "ADD" => {
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
                    out.bits[3 * (i-1) + 5] = reg_bin[0];
                    out.bits[3 * (i-1) + 6] = reg_bin[0];
                }
                // last few bits:
                if args[3].starts_with('R') {
                    // add 0 0 0 and regnum_to_bin
                } else if args[3].starts_with('#') {
                    // add 1 and number to add (it's signed i think)
                }
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
