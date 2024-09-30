
use crate::cpuproc::instruction::INSTRUCTIONS;
use crate::cpuproc::match_addr;
use crate::cpuproc::match_process;


#[derive(Debug)]
enum CondType
{
    CT_N, // Negative Flag (N) 
    CT_V, // Overflow Flag (V)
    CT_NONE, // Ignore
    CT_B, // Break
    CT_D, // Decimal (use BCD for arithmetics)
    CT_I, // Interrupt (IQR disable)
    CT_Z, // Zero
    CT_C // Carry
}

#[derive(Debug)]
enum RegType
{
    RT_PC, // Program Counter Registers
    RT_AC, // Accumulator Register
    RT_X, // Index Register X
    RT_Y, // Index Register Y
    RT_SR, // Status Register
    RT_SP, // Stack Pointer
    RT_NONE
}

// so we need to process the instruction set
// the instruction, addr mode, and cycles.
pub fn process_instruction(opcode: u8) -> bool
{
    let opcode_converter = usize::from(opcode);
    let opcode_instruction = &INSTRUCTIONS[opcode_converter];

    let instruction = match_process(&opcode_instruction.InstType);
    let mode = match_addr(&opcode_instruction.mode);
    

    print!("instruction: {:?} \n", opcode_instruction);
     
    print!("instruction: {:?} mode: {:?} \n", instruction, mode);
    return true;   
}









