#[path = "cpuproc.rs"] pub mod cpuproc;

use std::ptr::null;

use crate::cpuproc::Cpu_Execution;

#[derive(Debug)]
enum CondType
{
    CT_N    , // (1 << 7)  Negative Flag (N) 
    CT_V    , // (1 << 6)  Overflow Flag (V)
    CT_NONE , // (1 << 5)  Ignore
    CT_B    , // (1 << 4)  Break
    CT_D    , // (1 << 3)  Decimal (use BCD for arithmetics)
    CT_I    , // (1 << 2)  Interrupt (IQR disable)
    CT_Z    , // (1 << 1)  Zero
    CT_C    , // (1 << 0)  Carry
}

pub fn get_flag(flag: CondType, con: &mut Cpu_Execution) -> u8
{
    match flag
    {
       CondType::CT_N    => return if (con.RT_SR & 1 << 7) > 0 {1} else {0},
       CondType::CT_V    => return if (con.RT_SR & 1 << 6) > 0 {1} else {0},
       CondType::CT_NONE => 0,
       CondType::CT_B    => return if (con.RT_SR & 1 << 4) > 0 {1} else {0},
       CondType::CT_D    => return if (con.RT_SR & 1 << 3) > 0 {1} else {0},
       CondType::CT_I    => return if (con.RT_SR & 1 << 2) > 0 {1} else {0},
       CondType::CT_Z    => return if (con.RT_SR & 1 << 1) > 0 {1} else {0},
       CondType::CT_C    => return if (con.RT_SR & 1 << 0) > 0 {1} else {0},
    }
}

pub fn set_flag(flag: CondType, val: bool, con: &mut Cpu_Execution)
{
    
    match flag
    {
       CondType::CT_N    => if val == true {con.RT_SR |= (1 << 7);} else {con.RT_SR &= !(1 << 7)},
       CondType::CT_V    => if val == true {con.RT_SR |= (1 << 6);} else {con.RT_SR &= !(1 << 6)},
       CondType::CT_NONE => 0,
       CondType::CT_B    => if val == true {con.RT_SR |= (1 << 4);} else {con.RT_SR &= !(1 << 4)}
       CondType::CT_D    => if val == true {con.RT_SR |= (1 << 3);} else {con.RT_SR &= !(1 << 3)}
       CondType::CT_I    => if val == true {con.RT_SR |= (1 << 2);} else {con.RT_SR &= !(1 << 2)}
       CondType::CT_Z    => if val == true {con.RT_SR |= (1 << 1);} else {con.RT_SR &= !(1 << 1)}
       CondType::CT_C    => if val == true {con.RT_SR |= (1 << 0);} else {con.RT_SR &= !(1 << 0)}
    }
    
}