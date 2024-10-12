#[path = "instruction.rs"] pub mod instruction;
#[path = "cpu.rs"] pub mod cpu;

use cpu::{cpu_read, cpu_write};

use crate::cpuproc::instruction as inst;
use crate::{cpu::SystemState};

#[derive(Debug)]
pub enum CondType
{
    // negative flag indicates negative value using bit position 7.
    CtN    , // (1 << 7)  Negative Flag (N) 
    CtV    , // (1 << 6)  Overflow Flag (V)
    CtNone , // (1 << 5)  Ignore
    CtB    , // (1 << 4)  Break
    CtD    , // (1 << 3)  Decimal (use BCD for arithmetics)
    CtI    , // (1 << 2)  Interrupt (IQR disable)
    //zero flag is used to indicate all zero bits
    CtZ    , // (1 << 1)  Zero
    //used to carry over overflow in arithmetic operations.
    CtC    , // (1 << 0)  Carry
}

pub struct CpuExecution
{
    pub fetch: u8,
    pub temp: u16,
    pub addr_abs: u16,
    pub addr_rel: u16,
    pub opcode: u8,
    pub cycles: u8,
    pub clock_count: u32,

    pub rt_pc: u16, // Program Counter Registers
    pub rt_ac: u8, // Accumulator Register
    pub rt_x: u8, // Index Register X
    pub rt_y: u8, // Index Register Y
    pub rt_sr: u8, // Status Register
    pub rt_sp: u8, // Stack Pointer
    pub rt_none: u8
}

// Addr mode functions.
//fn accumulator_addr(con: &mut CpuExecution) -> u8
//{
//    println!("this is accumulator and 13");
//    return 1;
//}

fn absolute_addr(con: &mut CpuExecution) -> u8
{
    let lo : u16 = cpu::cpu_read(con.rt_pc as u16);
    con.rt_pc = con.rt_pc + 1;

    let hi : u16 = cpu::cpu_read(con.rt_pc as u16);
    con.rt_pc = con.rt_pc + 1;

    con.addr_abs = (hi << 8) | lo;

    return 0;
}

fn absolute_x_addr(con: &mut CpuExecution) -> u8
{
    let lo : u16 = cpu::cpu_read(con.rt_pc as u16);
    con.rt_pc = con.rt_pc + 1;
    let hi : u16 = cpu::cpu_read(con.rt_pc as u16);
    con.rt_pc = con.rt_pc + 1;

    con.addr_abs = (hi << 8) | lo;
    con.addr_abs += con.rt_x as u16;

    if (con.addr_abs & 0xFF00) != (hi << 8)
    {
        return 1;
    }
    else 
    {
        return 0;
    }

}

fn absolute_y_addr(con: &mut CpuExecution) -> u8
{
    let lo : u16 = cpu::cpu_read(con.rt_pc as u16);
    con.rt_pc = con.rt_pc + 1;
    let hi : u16 = cpu::cpu_read(con.rt_pc as u16);
    con.rt_pc = con.rt_pc + 1;

    con.addr_abs = (hi << 8) | lo;
    con.addr_abs += con.rt_y as u16;

    if (con.addr_abs & 0xFF00) != (hi << 8)
    {
        return 1;
    }
    else 
    {
        return 0;
    }
}

fn immediate_addr(con: &mut CpuExecution) -> u8
{
    println!("this is immediate");
    con.addr_abs = (con.rt_pc + 1) as u16;
    return 0;
}

fn implied_addr(con: &mut CpuExecution) -> u8
{
    con.fetch = con.rt_ac;
    return 0;
}

fn indirect_addr(con: &mut CpuExecution) -> u8
{
    let lo : u16 = cpu::cpu_read(con.rt_pc as u16);
    con.rt_pc = con.rt_pc + 1;

    let hi : u16 = cpu::cpu_read(con.rt_pc as u16);
    con.rt_pc = con.rt_pc + 1;

    let ptr : u16 = (hi << 8) | lo;

    if lo == 0x00FF
    {
        con.addr_abs = (cpu::cpu_read(ptr & 0xFF00) << 8) | cpu::cpu_read(ptr + 0);

    }
    else 
    {
        con.addr_abs = (cpu::cpu_read(ptr + 1) << 8) | cpu::cpu_read(ptr + 0);
    }

    return 0;

}

