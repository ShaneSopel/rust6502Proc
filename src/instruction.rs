use std::{fmt::Debug, ops::Add};


// Address modes 
#[derive(Debug)]
enum AddrMode
{
    A, // accumulator
    ABS, // absolute
    ABS_X, // absolute, x-indexed
    ABS_Y, // absolute, y-indexed
    IMM, // immediate
    IMP, // implied
    IND, // indirect
    IND_X, // x-indexed, indirect
    IND_Y, // indirect, y-indexed
    JAM, // JAM
    REL, // relative
    ZPG, //zero page
    ZPG_X, // zeropage, x-indexed
    ZPG_Y // zeropage, y-indexed
}

#[derive(Debug)]
enum InstructionType
{
    ADC, // add with carry
    ANC, // AND Op + Set C as ASL
    AND, // and (with accumlator)
    ANE, // OR X + And oper
    ALR, // and oper + LSR
    ARR, // and oper + ROR
    ASL, // arithmetic shift left
    BCC, // branch on carry clear
    BCS, // branch on carry set
    BEQ, // branch on equal (zero set)
    BIT, // bit test
    BMI, // branch on minus (negative set)
    BNE, // branch on not equal (zero clear)
    BPL, // branch on plus (negative clear)
    BRK, // break / interrupt
    BVC, // branch on overflow clear
    BVS, // branch on overflow set
    CLC, // clear carry
    CLD, // clear decimal
    CLI, // clear interrupt disable
    CLV, // clear overflow
    CMP, // compare (with accumlator)
    CPX, // compare with x
    CPY, // compare with y
    DCP, // DEC oper + CMP oper
    DEC, // decrement
    DEX, // decrement X
    DEY, // decrement y 
    EOR, // exclusive or (with accumulator)
    INC, // increment
    INX, // increment x
    INY, // increment y
    JAM, // JAM
    JMP, // jump
    JSR, // jump subroutine
    LAS, // LAS (LAR)
    LAX, // LDA oper + LDX oper
    LDA, // load accumulator
    LDX, // load x
    LDY, // load y
    LSR, // logical shift right
    LXA, // (LAX  Immediate)
    NOP, // no operation
    ORA, // or with accumulator
    PHA, // push accumulator
    PHP, // push processor status (SR)
    PLA, // pull accumulator
    PLP, // pull processor status (SR)
    RRA, // ROR Oper + ADC oper
    RLA, // ROL oper + AND oper
    ROL, // rotate left
    ROR, // rotate right
    RTI, // return from interrupt
    RTS, // return from subroutine
    SAX, // (AXS, AAX)
    SBX, // (AXS, SAX)
    SBC, // subtract with carry
    SEC, // set carry
    SED, // set decimal
    SEI, // set interrupt disable
    SHA, // (AHX, AXA)
    SHX, // (A11, SXA, XAS)
    SHY, // (A11, SYA, SAY)
    SLO, // 
    SRE, // 
    STA, // store accumulator
    STX, // store x
    STY, // store y
    TAS, // (XAS, SHS)
    TAX, // transfer accumulator x
    TAY, // transfer accumulator y
    TSX, // transfer stack pointer to x
    TXA, // transfer x to accumulator
    TXS, // transfer x to stack pointer
    TYA // transfer y to accumulator
}
/* 
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
}*/

#[derive(Debug)]
pub struct Instruction
{
    // instruction type
    InstType: InstructionType,

    // addrs mode
    mode: AddrMode,
    
    //CPU Cycles
    cycles: u8
}


