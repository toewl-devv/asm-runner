use crate::instruction::Instruction;

fn asm_to_instruction(line: String) -> Instruction {
    let mut out = Instruction{bits: [false;16]};
    let args: Vec<&str> = line.split(" ").collect();
    if args.len() > 0 {
        match args[0] {
            "ADD" => {},
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
    out
}
