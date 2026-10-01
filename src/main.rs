mod word;
mod cpu;
mod alu;
mod instruction;
mod compile;

use std::fs;

fn main() {
    let mut maincpu = cpu::Cpu::new();
    
    let file_path = "src/program.asm";
    let lines = if fs::exists(file_path).unwrap() {
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        contents
            .split('\n')
            .map(String::from)
            .collect()
    } else {
        vec![String::new()]
    };
    println!("{:?}", lines);

    maincpu.make_labels_hashmap(lines);

    // println!("{:?}", maincpu.labels);

    /*
    let line1 = "ADD R0, R0, #17".to_string();
    let line2 = "ADD R1, R1, #5".to_string();
    let line3 = "ADD R0, R0, R1".to_string();

    let ins1 = compile::asm_to_instruction(line1.clone()).unwrap();
    let ins2 = compile::asm_to_instruction(line2.clone()).unwrap();
    let ins3 = compile::asm_to_instruction(line3.clone()).unwrap();
    
    let mut maincpu = cpu::Cpu::new();

    maincpu.memory[0] = word::Word::from_instruction(&ins1);
    maincpu.memory[1] = word::Word::from_instruction(&ins2);
    maincpu.memory[2] = word::Word::from_instruction(&ins3);

    maincpu.run();

    println!("{}", line1);
    println!("{}", line2);
    println!("{}", line3);

    println!("R0: {}", maincpu.registers[0].to_string());
    */
}
