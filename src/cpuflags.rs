#[path = "cpuproc.rs"] pub mod cpuproc;

use crate::cpuproc::Cpu_Execution;

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

pub fn get_flag(flag: CondType)
{
    return 0;
}

pub fn set_flag(flag: CondType, val: bool, con: &mut Cpu_Execution)
{
    if (val == true)
    {
        con.RT_SR |= CondType;
    }
    else 
    {
        con.RT_SR &= ~f;
        
    }
        
}