fn indirect_x_addr(con: &mut CpuExecution) -> u8
{
    let t : u16 = cpu::cpu_read(con.rt_pc);
    con.rt_pc = con.rt_pc + 1;

    let lo : u16 = cpu::cpu_read((t+(con.rt_x as u16))&0x00FF);
    let hi : u16 = cpu::cpu_read((t+(con.rt_x as u16 )+1)&0x00FF);

    con.addr_abs = (hi << 8) | lo;

    return 0;
}

fn indirect_y_addr(con: &mut CpuExecution) -> u8
{
    let t : u16 = cpu::cpu_read(con.rt_pc);
    con.rt_pc = con.rt_pc + 1;

    let lo : u16 = cpu::cpu_read(t &0x00FF);
    let hi : u16 = cpu::cpu_read((t+1)&0x00FF);

    con.addr_abs = (hi << 8) | lo;
    con.addr_abs = con.addr_abs + (con.rt_y as u16);

    if (con.addr_abs & 0xFF00) != (hi << 8)
    {
        return 1;
    }
    else
    {
        return 0;
    }
}

fn jam_addr() -> u8
{
    return 1;
}

fn relative_addr(con: &mut CpuExecution) -> u8
{
    con.addr_rel = cpu::cpu_read(con.rt_pc);
    con.rt_pc = con.rt_pc + 1;

    if (con.addr_rel & 0x80) == 1
    {
        con.addr_rel |= 0xFF00;
    }
    return 0;
}

fn zero_page_addr(con: &mut CpuExecution) -> u8
{
    con.addr_abs = cpu::cpu_read(con.rt_pc);
    con.rt_pc =  con.rt_pc + 1;
    con.addr_abs &= 0x00FF;
    return 0;
}

fn zero_page_x_addr(con: &mut CpuExecution) -> u8
{
    con.addr_abs = cpu::cpu_read(con.rt_pc as u16 + con.rt_x as u16);
    con.rt_pc =  con.rt_pc + 1;
    con.addr_abs &= 0x00FF;
    return 0;
}

fn zero_page_y_addr(con: &mut CpuExecution) -> u8
{
    con.addr_abs = cpu::cpu_read(con.rt_pc as u16 + con.rt_y as u16);
    con.rt_pc =  con.rt_pc + 1;
    con.addr_abs &= 0x00FF;
    return 0;

}

// Instruction Type
fn adc(con: &mut CpuExecution) -> u8
{
    let temp = con.rt_ac + con.fetch + get_flag(CondType::CtC, con);

    let zval : bool = (temp & 0x00FF) == 1; 
    set_flag(CondType::CtZ,  zval, con);

    let nval : bool = (temp & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);
        
    let cval : bool = temp > 255;
    set_flag(CondType::CtC, cval, con);

    let vval : bool = !((con.rt_ac ^ con.fetch) & (con.rt_ac ^ temp)) == 1;
    set_flag(CondType::CtV, vval, con);

    con.rt_ac = temp & 0x00FF;
    
    return 1;
}

fn and(con: &mut CpuExecution) -> u8
{
    con.rt_ac = con.rt_ac & con.fetch;

    let zval : bool = con.rt_ac == 0x00;
    set_flag(CondType::CtC, zval, con);

    let nval : bool = (con.rt_ac & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);

    return 1;
}

fn asl(con: &mut CpuExecution) -> u8
{
    let temp =  get_flag(CondType::CtC, con);

    let zval : bool = con.rt_ac == 0x00FF;
    set_flag(CondType::CtC, zval, con);

    let nval : bool = (con.rt_ac  & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);

    //set_flag(CondType::CtC, (temp & 0xFF00) > 0, con);

    // add more logic

    return 1;

}

