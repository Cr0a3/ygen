use Ygen::Target::{instr::*, x64Reg, Reg};

pub fn main() {
    let instr = Instr::with2(
        Mnemonic::Add, 
        Operand::Reg(x64Reg::Rax.boxed()), 
        Operand::Mem(x64Reg::Rcx + x64Reg::Rdx)
    );

    println!("{}", instr);

    match instr.encode() {
        Ok(bytes) => {
            let mut fmt = String::from("0x");
    
            for byte in bytes {
                fmt += &format!("{:#04X}", byte).replace("0x", "");
            }

            println!("{}", fmt)
        }
        Err(e) => println!("{}", e),
    }
}