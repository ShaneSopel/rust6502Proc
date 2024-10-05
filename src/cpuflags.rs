#[path = "cpuproc.rs"] pub mod cpuproc;

use crate::{cpu::SystemState, cpuproc::CpuExecution};

#[derive(Debug)]
pub enum CondType
{
    CtN    , // (1 << 7)  Negative Flag (N) 
    CtV    , // (1 << 6)  Overflow Flag (V)
    CtNone , // (1 << 5)  Ignore
    CtB    , // (1 << 4)  Break
    CtD    , // (1 << 3)  Decimal (use BCD for arithmetics)
    CtI    , // (1 << 2)  Interrupt (IQR disable)
    CtZ    , // (1 << 1)  Zero
    CtC    , // (1 << 0)  Carry
}

pub fn get_flag(flag: CondType, con: &mut CpuExecution) -> u8
{
    match flag
    {
       CondType::CtN    => return if (con.rt_sr & 1 << 7) > 0 {1} else {0},
       CondType::CtV    => return if (con.rt_sr & 1 << 6) > 0 {1} else {0},
       CondType::CtNone => 0,
       CondType::CtB    => return if (con.rt_sr & 1 << 4) > 0 {1} else {0},
       CondType::CtD    => return if (con.rt_sr & 1 << 3) > 0 {1} else {0},
       CondType::CtI    => return if (con.rt_sr & 1 << 2) > 0 {1} else {0},
       CondType::CtZ    => return if (con.rt_sr & 1 << 1) > 0 {1} else {0},
       CondType::CtC    => return if (con.rt_sr & 1 << 0) > 0 {1} else {0},
    }
}

pub fn set_flag(flag: CondType, val: bool, con: &mut CpuExecution) -> SystemState
{
    
    match flag
    {
       CondType::CtN    => if val == true {con.rt_sr |= 1 << 7;} else {con.rt_sr &= !(1 << 7)},
       CondType::CtV    => if val == true {con.rt_sr |= 1 << 6;} else {con.rt_sr &= !(1 << 6)},
       CondType::CtNone => if val == true {return SystemState::None}
       CondType::CtB    => if val == true {con.rt_sr |= 1 << 4;} else {con.rt_sr &= !(1 << 4)}
       CondType::CtD    => if val == true {con.rt_sr |= 1 << 3;} else {con.rt_sr &= !(1 << 3)}
       CondType::CtI    => if val == true {con.rt_sr |= 1 << 2;} else {con.rt_sr &= !(1 << 2)}
       CondType::CtZ    => if val == true {con.rt_sr |= 1 << 1;} else {con.rt_sr &= !(1 << 1)}
       CondType::CtC    => if val == true {con.rt_sr |= 1 << 0;} else {con.rt_sr &= !(1 << 0)}
    }

    return SystemState::CpuSetFlag;
}