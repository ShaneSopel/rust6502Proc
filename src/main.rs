
pub mod cpu;
#[path = "cpuproc.rs"] pub mod cpuproc;
#[path = "instruction.rs"] pub mod instruction; 


fn main() {
    println!("Hello, world!");
    print!("{:?} \n", cpu::process_instruction(0));
}