//Look Up Table
pub const INSTRUCTIONS: [Instruction; 0x100] =
[
    // 0x00 - 0xF0
    Instruction {InstType: InstructionType::BRK, mode: AddrMode::IMP, cycles: 7},
    Instruction {InstType: InstructionType::ORA, mode: AddrMode::IND_X, cycles: 6},
    Instruction {InstType: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {InstType: InstructionType::SLO, mode: AddrMode::ZPG, cycles: 5},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::ORA, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::ASL, mode: AddrMode::ZPG, cycles: 5},
    Instruction {InstType: InstructionType::SLO, mode: AddrMode::ZPG, cycles: 5},
    Instruction {InstType: InstructionType::PHP, mode: AddrMode::IMP, cycles: 3},
    Instruction {InstType: InstructionType::ORA, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::ASL, mode: AddrMode::A, cycles: 2},
    Instruction {InstType: InstructionType::ANC, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::ORA, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::ASL, mode: AddrMode::ABS, cycles: 6},
    Instruction {InstType: InstructionType::SLO, mode: AddrMode::ABS, cycles: 6},

    // 0x10
    Instruction {InstType: InstructionType::BPL, mode: AddrMode::REL, cycles: 2},
    Instruction {InstType: InstructionType::ORA, mode: AddrMode::IND_Y, cycles: 5},
    Instruction {InstType: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {InstType: InstructionType::SLO, mode: AddrMode::IND_Y, cycles: 8},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::ZPG_X, cycles: 4},
    Instruction {InstType: InstructionType::ORA, mode: AddrMode::ZPG_X, cycles: 4},
    Instruction {InstType: InstructionType::ASL, mode: AddrMode::ZPG_X, cycles: 6},
    Instruction {InstType: InstructionType::SLO, mode: AddrMode::ZPG_X, cycles: 6},
    Instruction {InstType: InstructionType::CLC, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::ORA, mode: AddrMode::ABS_Y, cycles: 4},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::SLO, mode: AddrMode::ABS_Y, cycles: 7},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::ABS_X, cycles: 4},
    Instruction {InstType: InstructionType::ORA, mode: AddrMode::ABS_X, cycles: 4},
    Instruction {InstType: InstructionType::ASL, mode: AddrMode::ABS_X, cycles: 7},
    Instruction {InstType: InstructionType::SLO, mode: AddrMode::ABS_X, cycles: 7},

    // 0x20
    Instruction {InstType: InstructionType::JSR, mode: AddrMode::ABS, cycles: 3},
    Instruction {InstType: InstructionType::AND, mode: AddrMode::IND_X, cycles: 6},
    Instruction {InstType: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {InstType: InstructionType::RLA, mode: AddrMode::IND_X, cycles: 8},
    Instruction {InstType: InstructionType::BIT, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::AND, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::ROL, mode: AddrMode::ZPG, cycles: 5},
    Instruction {InstType: InstructionType::RLA, mode: AddrMode::ZPG, cycles: 5},
    Instruction {InstType: InstructionType::PLP, mode: AddrMode::IMP, cycles: 4},
    Instruction {InstType: InstructionType::AND, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::ROL, mode: AddrMode::A, cycles: 2},
    Instruction {InstType: InstructionType::ANC, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::BIT, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::AND, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::ROL, mode: AddrMode::ABS, cycles: 6},
    Instruction {InstType: InstructionType::RLA, mode: AddrMode::ABS, cycles: 6},

    // 0x30
    Instruction {InstType: InstructionType::BMI, mode: AddrMode::REL, cycles: 2},
    Instruction {InstType: InstructionType::AND, mode: AddrMode::IND_Y, cycles: 5},
    Instruction {InstType: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {InstType: InstructionType::RLA, mode: AddrMode::IND_Y, cycles: 8},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::ZPG_X, cycles: 4},
    Instruction {InstType: InstructionType::AND, mode: AddrMode::ZPG_X, cycles: 4},
    Instruction {InstType: InstructionType::ROL, mode: AddrMode::ZPG_X, cycles: 6},
    Instruction {InstType: InstructionType::RLA, mode: AddrMode::ZPG_X, cycles: 6},
    Instruction {InstType: InstructionType::SEC, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::AND, mode:AddrMode::ABS_Y, cycles: 4},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::RLA, mode: AddrMode::ABS_Y, cycles: 7},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::ABS_X, cycles: 4},
    Instruction {InstType: InstructionType::AND, mode: AddrMode::ABS_X, cycles: 4},
    Instruction {InstType: InstructionType::ROL, mode: AddrMode::ABS_X, cycles: 7},
    Instruction {InstType: InstructionType::RLA, mode: AddrMode::ABS_X, cycles: 7},

    // 0x40
    Instruction {InstType: InstructionType::RTI, mode: AddrMode::IMP, cycles: 6},
    Instruction {InstType: InstructionType::EOR, mode: AddrMode::IND_X, cycles: 6},
    Instruction {InstType: InstructionType::JAM, mode: AddrMode::JAM, cycle: 0},
    Instruction {InstType: InstructionType::SRE, mode: AddrMode::IND_X, cycles: 8},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::ZPG, cycles: 4},
    Instruction {InstType: InstructionType::EOR, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::LSR, mode: AddrMode::ZPG, cycles: 5},
    Instruction {InstType: InstructionType::SRE, mode: AddrMode::ZPG, cycles: 5},
    Instruction {InstType: InstructionType::PHA, mode: AddrMode::IMP, cycles: 3},
    Instruction {InstType: InstructionType::EOR, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::LSR, mode: AddrMode::A, cycles: 2},
    Instruction {InstType: InstructionType::ALR, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::JMP, mode: AddrMode::ABS, cycles: 3},
    Instruction {InstType: InstructionType::EOR, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::LSR, mode: AddrMode::ABS, cycles: 6},
    Instruction {InstType: InstructionType::SRE, mode: AddrMode::ABS, cycles: 6},

    // 0x50
    Instruction {InstType: InstructionType::BVC, mode: AddrMode::REL, cycles: 2},
    Instruction {InstType: InstructionType::EOR, mode: AddrMode::IND_Y, cycles: 5},
    Instruction {InstType: InstructionType::JAM. mode: AddrMode::JAM, cycles: 0},
    Instruction {InstType: InstructionType::SRE, mode: AddrMode::IND_Y, cycles: 8},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::ZPG_X, cycles: 4},
    Instruction {InstType: InstructionType::EOR, mode: AddrMode::ZPG_X, cycles: 4},
    Instruction {InstType: InstructionType::LSR, mode: AddrMode::ZPG_X, cycles: 6},
    Instruction {InstType: InstructionType::SRE, mode: AddrMode::ZPG_X, cycles: 6},
    Instruction {InstType: InstructionType::CLI, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::EOR, mode: AddrMode::ABS_Y, cycles: 4},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::SRE, mode: AddrMode::ABS_Y, cycles: 7},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::ABS_X, cycles: 4},
    Instruction {InstType: InstructionType::EOR, mode: AddrMode::ABS_X, cycles: 4},
    Instruction {InstType: InstructionType::LSR, mode: AddrMode::ABS_X, cycles: 7},
    Instruction {InstType: InstructionType::SRE, mode: AddrMode::ABS_X, cycles: 7},

    // 0x60
    Instruction {InstType: InstructionType::RTS, mode: AddrMode::IMP, cycles: 6},
    Instruction {InstType: InstructionType::ADC, mode: AddrMode::IND_X, cycles: 6},
    Instruction {InstType: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {InstType: InstructionType::RRA, mode: AddrMode::IND_X, cycles: 8},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::ZPG, cycles: 4},
    Instruction {InstType: InstructionType::ADC, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::ROR, mode: AddrMode::ZPG, cycles: 5},
    Instruction {InstType: InstructionType::RRA, mode: AddrMode::ZPG, cycles: 5},
    Instruction {InstType: InstructionType::PLA, mode: AddrMode::IMP, cycles: 4},
    Instruction {InstType: InstructionType::ADC, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::ROR, mode: AddrMode::A, cycles: 2},
    Instruction {InstType: InstructionType::ARR, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::JMP, mode: AddrMode::IND, cycles: 5},
    Instruction {InstType: InstructionType::ADC, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::ROR, mode: AddrMode::ABS, cycles: 6},
    Instruction {InstType: InstructionType::RRA, mode: AddrMode::ABS, cycles: 6},

    // 0x70
    Instruction {InstType: InstructionType::BVS, mode: AddrMode::REL, cycles: 2},
    Instruction {InstType: InstructionType::ADC, mode: AddrMode::IND_Y, cycles: 5},
    Instruction {InstType: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {InstType: InstructionType::RRA, mode: AddrMode::IND_Y, cycles: 8},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::ZPG_X, cycles: 4},
    Instruction {InstType: InstructionType::ADC, mode: AddrMode::ZPG_X, cycles: 4},
    Instruction {InstType: InstructionType::ROR, mode: AddrMode::ZPG_X, cycles: 5},
    Instruction {InstType: InstructionType::RRA, mode: AddrMode::ZPG_X, cycles: 6},
    Instruction {InstType: InstructionType::SEI, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::ADC, mode: AddrMode::ABS_Y, cycles: 4},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::IMP, cycles:2},
    Instruction {InstType: InstructionType::RRA, mode: AddrMode::ABS_Y, cycles: 7},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::ABS_X, cycles: 4},
    Instruction {InstType: InstructionType::ADC, mode: AddrMode::ABS_X, cycles: 4},
    Instruction {InstType: InstructionType::ROR, mode: AddrMode::ABS_X, cycles: 7},
    Instruction {InstType: InstructionType::RRA, mode: AddrMode::ABS_X, cycles: 7},

    // 0x80
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::STA, mode: AddrMode::IND_X, cycles: 6},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::SAX, mode: AddrMode::IND_X, cycles: 6},
    Instruction {InstType: InstructionType::STY, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::STA, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::STX, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::SAX, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::DEY, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::TXA, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::ANE, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::STY, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::STA, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::STX, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::SAX, mode: AddrMode::ABS, cycles: 4},

    // 0x90
    Instruction {InstType: InstructionType::BCC, mode: AddrMode::REL, cycles: 2},
    Instruction {InstType: InstructionType::STA, mode: AddrMode::IND_Y, cycles: 6},
    Instruction {InstType: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {InstType: InstructionType::SHA, mode: AddrMode::IND_Y, cycles: 6},
    Instruction {InstType: InstructionType::STY, mode: AddrMode::ZPG_X, cycles: 4},
    Instruction {InstType: InstructionType::STA, mode: AddrMode::ZPG_X, cycles: 4},
    Instruction {InstType: InstructionType::STX, mode: AddrMode::ZPG_Y, cycles: 4},
    Instruction {InstType: InstructionType::SAX, mode: AddrMode::ZPG_Y, cycles: 4},
    Instruction {InstType: InstructionType::TYA, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::STA, mode: AddrMode::ABS_Y, cycles: 5},
    Instruction {InstType: InstructionType::TXS, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::TAS, mode: AddrMode::ABS_Y, cycles: 5},
    Instruction {InstType: InstructionType::SHY, mode: AddrMode::ABS_X, cycles: 5},
    Instruction {InstType: InstructionType::STA, mode: AddrMode::ABS_X, cycles: 5},
    Instruction {InstType: InstructionType::SHX, mode: AddrMode::ABS_Y, cycles: 5},
    Instruction {InstType: InstructionType::SHA, mode: AddrMode::ABS_Y, cycles: 5},

    // 0xA0
    Instruction {InstType: InstructionType::LDY, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::LDA, mode: AddrMode::IND_X, cycles: 6},
    Instruction {InstType: InstructionType::LDX, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::LAX, mode: AddrMode::IND_X, cycles:6},
    Instruction {InstType: InstructionType::LDY, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::LDA, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::LDX, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::LAX, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::TAY, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::LDA, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::TAX, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::LXA, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::LDY, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::LDA, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::LDX, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::LAX, mode: AddrMode::ABS, cycles: 4},

    // 0xB0
    Instruction {InstType: InstructionType::BCS, mode: AddrMode::REL, cycles: 2},
    Instruction {InstType: InstructionType::LDA, mode: AddrMode::IND_Y, cycles: 5},
    Instruction {InstType: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {InstType: InstructionType::LAX, mode: AddrMode::IND_Y, cycles 5},
    Instruction {InstType: InstructionType::LDY, mode: AddrMode::ZPG_X, cycles: 4},
    Instruction {InstType: InstructionType::LDA, mode: AddrMode::ZPG_X, cycles: 4},
    Instruction {InstType: InstructionType::LDX, mode: AddrMode::ZPG, cycles: 4},
    Instruction {InstType: InstructionType::LAX, mode: AddrMode::ZPG_Y, cycles: 4},
    Instruction {InstType: InstructionType::CLV, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::LDA, mode: AddrMode::ABS_Y, cycles: 4},
    Instruction {InstType: InstructionType::TSX, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::LAS, mode: AddrMode::ABS_Y, cycles: 4},
    Instruction {InstType: InstructionType::LDY, mode: AddrMode::ABS_X, cycles: 4},
    Instruction {InstType: InstructionType::LDA, mode: AddrMode::ABS_X, cycles: 4},
    Instruction {InstType: InstructionType::LDX, mode: AddrMode::ABS_Y, cycles: 4},
    Instruction {InstType: InstructionType::LAX, mode: AddrMode::ABS_Y, cycles: 4},

    // 0xC0
    Instruction {InstType: InstructionType::CPY, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::CMP, mode: AddrMode::IND_X, cycles: 6},
    Instruction {InstType: InstructionType::NOP, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::DCP, mode: AddrMode::IND_X, cycles: 8},
    Instruction {InstType: InstructionType::CPY, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::CMP, mode: AddrMode::ZPG, cycles: 3},
    Instruction {InstType: InstructionType::DEC, mode: AddrMode::ZPG, cycles: 5},
    Instruction {InstType: InstructionType::DCP, mode: AddrMode::ZPG, cycles: 5},
    Instruction {InstType: InstructionType::INY, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::CMP, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::DEX, mode: AddrMode::IMP, cycles: 2},
    Instruction {InstType: InstructionType::SBX, mode: AddrMode::IMM, cycles: 2},
    Instruction {InstType: InstructionType::CPY, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::CMP, mode: AddrMode::ABS, cycles: 4},
    Instruction {InstType: InstructionType::DEC, mode: AddrMode::ABS, cycles: 6},
    Instruction {InstType: InstructionType::DCP, mode: AddrMode::ABS, cycles: 6},

    // 0xD0

    Instruction {InstType: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    // D3
    Instruction {InstType: InstructionType::DCP, mode: AddrMode::IND_Y, cycles: 8},

    // D7
    Instruction {InstType: InstructionType::DCP, mode: AddrMode::ZPG_X, cycles: 6},

    // DB
    Instruction {InstType: InstructionType::DCP, mode: AddrMode::ABS_Y, cycles: 7},

    // DF
    Instruction {InstType: InstructionType::DCP, mode: AddrMode::ABS_X, cycles: 7},

    // 0xE0

    // 0xF0


    



];
