
// Address modes 
enum addr_mode
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
    REL, // relative
    ZPG, //zero page
    ZPG_X, // zeropage, x-indexed
    ZPG_Y // zeropage, y-indexed
}

enum instruction_type
{
    ADC, // add with carry
    AND, // and (with accumlator)
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
    DEC, // decrement
    DEX, // decrement X
    DEY, // decrement y 
    EOR, // exclusive or (with accumulator)
    INC, // increment
    INX, // increment x
    INY, // increment y
    JMP, // jump
    JSR, // jump subroutine
    LDA, // load accumulator
    LDX, // load x
    LDY, // load y
    LSR, // logical shift right
    NOP, // no operation
    ORA, // or with accumulator
    PHA, // push accumulator
    PHP, // push processor status (SR)
    PLA, // pull accumulator
    PLP, // pull processor status (SR)
    ROL, // rotate left
    ROR, // rotate right
    RTI, // return from interrupt
    RTS, // return from subroutine
    SBC, // subtract with carry
    SEC, // set carry
    SED, // set decimal
    SEI, // set interrupt disable
    STA, // store accumulator
    STX, // store x
    STY, // store y
    TAX, // transfer accumulator x
    TAY, // transfer accumulator y
    TSX, // transfer stack pointer to x
    TXA, // transfer x to accumulator
    TXS, // transfer x to stack pointer
    TYA // transfer y to accumulator
}

enum cond_type
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

enum reg_type
{
    RT_PC, // Program Counter Registers
    RT_AC, // Accumulator Register
    RT_X, // Index Register X
    RT_Y, // Index Register Y
    RT_SR, // Status Register
    RT_SP // Stack Pointer
}

type addr = addr_mode;
type inst = instruction_type;

struct instruction
{
    // instruction type
    inst type;



}
