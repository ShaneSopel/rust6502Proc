pub mod instruction;
use crate::instruction::INSTRUCTIONS;

// Addr mode functions.
fn accumulator_addr() -> u8
{

}

fn absolute_addr() -> u8
{

}

fn absolute_x_addr() -> u8
{

}

fn absolute_y_addr() -> u8
{

}

fn immediate_addr() -> u8
{

}

fn implied_addr() -> u8
{

}

fn indirect_addr() -> u8
{

}

fn indirect_x_addr() -> u8
{

}

fn indirect_y_addr() -> u8
{

}

fn jam_addr() -> u8
{

}

fn relative_addr() -> u8
{

}

fn zero_page_addr() -> u8
{

}

fn zero_page_x_addr() -> u8
{

}

// Instruction Type
fn ADC() -> u8
{

}

fn AND() -> u8
{

}

fn ASL() -> u8
{

}

fn BCC() -> u8
{

}

fn BCS() -> u8
{

}

fn BEQ() -> u8
{

}

fn BIT() -> u8
{

}

fn BRK() -> u8
{

}

fn BPL() -> u8
{

}

fn BMI() -> u8
{

}

fn BNE() -> u8
{

}

fn BVC() -> u8
{

}

fn BVS() -> u8
{

}

fn CLC() -> u8
{

}

fn CLD() -> u8
{

}

fn CLI() -> u8
{

}

fn CLV() -> u8
{

}

fn CMP() -> u8
{

}

fn CPX() -> u8
{

}

fn CPY() -> u8
{

}

fn DEC() -> u8
{

}

fn DEX() -> u8
{

}

fn DEY() -> u8
{

}

fn EOR() -> u8
{

}

fn INC() -> u8
{

}

fn INX() -> u8
{

}

fn INY() -> u8
{

}

fn JSR() -> u8
{

}

fn LDA() -> u8
{

}

fn LDX() -> u8
{

}

fn LDY() -> u8
{

}

fn LSR() -> u8
{

}

fn NOP() -> u8
{

}

fn ORA() -> u8
{

} 

fn PHA() -> u8
{

}

fn PHP() -> u8
{

}

fn PLP() -> u8
{

}

fn ROL() -> u8
{

}

fn ROR() -> u8
{

}

fn RTI() -> u8
{

}

fn RTS() -> u8
{

}

fn SBC() -> u8
{

}

fn SEC() -> u8
{

}

fn SED() -> u8
{

}

fn SEI() -> u8
{

}

fn STA() -> u8
{

}

fn STX() -> u8
{

}

fn STY() -> u8
{

}

fn TAX() -> u8
{

}

fn TAY() -> u8
{

}

fn TSX() -> u8
{

}

fn TXA() -> u8
{

}

fn TXS() -> u8
{

}

fn TYA() -> u8
{

}

// configure the processor instruction we need for the CPU.
pub const PROCESSORS: [InstType] =
[
    instruction::InstructionType::BRK = break_interrupt();





]

// return the processor intstruction type
PROCESSORS getProc (instruction::InstructionType type) -> bool
{
    return PROCESSORS [type];
}


// link addr mode function with the enum
pub const ADDRMODE: [AddrMode] =
[

]

// return the addrmode
ADDRMODE getAddrMode (Instruction::AddrMode addr) -> bool
{
    RETURN ADDRMODE: [addr];
}