fn bcc(con: &mut CpuExecution) -> u8
{
    if get_flag(CondType::CtC, con) == 0
    {
        con.cycles = con.cycles +1;
        con.addr_abs = con.rt_pc + con.addr_rel;

        if(con.addr_abs & 0xFF00) != (con.rt_pc & 0xFF00)
        {
            con.cycles = con.cycles +1;
        }

        con.rt_pc = con.addr_abs;
    }

    return 0;
}

fn bcs(con: &mut CpuExecution) -> u8
{
    if get_flag(CondType::CtC, con) == 1
    {
        con.cycles = con.cycles + 1;
        con.addr_abs = con.rt_pc + con.addr_rel;

        if(con.addr_abs & 0xFF00) != (con.rt_pc & 0xFF00)
        {
            con.cycles = con.cycles +1;
        }

        con.rt_pc = con.addr_abs;

    }
    return 0;
}

fn beq(con: &mut CpuExecution) -> u8
{
    if get_flag(CondType::CtZ, con) == 1
    {
        con.cycles= con.cycles + 1;
        con.addr_abs =  con.rt_pc + con.addr_rel;

        if(con.addr_abs & 0xFF00) != (con.rt_pc & 0xFF00)
        {
            con.cycles = con.cycles + 1;
        }

        con.rt_pc = con.addr_abs;
    }
    return 0;

}

fn bit(con: &mut CpuExecution) -> u8
{
    let temp : u8 = con.rt_ac & con.fetch;

    let zval : bool = temp & 0x00FF == 0x00;
    set_flag(CondType::CtZ, zval, con);

    let vval : bool = con.fetch & (1 << 6) == 1;
    set_flag(CondType::CtV, vval, con);

    let nval : bool = con.fetch & (1 << 7) == 1;
    set_flag(CondType::CtN, nval, con);

    return 0;
}

fn brk(con: &mut CpuExecution) -> u8
{
    con.rt_pc = con.rt_pc + 1;

    
    set_flag(CondType::CtI, true, con);
    cpu_write(0x0100 + con.rt_sp as u16, ((con.rt_pc >> 8) & 0x00FF) as u8);
    con.rt_sp = con.rt_sp - 1;
    cpu_write(0x0100 + con.rt_sp as u16, (con.rt_pc & 0x00FF) as u8);
    con.rt_sp = con.rt_sp - 1;

    set_flag(CondType::CtB,true, con);
    cpu_write(0x100 + con.rt_sp as u16, con.rt_sr);
    con.rt_sp = con.rt_sp - 1;
    set_flag(CondType::CtB, false, con);

    con.rt_pc = cpu_read(0xFFFE) | (cpu_read(0xFFFF) << 8);

    return 0;
}

fn bpl(con: &mut CpuExecution) -> u8
{
    if get_flag(CondType::CtN, con) == 0
    {
        con.cycles = con.cycles + 1;
        con.addr_abs =  con.rt_pc + con.addr_rel;

        if(con.addr_abs & 0xFF00) != (con.rt_pc & 0xFF00)
        {
            con.cycles = con.cycles +1;
        }

        con.rt_pc = con.addr_abs;

    }
    return 0;

}

fn bmi(con: &mut CpuExecution) -> u8
{
    if get_flag(CondType::CtN, con) == 1
    {
        con.cycles= con.cycles + 1;
        con.addr_abs =  con.rt_pc + con.addr_rel;

        if(con.addr_abs & 0xFF00) != (con.rt_pc & 0xFF00)
        {
            con.cycles = con.cycles + 1;
        }

        con.rt_pc = con.addr_abs;
    }
    return 0;
}

fn bne(con: &mut CpuExecution) -> u8
{
    if get_flag(CondType::CtZ, con) == 0
    {
        con.cycles= con.cycles + 1;
        con.addr_abs =  con.rt_pc + con.addr_rel;

        if(con.addr_abs & 0xFF00) != (con.rt_pc & 0xFF00)
        {
            con.cycles = con.cycles + 1;
        }

        con.rt_pc = con.addr_abs;
    }
    return 0;

}

fn bvc(con: &mut CpuExecution) -> u8
{
    if get_flag(CondType::CtV, con) == 0
    {
        con.cycles= con.cycles + 1;
        con.addr_abs =  con.rt_pc + con.addr_rel;

        if(con.addr_abs & 0xFF00) != (con.rt_pc & 0xFF00)
        {
            con.cycles = con.cycles + 1;
        }

        con.rt_pc = con.addr_abs;
    }
    return 0;

}

