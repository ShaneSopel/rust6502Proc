use cpuproc::Cpu_Execution;


pub mod cpu;
#[path = "cpuproc.rs"] pub mod cpuproc;
#[path = "cpuflags.rs"] pub mod cpuflags; 
#[path = "instruction.rs"] pub mod instruction; 


fn main() {

    let mut con: Cpu_Execution = Cpu_Execution
    {
        fetch: 0x00,
        temp: 0x0000,
        addr_abs: 0x0000,
        addr_rel: 0x00,
        opcode: 0x00,
        cycles: 0,
        clock_count: 0,

        RT_PC: 0x00, // Program Counter Registers
        RT_AC: 0x00, // Accumulator Register
        RT_X: 0x00, // Index Register X
        RT_Y: 0x00, // Index Register Y
        RT_SR: 0x00, // Status Register
        RT_SP: 0x00, // Stack Pointer
        RT_NONE: 0x00

    };

    println!("Hello, world!");
    print!("{:?} \n", cpu::process_instruction(0, & mut con));
}
