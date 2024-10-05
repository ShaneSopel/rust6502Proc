use std::fmt::Debug;

// Address modes 
#[derive(Debug)]
pub enum AddrMode
{
    A, // accumulator
    ABS, // absolute
    AbsX, // absolute, x-indexed
    AbsY, // absolute, y-indexed
    IMM, // immediate
    IMP, // implied
    IND, // indirect
    IndX, // x-indexed, indirect
    IndY, // indirect, y-indexed
    JAM, // JAM
    REL, // relative
    ZPG, //zero page
    ZpgX, // zeropage, x-indexed
    ZpgY // zeropage, y-indexed
}

#[derive(Debug)]
pub enum InstructionType
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
    ISC, // INC oper + SBC Oper
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
    TYA,// transfer y to accumulator
    USBC // SBC oper + NOP
}

#[derive(Debug)]
pub struct Instruction
{
    // instruction type
    pub inst_type: InstructionType,

    // addrs mode
   pub  mode: AddrMode,
    
    //CPU Cycles
   pub  cycles: u8
}


//Look Up Table
pub const INSTRUCTIONS: [Instruction; 0x100] =
[
    // 0x00 - 0xF0
    Instruction {inst_type: InstructionType::BRK, mode: AddrMode::IMP, cycles: 7},
    Instruction {inst_type: InstructionType::ORA, mode: AddrMode::IndX, cycles: 6},
    Instruction {inst_type: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {inst_type: InstructionType::SLO, mode: AddrMode::ZPG, cycles: 5},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::ORA, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::ASL, mode: AddrMode::ZPG, cycles: 5},
    Instruction {inst_type: InstructionType::SLO, mode: AddrMode::ZPG, cycles: 5},
    Instruction {inst_type: InstructionType::PHP, mode: AddrMode::IMP, cycles: 3},
    Instruction {inst_type: InstructionType::ORA, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::ASL, mode: AddrMode::A, cycles: 2},
    Instruction {inst_type: InstructionType::ANC, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::ORA, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::ASL, mode: AddrMode::ABS, cycles: 6},
    Instruction {inst_type: InstructionType::SLO, mode: AddrMode::ABS, cycles: 6},

    // 0x10
    Instruction {inst_type: InstructionType::BPL, mode: AddrMode::REL, cycles: 2},
    Instruction {inst_type: InstructionType::ORA, mode: AddrMode::IndY, cycles: 5},
    Instruction {inst_type: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {inst_type: InstructionType::SLO, mode: AddrMode::IndY, cycles: 8},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::ORA, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::ASL, mode: AddrMode::ZpgX, cycles: 6},
    Instruction {inst_type: InstructionType::SLO, mode: AddrMode::ZpgX, cycles: 6},
    Instruction {inst_type: InstructionType::CLC, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::ORA, mode: AddrMode::AbsY, cycles: 4},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::SLO, mode: AddrMode::AbsY, cycles: 7},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::ORA, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::ASL, mode: AddrMode::AbsX, cycles: 7},
    Instruction {inst_type: InstructionType::SLO, mode: AddrMode::AbsX, cycles: 7},

    // 0x20
    Instruction {inst_type: InstructionType::JSR, mode: AddrMode::ABS, cycles: 3},
    Instruction {inst_type: InstructionType::AND, mode: AddrMode::IndX, cycles: 6},
    Instruction {inst_type: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {inst_type: InstructionType::RLA, mode: AddrMode::IndX, cycles: 8},
    Instruction {inst_type: InstructionType::BIT, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::AND, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::ROL, mode: AddrMode::ZPG, cycles: 5},
    Instruction {inst_type: InstructionType::RLA, mode: AddrMode::ZPG, cycles: 5},
    Instruction {inst_type: InstructionType::PLP, mode: AddrMode::IMP, cycles: 4},
    Instruction {inst_type: InstructionType::AND, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::ROL, mode: AddrMode::A, cycles: 2},
    Instruction {inst_type: InstructionType::ANC, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::BIT, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::AND, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::ROL, mode: AddrMode::ABS, cycles: 6},
    Instruction {inst_type: InstructionType::RLA, mode: AddrMode::ABS, cycles: 6},

    // 0x30
    Instruction {inst_type: InstructionType::BMI, mode: AddrMode::REL, cycles: 2},
    Instruction {inst_type: InstructionType::AND, mode: AddrMode::IndY, cycles: 5},
    Instruction {inst_type: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {inst_type: InstructionType::RLA, mode: AddrMode::IndY, cycles: 8},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::AND, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::ROL, mode: AddrMode::ZpgX, cycles: 6},
    Instruction {inst_type: InstructionType::RLA, mode: AddrMode::ZpgX, cycles: 6},
    Instruction {inst_type: InstructionType::SEC, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::AND, mode:AddrMode::AbsY, cycles: 4},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::RLA, mode: AddrMode::AbsY, cycles: 7},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::AND, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::ROL, mode: AddrMode::AbsX, cycles: 7},
    Instruction {inst_type: InstructionType::RLA, mode: AddrMode::AbsX, cycles: 7},

    // 0x40
    Instruction {inst_type: InstructionType::RTI, mode: AddrMode::IMP, cycles: 6},
    Instruction {inst_type: InstructionType::EOR, mode: AddrMode::IndX, cycles: 6},
    Instruction {inst_type: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {inst_type: InstructionType::SRE, mode: AddrMode::IndX, cycles: 8},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::ZPG, cycles: 4},
    Instruction {inst_type: InstructionType::EOR, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::LSR, mode: AddrMode::ZPG, cycles: 5},
    Instruction {inst_type: InstructionType::SRE, mode: AddrMode::ZPG, cycles: 5},
    Instruction {inst_type: InstructionType::PHA, mode: AddrMode::IMP, cycles: 3},
    Instruction {inst_type: InstructionType::EOR, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::LSR, mode: AddrMode::A, cycles: 2},
    Instruction {inst_type: InstructionType::ALR, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::JMP, mode: AddrMode::ABS, cycles: 3},
    Instruction {inst_type: InstructionType::EOR, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::LSR, mode: AddrMode::ABS, cycles: 6},
    Instruction {inst_type: InstructionType::SRE, mode: AddrMode::ABS, cycles: 6},

    // 0x50
    Instruction {inst_type: InstructionType::BVC, mode: AddrMode::REL, cycles: 2},
    Instruction {inst_type: InstructionType::EOR, mode: AddrMode::IndY, cycles: 5},
    Instruction {inst_type: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {inst_type: InstructionType::SRE, mode: AddrMode::IndY, cycles: 8},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::EOR, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::LSR, mode: AddrMode::ZpgX, cycles: 6},
    Instruction {inst_type: InstructionType::SRE, mode: AddrMode::ZpgX, cycles: 6},
    Instruction {inst_type: InstructionType::CLI, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::EOR, mode: AddrMode::AbsY, cycles: 4},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::SRE, mode: AddrMode::AbsY, cycles: 7},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::EOR, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::LSR, mode: AddrMode::AbsX, cycles: 7},
    Instruction {inst_type: InstructionType::SRE, mode: AddrMode::AbsX, cycles: 7},

    // 0x60
    Instruction {inst_type: InstructionType::RTS, mode: AddrMode::IMP, cycles: 6},
    Instruction {inst_type: InstructionType::ADC, mode: AddrMode::IndX, cycles: 6},
    Instruction {inst_type: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {inst_type: InstructionType::RRA, mode: AddrMode::IndX, cycles: 8},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::ZPG, cycles: 4},
    Instruction {inst_type: InstructionType::ADC, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::ROR, mode: AddrMode::ZPG, cycles: 5},
    Instruction {inst_type: InstructionType::RRA, mode: AddrMode::ZPG, cycles: 5},
    Instruction {inst_type: InstructionType::PLA, mode: AddrMode::IMP, cycles: 4},
    Instruction {inst_type: InstructionType::ADC, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::ROR, mode: AddrMode::A, cycles: 2},
    Instruction {inst_type: InstructionType::ARR, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::JMP, mode: AddrMode::IND, cycles: 5},
    Instruction {inst_type: InstructionType::ADC, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::ROR, mode: AddrMode::ABS, cycles: 6},
    Instruction {inst_type: InstructionType::RRA, mode: AddrMode::ABS, cycles: 6},

    // 0x70
    Instruction {inst_type: InstructionType::BVS, mode: AddrMode::REL, cycles: 2},
    Instruction {inst_type: InstructionType::ADC, mode: AddrMode::IndY, cycles: 5},
    Instruction {inst_type: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {inst_type: InstructionType::RRA, mode: AddrMode::IndY, cycles: 8},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::ADC, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::ROR, mode: AddrMode::ZpgX, cycles: 5},
    Instruction {inst_type: InstructionType::RRA, mode: AddrMode::ZpgX, cycles: 6},
    Instruction {inst_type: InstructionType::SEI, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::ADC, mode: AddrMode::AbsY, cycles: 4},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::IMP, cycles:2},
    Instruction {inst_type: InstructionType::RRA, mode: AddrMode::AbsY, cycles: 7},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::ADC, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::ROR, mode: AddrMode::AbsX, cycles: 7},
    Instruction {inst_type: InstructionType::RRA, mode: AddrMode::AbsX, cycles: 7},

    // 0x80
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::STA, mode: AddrMode::IndX, cycles: 6},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::SAX, mode: AddrMode::IndX, cycles: 6},
    Instruction {inst_type: InstructionType::STY, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::STA, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::STX, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::SAX, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::DEY, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::TXA, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::ANE, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::STY, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::STA, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::STX, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::SAX, mode: AddrMode::ABS, cycles: 4},

    // 0x90
    Instruction {inst_type: InstructionType::BCC, mode: AddrMode::REL, cycles: 2},
    Instruction {inst_type: InstructionType::STA, mode: AddrMode::IndY, cycles: 6},
    Instruction {inst_type: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {inst_type: InstructionType::SHA, mode: AddrMode::IndY, cycles: 6},
    Instruction {inst_type: InstructionType::STY, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::STA, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::STX, mode: AddrMode::ZpgY, cycles: 4},
    Instruction {inst_type: InstructionType::SAX, mode: AddrMode::ZpgY, cycles: 4},
    Instruction {inst_type: InstructionType::TYA, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::STA, mode: AddrMode::AbsY, cycles: 5},
    Instruction {inst_type: InstructionType::TXS, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::TAS, mode: AddrMode::AbsY, cycles: 5},
    Instruction {inst_type: InstructionType::SHY, mode: AddrMode::AbsX, cycles: 5},
    Instruction {inst_type: InstructionType::STA, mode: AddrMode::AbsX, cycles: 5},
    Instruction {inst_type: InstructionType::SHX, mode: AddrMode::AbsY, cycles: 5},
    Instruction {inst_type: InstructionType::SHA, mode: AddrMode::AbsY, cycles: 5},

    // 0xA0
    Instruction {inst_type: InstructionType::LDY, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::LDA, mode: AddrMode::IndX, cycles: 6},
    Instruction {inst_type: InstructionType::LDX, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::LAX, mode: AddrMode::IndX, cycles:6},
    Instruction {inst_type: InstructionType::LDY, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::LDA, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::LDX, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::LAX, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::TAY, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::LDA, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::TAX, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::LXA, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::LDY, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::LDA, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::LDX, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::LAX, mode: AddrMode::ABS, cycles: 4},

    // 0xB0
    Instruction {inst_type: InstructionType::BCS, mode: AddrMode::REL, cycles: 2},
    Instruction {inst_type: InstructionType::LDA, mode: AddrMode::IndY, cycles: 5},
    Instruction {inst_type: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {inst_type: InstructionType::LAX, mode: AddrMode::IndY, cycles: 5},
    Instruction {inst_type: InstructionType::LDY, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::LDA, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::LDX, mode: AddrMode::ZPG, cycles: 4},
    Instruction {inst_type: InstructionType::LAX, mode: AddrMode::ZpgY, cycles: 4},
    Instruction {inst_type: InstructionType::CLV, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::LDA, mode: AddrMode::AbsY, cycles: 4},
    Instruction {inst_type: InstructionType::TSX, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::LAS, mode: AddrMode::AbsY, cycles: 4},
    Instruction {inst_type: InstructionType::LDY, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::LDA, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::LDX, mode: AddrMode::AbsY, cycles: 4},
    Instruction {inst_type: InstructionType::LAX, mode: AddrMode::AbsY, cycles: 4},

    // 0xC0
    Instruction {inst_type: InstructionType::CPY, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::CMP, mode: AddrMode::IndX, cycles: 6},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::DCP, mode: AddrMode::IndX, cycles: 8},
    Instruction {inst_type: InstructionType::CPY, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::CMP, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::DEC, mode: AddrMode::ZPG, cycles: 5},
    Instruction {inst_type: InstructionType::DCP, mode: AddrMode::ZPG, cycles: 5},
    Instruction {inst_type: InstructionType::INY, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::CMP, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::DEX, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::SBX, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::CPY, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::CMP, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::DEC, mode: AddrMode::ABS, cycles: 6},
    Instruction {inst_type: InstructionType::DCP, mode: AddrMode::ABS, cycles: 6},

    // 0xD0
    Instruction {inst_type: InstructionType::BNE, mode: AddrMode::REL, cycles: 2},
    Instruction {inst_type: InstructionType::CMP, mode: AddrMode::IndY, cycles: 5},
    Instruction {inst_type: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {inst_type: InstructionType::DCP, mode: AddrMode::IndY, cycles: 8},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::CMP, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::DEC, mode: AddrMode::ZpgX, cycles: 6},
    Instruction {inst_type: InstructionType::DCP, mode: AddrMode::ZpgX, cycles: 6},
    Instruction {inst_type: InstructionType::CLD, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::CMP, mode: AddrMode::AbsY, cycles: 4},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::DCP, mode: AddrMode::AbsY, cycles: 7},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::CMP, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::DEC, mode: AddrMode::AbsX, cycles: 7},
    Instruction {inst_type: InstructionType::DCP, mode: AddrMode::AbsX, cycles: 7},

    // 0xE0
    Instruction {inst_type: InstructionType::CPX, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::SBC, mode: AddrMode::IndX, cycles: 6},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::ISC, mode: AddrMode::IndX, cycles: 8},
    Instruction {inst_type: InstructionType::CPX, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::SBC, mode: AddrMode::ZPG, cycles: 3},
    Instruction {inst_type: InstructionType::INC, mode: AddrMode::ZPG, cycles: 5},
    Instruction {inst_type: InstructionType::ISC, mode: AddrMode::ZPG, cycles: 5},
    Instruction {inst_type: InstructionType::INX, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::SBC, mode: AddrMode::AbsY, cycles: 4},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::USBC, mode: AddrMode::IMM, cycles: 2},
    Instruction {inst_type: InstructionType::CPX, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::SBC, mode: AddrMode::ABS, cycles: 4},
    Instruction {inst_type: InstructionType::INC, mode: AddrMode::ABS , cycles: 6},
    Instruction {inst_type: InstructionType::ISC, mode: AddrMode::ABS, cycles: 6},
    
    // 0xF0
    Instruction {inst_type: InstructionType::BEQ, mode: AddrMode::REL, cycles: 2},
    Instruction {inst_type: InstructionType::SBC, mode: AddrMode::IndY, cycles: 5},
    Instruction {inst_type: InstructionType::JAM, mode: AddrMode::JAM, cycles: 0},
    Instruction {inst_type: InstructionType::ISC, mode: AddrMode::IndY, cycles: 8},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::SBC, mode: AddrMode::ZpgX, cycles: 4},
    Instruction {inst_type: InstructionType::INC, mode: AddrMode::ZpgX, cycles: 6},
    Instruction {inst_type: InstructionType::ISC, mode: AddrMode::ZpgX, cycles: 6},
    Instruction {inst_type: InstructionType::SED, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::SBC, mode: AddrMode::AbsY, cycles: 4},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::IMP, cycles: 2},
    Instruction {inst_type: InstructionType::ISC, mode: AddrMode::AbsY, cycles: 7},
    Instruction {inst_type: InstructionType::NOP, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::SBC, mode: AddrMode::AbsX, cycles: 4},
    Instruction {inst_type: InstructionType::INC, mode: AddrMode::AbsX, cycles: 7},
    Instruction {inst_type: InstructionType::ISC, mode: AddrMode::AbsX, cycles: 7}

];
