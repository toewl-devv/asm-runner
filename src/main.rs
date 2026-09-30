mod word;
mod cpu;
mod alu;
mod instruction;
mod compile;

fn main() {
    let line = "ADD R0, R0, #5".to_string();

    let add_instr = compile::asm_to_instruction(line).unwrap();
    println!("{}", word::Word::from_instruction(&add_instr).to_string());
    
    let mut maincpu = cpu::Cpu::new();

    maincpu.memory[0] = word::Word::from_instruction(&add_instr);

    maincpu.run();

    println!("{}", maincpu.registers[0].to_string())
}
