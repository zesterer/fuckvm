use std::collections::HashMap;
use super::{
    hir,
    IdGenerator,
    OffsetGenerator,
    Value,
    OpKind,
};
use crate::Error;

#[derive(Debug)]
pub enum LirError {
    LocalReassigned(String),
    NoSuchLocal(String),
}

#[derive(Debug)]
pub struct Program {
    pub(crate) entry_id: usize,
    pub(crate) blocks: HashMap<usize, Block>,
}

#[derive(Debug)]
pub struct Block {
    pub(crate) ops: Vec<Op>,
    pub(crate) branch: Branch,
}

#[derive(Debug)]
pub enum Op {
    Unary(usize, OpKind, usize),
    Binary(usize, OpKind, usize, usize),
    Decl(usize, Value),
    In(usize),
    Out(usize),
    Incr(usize),
    Decr(usize),
    Memcopy {
        from: usize,
        to: usize,
        num: usize,
    },
    Call {
        /* TODO */
    },
}

#[derive(Debug)]
pub enum Branch {
    Exit,
    Return(usize),
    Goto(usize),
    If(usize, usize, usize),
}

fn gen_block_name(func: &str, block: &str) -> String {
    func.to_string() + ":" + block
}

impl Program {
    pub fn from_hir(hir_prog: &hir::Program) -> Result<Self, Error> {
        let mut id_gen = IdGenerator(1);

        let mut block_ids = HashMap::new();
        let mut func_offs = HashMap::new();

        // Iterate through functions to generate LIR
        for (func_name, func) in &hir_prog.funcs {
            // Outputs always come first
            let mut offs_gen = OffsetGenerator(func.output.size_of());
            let mut local_offs = HashMap::new();

            // Then inputs
            local_offs.insert(func.input.0.clone(), offs_gen.next(func.input.1.size_of()));

            // Generate block IDs
            for (block_name, block) in &func.blocks {
                let block_name = gen_block_name(&func_name, &block_name);
                let block_id = id_gen.next();
                block_ids.insert(block_name, block_id);

                for op in &block.ops {
                    if let Some(hir::Local(name, ty)) = op.generated_local() {
                        if !local_offs.contains_key(name) {
                            local_offs.insert(
                                name.to_string(),
                                offs_gen.next(ty.size_of()),
                            );
                        } else {
                            //return Err(Error::Lir(LirError::LocalReassigned(name.clone())));
                        }
                    }
                }
            }
            //println!("LIR offsets: {:?}", local_offs);
            func_offs.insert(func_name.clone(), (local_offs, offs_gen.0));
        }

        println!("LIR blocks: {:?}", block_ids);

        let mut blocks = HashMap::new();
        for (func_name, func) in &hir_prog.funcs {
            let (this_func_offs, func_frame_size) = func_offs.get(func_name.as_str()).unwrap();

            let local_to_offs = |name: &String| {
                *this_func_offs.get(name.as_str()).unwrap()
            };

            for (block_name, block) in &func.blocks {
                let mut ops = Vec::new();

                for op in &block.ops {
                    match op {
                        hir::Op::Unary(tgt, kind, arg) => ops.push(Op::Unary(
                            local_to_offs(&tgt.0),
                            kind.clone(),
                            local_to_offs(&arg.0),
                        )),
                        hir::Op::Binary(tgt, kind, arg0, arg1) => ops.push(Op::Binary(
                            local_to_offs(&tgt.0),
                            kind.clone(),
                            local_to_offs(&arg0.0),
                            local_to_offs(&arg1.0),
                        )),
                        hir::Op::Decl(tgt, value) => ops.push(Op::Decl(
                            local_to_offs(&tgt.0),
                            value.clone(),
                        )),
                        hir::Op::In(tgt) => ops.push(Op::In(
                            local_to_offs(&tgt.0),
                        )),
                        hir::Op::Out(src) => ops.push(Op::Out(
                            local_to_offs(&src.0),
                        )),
                        hir::Op::Incr(tgt) => ops.push(Op::Incr(
                            local_to_offs(&tgt.0),
                        )),
                        hir::Op::Decr(tgt) => ops.push(Op::Decr(
                            local_to_offs(&tgt.0),
                        )),
                        _ => unimplemented!(),
                    }
                }

                let branch = match &block.branch {
                    hir::Branch::Goto(name) => {
                        Branch::Goto(*block_ids.get(&gen_block_name(func_name, name)).unwrap())
                    },
                    hir::Branch::Exit => Branch::Exit,
                    hir::Branch::ReturnVal(hir::Local(name, ty)) => {
                        let from = local_to_offs(name);
                        ops.push(Op::Memcopy { from, to: 0, num: ty.size_of() });
                        Branch::Return(*func_frame_size)
                    },
                    hir::Branch::ReturnNone => {
                        Branch::Return(*func_frame_size)
                    },
                    hir::Branch::If { predicate, if_true, if_false } => {
                        Branch::If(
                            local_to_offs(&predicate.0),
                            *block_ids.get(&gen_block_name(func_name, if_true)).unwrap(),
                            *block_ids.get(&gen_block_name(func_name, if_false)).unwrap(),
                        )
                    },
                    _ => unimplemented!(),
                };

                let block_id = *block_ids.get(&gen_block_name(func_name, block_name)).unwrap();
                blocks.insert(block_id, Block {
                    ops,
                    branch,
                });
            }
        }

        //println!("LIR blocks: {:?}", block_ids);
        println!("LIR function offsets: {:?}", func_offs);

        Ok(Program {
            entry_id: *block_ids.get(&gen_block_name("main", "entry")).unwrap(),
            blocks,
        })
    }
}
