#[path = "instruction.rs"] pub mod instruction;

use crate::cpuproc::instruction as inst;

// Addr mode functions.
fn accumulator_addr() -> u8
{
    println!("this is accumulator and 13");
    return 1;
}

fn absolute_addr() -> u8
{
    return 1;

}

fn absolute_x_addr() -> u8
{
    return 1;
}

fn absolute_y_addr() -> u8
{
    return 1;

}

fn immediate_addr() -> u8
{
    println!("this is immediate");
    return 22;

}

fn implied_addr() -> u8
{
    println!("this is implied");
    return 23;

}

fn indirect_addr() -> u8
{
    return 1;

}

fn indirect_x_addr() -> u8
{
    return 1;

}

fn indirect_y_addr() -> u8
{
    return 1;

}

fn jam_addr() -> u8
{
    return 1;

}

fn relative_addr() -> u8
{
    return 1;

}

fn zero_page_addr() -> u8
{
    return 1;

}

fn zero_page_x_addr() -> u8
{
    return 1;

}

fn zero_page_y_addr() -> u8
{
    return 1;

}

// Instruction Type
fn adc() -> u8
{
    println!("this is adc and 13");
    return 24;
}

fn and() -> u8
{
    return 1;

}

fn asl() -> u8
{
    return 1;

}

fn bcc() -> u8
{
    return 1;

}

fn bcs() -> u8
{
    return 1;

}

fn beq() -> u8
{
    return 1;

}

fn bit() -> u8
{
    return 1;

}

fn brk() -> u8
{
    println!("this is brk and 20");
    return 20;
}

fn bpl() -> u8
{
    return 1;

}

fn bmi() -> u8
{
    return 1;

}

fn bne() -> u8
{
    return 1;

}

fn bvc() -> u8
{
    return 1;

}

fn bvs() -> u8
{
    return 1;

}

fn clc() -> u8
{
    return 1;

}

fn cld() -> u8
{
    return 1;
}

fn cli() -> u8
{
    return 1;

}

fn clv() -> u8
{
    return 1;

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

fn dex() -> u8
{
    return 1;

}

fn dey() -> u8
{
    return 1;
}

fn eor() -> u8
{
    return 1;
}

fn inc() -> u8
{
    return 1;
}

fn inx() -> u8
{
    return 1;
}

fn iny() -> u8
{
    return 1;
}

fn jmp() -> u8
{
    return 1;
}

fn jsr() -> u8
{
    return 1;
}


fn lda() -> u8
{
    return 1;
}

fn ldx() -> u8
{
    return 1;
}

fn ldy() -> u8
{
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

fn sec() -> u8
{
    return 1;
}

fn sed() -> u8
{
    return 1;
}

fn sei() -> u8
{
    return 1;
}

fn sta() -> u8
{
    return 1;
}

fn stx() -> u8
{
    return 1;
}

fn sty() -> u8
{
    return 1;
}

fn tax() -> u8
{
    return 1;
}

fn tay() -> u8
{
    return 1;
}

fn tsx() -> u8
{
    return 1;
}

fn txa() -> u8
{
    return 1;

}

fn txs() -> u8
{
    return 1;

}

fn tya() -> u8
{
    return 1;

}

fn illegal_opcode() -> u8
{
    return 1;
}

// configure the processor instruction we need for the CPU.
pub fn match_process(inst_type: &inst::InstructionType) -> u8
{
    match inst_type
    {
        inst::InstructionType::ADC => adc(), //legal
        inst::InstructionType::ANC => illegal_opcode(),
        inst::InstructionType::AND => and(),
        inst::InstructionType::ANE => illegal_opcode(),
        inst::InstructionType::ALR => illegal_opcode(), //illegal
        inst::InstructionType::ARR => illegal_opcode(),
        inst::InstructionType::ASL => asl(),
        inst::InstructionType::BCC => bcc(),
        inst::InstructionType::BCS => bcs(),
        inst::InstructionType::BEQ => beq(),
        inst::InstructionType::BIT => bit(),
        inst::InstructionType::BMI => bmi(),
        inst::InstructionType::BNE => bne(),
        inst::InstructionType::BPL => bpl(),
        inst::InstructionType::BRK => brk(),
        inst::InstructionType::BVC => bvc(),
        inst::InstructionType::BVS => bvs(),
        inst::InstructionType::CLC => clc(),
        inst::InstructionType::CLD => cld(),
        inst::InstructionType::CLI => cli(),
        inst::InstructionType::CLV => clv(),
        inst::InstructionType::CMP => cmp(),
        inst::InstructionType::CPX => cpx(),
        inst::InstructionType::CPY => cpy(),
        inst::InstructionType::DCP => illegal_opcode(),
        inst::InstructionType::DEC => dec(),
        inst::InstructionType::DEX => dex(),
        inst::InstructionType::DEY => dey(),
        inst::InstructionType::EOR => eor(),
        inst::InstructionType::INC => inc(),
        inst::InstructionType::INX => inx(),
        inst::InstructionType::INY => iny(),
        inst::InstructionType::ISC => illegal_opcode(),
        inst::InstructionType::JAM => illegal_opcode(),
        inst::InstructionType::JMP => jmp(),
        inst::InstructionType::JSR => jsr(),
        inst::InstructionType::LAS => illegal_opcode(),
        inst::InstructionType::LAX => illegal_opcode(),
        inst::InstructionType::LDA => lda(),
        inst::InstructionType::LDX => ldx(),
        inst::InstructionType::LDY => ldy(),
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
        inst::InstructionType::SEC => sec(),
        inst::InstructionType::SED => sed(),
        inst::InstructionType::SEI => sei(),
        inst::InstructionType::SHA => illegal_opcode(),
        inst::InstructionType::SHX => illegal_opcode(),
        inst::InstructionType::SHY => illegal_opcode(),
        inst::InstructionType::SLO => illegal_opcode(),
        inst::InstructionType::SRE => illegal_opcode(),
        inst::InstructionType::STA => sta(),
        inst::InstructionType::STX => stx(),
        inst::InstructionType::STY => sty(),
        inst::InstructionType::TAS => illegal_opcode(),
        inst::InstructionType::TAX => tax(),
        inst::InstructionType::TAY => tay(),
        inst::InstructionType::TSX => tsx(),
        inst::InstructionType::TXA => txa(),
        inst::InstructionType::TXS => txs(),
        inst::InstructionType::TYA => tya(),
        inst::InstructionType::USBC => illegal_opcode()
    }
}


// link addr mode function with the enum

// configure the processor instruction we need for the CPU.
pub fn match_addr(addr_type: &inst::AddrMode) -> u8
{
    match addr_type
    {
       inst::AddrMode::A => accumulator_addr(),
       inst::AddrMode::ABS => absolute_addr(),
       inst::AddrMode::ABS_X => absolute_x_addr(),
       inst::AddrMode::ABS_Y => absolute_y_addr(),
       inst::AddrMode::IMM => immediate_addr(),
       inst::AddrMode::IMP => implied_addr(),
       inst::AddrMode::IND => indirect_addr(),
       inst::AddrMode::IND_X => indirect_x_addr(),
       inst::AddrMode::IND_Y => indirect_y_addr(),
       inst::AddrMode::JAM => jam_addr(),
       inst::AddrMode::REL => relative_addr(),
       inst::AddrMode::ZPG => zero_page_addr(),
       inst::AddrMode::ZPG_X => zero_page_x_addr(),
       inst::AddrMode::ZPG_Y => zero_page_y_addr()
    }
    
}
