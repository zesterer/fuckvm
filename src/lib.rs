pub mod bf;
pub mod ir;
pub mod vm;

#[derive(Debug)]
pub enum Error {
    VmError(vm::VmError),
    Lir(ir::lir::LirError),
}