fn bvs(con: &mut CpuExecution) -> u8
{
    if get_flag(CondType::CtV, con) == 1
    {
        con.cycles= con.cycles + 1;
        con.addr_abs =  con.rt_pc + con.addr_rel;

        if(con.addr_abs & 0xFF00) != (con.rt_pc & 0xFF00)
        {
            con.cycles = con.cycles + 1;
        }

        con.rt_pc = con.addr_abs;
    }
    return 1;
}

fn clc(con: &mut CpuExecution) -> u8
{
    set_flag(CondType::CtC, false, con);
    return 0;
}

fn cld(con: &mut CpuExecution) -> u8
{
    set_flag(CondType::CtD, false, con);
    return 0;
}

fn cli(con: &mut CpuExecution) -> u8
{
    set_flag(CondType::CtI, false, con);
    return 0;
}

fn clv(con: &mut CpuExecution) -> u8
{
    set_flag(CondType::CtV, false, con);
    return 0;
}

fn cmp() -> u8
{

    return 1;

}

fn cpx() -> u8
{
    return 1;

}

fn cpy() -> u8
{
    return 1;

}

fn dec() -> u8
{
    return 1;

}

fn dex(con: &mut CpuExecution) -> u8
{
    con.rt_x = con.rt_x - 1;

    let zval : bool = (con.rt_x == 0x00) == true;
    set_flag(CondType::CtZ, zval,con);

    let nval : bool = (con.rt_ac & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);
    return 0;
    
}

fn dey(con: &mut CpuExecution) -> u8
{
    con.rt_y = con.rt_y - 1;

    let zval : bool = (con.rt_y == 0x00) == true;
    set_flag(CondType::CtZ, zval,con);

    let nval : bool = (con.rt_ac & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);
    return 0;
}

fn eor() -> u8
{
    return 1;
}

fn inc() -> u8
{
    return 1;
}

fn inx(con: &mut CpuExecution) -> u8
{
    con.rt_x = con.rt_x + 1;
    
    let zval : bool = (con.rt_x == 0x00);
    set_flag(CondType::CtZ, zval, con);

    let nval : bool = (con.rt_x & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);
    return 0;
}

fn iny(con: &mut CpuExecution) -> u8
{
    con.rt_y = con.rt_y + 1;
    
    let zval : bool = (con.rt_y == 0x00);
    set_flag(CondType::CtZ, zval, con);

    let nval : bool = (con.rt_y & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);
    return 0;
}

fn jmp(con: &mut CpuExecution) -> u8
{
    con.rt_pc = con.addr_abs;
    return 0;
}

fn jsr() -> u8
{
    return 1;
}


fn lda(con: &mut CpuExecution) -> u8
{
    con.rt_ac = con.fetch;

    let zval : bool = (con.rt_ac == 0x00);
    set_flag(CondType::CtZ, zval, con);

    let nval : bool = (con.rt_ac & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);

    return 1;
}

fn ldx(con: &mut CpuExecution) -> u8
{
    con.rt_x = con.fetch;

    let zval : bool = (con.rt_ac == 0x00);
    set_flag(CondType::CtZ, zval, con);

    let nval : bool = (con.rt_ac & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);
    return 1;
}

fn ldy(con: &mut CpuExecution) -> u8
{
    con.rt_y = con.fetch;

    let zval : bool = (con.rt_ac == 0x00);
    set_flag(CondType::CtZ, zval, con);

    let nval : bool = (con.rt_ac & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);
    return 1;
}

fn lsr() -> u8
{
    return 1;
}

fn ora() -> u8
{
    return 1;
} 

fn pla() -> u8
{
    return 1;
}

fn pha() -> u8
{
    return 1;
}

fn php() -> u8
{
    return 1;
}

fn plp() -> u8
{
    return 1;
}

fn rol() -> u8
{
    return 1;
}

fn ror() -> u8
{
    return 1;
}

fn rti() -> u8
{
    return 1;
}

