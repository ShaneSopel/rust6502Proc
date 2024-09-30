
use crate::cpuproc::instruction::INSTRUCTIONS;
use crate::cpuproc::match_addr;
use crate::cpuproc::match_process;
use crate::cpuproc::inst_cycles;

// so we need to process the instruction set
// the instruction, addr mode, and cycles.
pub fn process_instruction(opcode: u8) -> bool
{
    let opcode_converter = usize::from(opcode);
    let opcode_instruction = &INSTRUCTIONS[opcode_converter];

    let instruction = match_process(&opcode_instruction.InstType);
    let mode = match_addr(&opcode_instruction.mode);
    let cycle = inst_cycles (opcode_instruction.cycles);
    
    print!("instruction: {:?} \n", opcode_instruction);
     
    print!("instruction: {:?} mode: {:?} cycle: {:?} \n", instruction, mode, cycle);
    return true;   
}

pub fn cpu_init() -> bool
{
    return true;
}









