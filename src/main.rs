pub mod instruction;
use crate::instruction::INSTRUCTIONS;

fn main() {
    println!("Hello, world!");

    for n in 1..256
    {
        print!("{:?} \n", INSTRUCTIONS[n]);
    }

    //INSTRUCTIONS[1].mode;

    print!("{:?} \n", INSTRUCTIONS[1].mode);
    
}
