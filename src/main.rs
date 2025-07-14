use crate::assembler::assemble;
pub mod assembler;

fn main() {
    assemble("./src/file.txt");
}
