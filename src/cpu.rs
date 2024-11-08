
use crate::cpuproc::instruction::INSTRUCTIONS;
use crate::cpuproc::match_addr;
use crate::cpuproc::match_process;
use crate::cpuproc::inst_cycles;
use crate::cpuproc::CpuExecution;



#[derive(Debug)]
pub enum SystemState
{
    CpuInit,
    CpuInst,
    CpuSetFlag,
    None,
    Jam,
}

// so we need to process the instruction set
// the instruction, addr mode, and cycles.
pub fn process_instruction(opcode: u8, con: &mut CpuExecution) -> SystemState
{
    let opcode_converter = usize::from(opcode);
    let opcode_instruction = &INSTRUCTIONS[opcode_converter];

    let instruction = match_process(&opcode_instruction.inst_type, con);
    let mode = match_addr(&opcode_instruction.mode, con);
    let cycle = inst_cycles (opcode_instruction.cycles);
    
    print!("instruction: {:?} \n", opcode_instruction);
     
    print!("instruction: {:?} mode: {:?} cycle: {:?} \n", instruction, mode, cycle);
    return SystemState::CpuInst;   
}

pub fn cpu_init() -> SystemState
{
    return SystemState::CpuInit;
}

pub fn cpu_read(addr: u16) -> u16
{
    if addr >= 0x0000 && addr <= 0xFFFF
    {
        return addr;
    }
    return 0x00;
}

pub fn cpu_read_u8(addr: u8) -> u8
{
    if addr >= 0x0000 && addr <= 0xFFFF
    {
        return addr;
    }
    return 0x00;
}

pub fn cpu_write(addr: u16, data: u8) -> u8
{
    if addr >= 0x0000 && addr <= 0xFFFF
    {
        return data;
    }
    else  {
        return 0;
    }

}