fn rts() -> u8
{
    return 1;

}

fn sbc() -> u8
{
    return 1;
}

fn sec(con: &mut CpuExecution) -> u8
{
    set_flag(CondType::CtC, true, con);
    return 0;
}

fn sed(con: &mut CpuExecution) -> u8
{
    set_flag(CondType::CtD, true, con);
    return 0;
}

fn sei(con: &mut CpuExecution) -> u8
{
    set_flag(CondType::CtI, true, con);
    return 0;
}

fn sta(con: &mut CpuExecution) -> u8
{
    cpu_write(con.addr_abs, con.rt_ac);
    return 0;
}

fn stx(con: &mut CpuExecution) -> u8
{
    cpu_write(con.addr_abs, con.rt_x);
    return 0;
}

fn sty(con: &mut CpuExecution) -> u8
{
    cpu_write(con.addr_abs, con.rt_y);
    return 0;
}

fn tax(con: &mut CpuExecution) -> u8
{
    con.rt_x = con.rt_ac;
    
    let zval : bool = (con.rt_x == 0x00);
    set_flag(CondType::CtZ, zval, con);

    let nval : bool = (con.rt_x & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);

    return 0;
}

fn tay(con: &mut CpuExecution) -> u8
{
    con.rt_y = con.rt_ac;
    
    let zval : bool = (con.rt_y == 0x00);
    set_flag(CondType::CtZ, zval, con);

    let nval : bool = (con.rt_y & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);

    return 0;
}

fn tsx(con: &mut CpuExecution) -> u8
{  
    con.rt_x = con.rt_sp;
    
    let zval : bool = (con.rt_x == 0x00);
    set_flag(CondType::CtZ, zval, con);

    let nval : bool = (con.rt_x & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);

    return 0;
}

fn txa(con: &mut CpuExecution) -> u8
{
    con.rt_ac = con.rt_x;

    let zval : bool = (con.rt_ac == 0x00);
    set_flag(CondType::CtZ, zval, con);

    let nval : bool = (con.rt_ac & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);

    return 0;
}

fn txs(con: &mut CpuExecution) -> u8
{
    con.rt_sp = con.rt_x;
    return 0;
}

fn tya(con: &mut CpuExecution) -> u8
{
    con.rt_ac = con.rt_y;

    let zval : bool = (con.rt_ac == 0x00);
    set_flag(CondType::CtZ, zval, con);

    let nval : bool = (con.rt_ac & 0x80) == 1;
    set_flag(CondType::CtN, nval, con);

    return 0;

}

