use cpuproc::CpuExecution;


pub mod cpu;
#[path = "cpuproc.rs"] pub mod cpuproc; 
#[path = "instruction.rs"] pub mod instruction; 


fn main() {

    let mut con: CpuExecution = CpuExecution
    {
        fetch: 0x00,
        temp: 0x0000,
        addr_abs: 0x0000,
        addr_rel: 0x00,
        opcode: 0x00,
        cycles: 0,
        clock_count: 0,

        rt_pc: 0x00, // Program Counter Registers
        rt_ac: 0x00, // Accumulator Register
        rt_x: 0x00, // Index Register X
        rt_y: 0x00, // Index Register Y
        rt_sr: 0x00, // Status Register
        rt_sp: 0x00, // Stack Pointer
        rt_none: 0x00

    };

    println!("Hello, world!");
    print!("{:?} \n", cpu::process_instruction(0, & mut con));
}