fn illegal_opcode() -> u8
{
    return 0;
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
       CondType::CtN    => if val == true {con.rt_sr|= 1 << 7;} else {con.rt_sr &= !(1 << 7)},
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

// configure the processor instruction we need for the CPU.
pub fn match_process(inst_type: &inst::InstructionType, con: &mut CpuExecution) -> u8
{
    match inst_type
    {
        inst::InstructionType::ADC => adc(con), //legal
        inst::InstructionType::ANC => illegal_opcode(),
        inst::InstructionType::AND => and(con),
        inst::InstructionType::ANE => illegal_opcode(),
        inst::InstructionType::ALR => illegal_opcode(), //illegal
        inst::InstructionType::ARR => illegal_opcode(),
        inst::InstructionType::ASL => asl(con),
        inst::InstructionType::BCC => bcc(con),
        inst::InstructionType::BCS => bcs(con),
        inst::InstructionType::BEQ => beq(con),
        inst::InstructionType::BIT => bit(con),
        inst::InstructionType::BMI => bmi(con),
        inst::InstructionType::BNE => bne(con),
        inst::InstructionType::BPL => bpl(con),
        inst::InstructionType::BRK => brk(con),
        inst::InstructionType::BVC => bvc(con),
        inst::InstructionType::BVS => bvs(con),
        inst::InstructionType::CLC => clc(con),
        inst::InstructionType::CLD => cld(con),
        inst::InstructionType::CLI => cli(con),
        inst::InstructionType::CLV => clv(con),
        inst::InstructionType::CMP => cmp(),
        inst::InstructionType::CPX => cpx(),
        inst::InstructionType::CPY => cpy(),
        inst::InstructionType::DCP => illegal_opcode(),
        inst::InstructionType::DEC => dec(),
        inst::InstructionType::DEX => dex(con),
        inst::InstructionType::DEY => dey(con),
        inst::InstructionType::EOR => eor(),
        inst::InstructionType::INC => inc(),
        inst::InstructionType::INX => inx(con),
        inst::InstructionType::INY => iny(con),
        inst::InstructionType::ISC => illegal_opcode(),
        inst::InstructionType::JAM => illegal_opcode(),
        inst::InstructionType::JMP => jmp(con),
        inst::InstructionType::JSR => jsr(),
        inst::InstructionType::LAS => illegal_opcode(),
        inst::InstructionType::LAX => illegal_opcode(),
        inst::InstructionType::LDA => lda(con),
        inst::InstructionType::LDX => ldx(con),
        inst::InstructionType::LDY => ldy(con),
        inst::InstructionType::LSR => lsr(),
        inst::InstructionType::LXA => illegal_opcode(),
        inst::InstructionType::NOP => illegal_opcode(),
        inst::InstructionType::ORA => ora(),
        inst::InstructionType::PHA => pha(),
        inst::InstructionType::PHP => php(),
        inst::InstructionType::PLA => pla(),
        inst::InstructionType::PLP => plp(),
        inst::InstructionType::RLA => illegal_opcode(),
        inst::InstructionType::ROL => rol(),
        inst::InstructionType::ROR => ror(),
        inst::InstructionType::RRA => illegal_opcode(),
        inst::InstructionType::RTI => rti(),
        inst::InstructionType::RTS => rts(),
        inst::InstructionType::SAX => illegal_opcode(),
        inst::InstructionType::SBC => sbc(),
        inst::InstructionType::SBX => illegal_opcode(),
        inst::InstructionType::SEC => sec(con),
        inst::InstructionType::SED => sed(con),
        inst::InstructionType::SEI => sei(con),
        inst::InstructionType::SHA => illegal_opcode(),
        inst::InstructionType::SHX => illegal_opcode(),
        inst::InstructionType::SHY => illegal_opcode(),
        inst::InstructionType::SLO => illegal_opcode(),
        inst::InstructionType::SRE => illegal_opcode(),
        inst::InstructionType::STA => sta(con),
        inst::InstructionType::STX => stx(con),
        inst::InstructionType::STY => sty(con),
        inst::InstructionType::TAS => illegal_opcode(),
        inst::InstructionType::TAX => tax(con),
        inst::InstructionType::TAY => tay(con),
        inst::InstructionType::TSX => tsx(con),
        inst::InstructionType::TXA => txa(con),
        inst::InstructionType::TXS => txs(con),
        inst::InstructionType::TYA => tya(con),
        inst::InstructionType::USBC => illegal_opcode()
    }
}


// link addr mode function with the enum

// configure the processor instruction we need for the CPU.
pub fn match_addr(addr_type: &inst::AddrMode, con: &mut CpuExecution) -> u8
{
    match addr_type
    {
       inst::AddrMode::A => implied_addr(con),
       inst::AddrMode::ABS => absolute_addr(con),
       inst::AddrMode::AbsX => absolute_x_addr(con),
       inst::AddrMode::AbsY => absolute_y_addr(con),
       inst::AddrMode::IMM => immediate_addr(con),
       inst::AddrMode::IMP => implied_addr(con),
       inst::AddrMode::IND => indirect_addr(con),
       inst::AddrMode::IndX => indirect_x_addr(con),
       inst::AddrMode::IndY => indirect_y_addr(con),
       inst::AddrMode::JAM => jam_addr(),
       inst::AddrMode::REL => relative_addr(con),
       inst::AddrMode::ZPG => zero_page_addr(con),
       inst::AddrMode::ZpgX => zero_page_x_addr(con),
       inst::AddrMode::ZpgY => zero_page_y_addr(con)
    }
    
}

pub fn inst_cycles(cycles: u8) -> u8
{
    let cyc = cycles;
    return cyc;
}
