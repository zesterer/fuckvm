use std::collections::HashMap;
use super::{
    lir,
    Type,
    Value,
    OpKind,
};
use crate::Error;

#[derive(Debug)]
pub struct Program {
    pub(crate) funcs: HashMap<String, Function>,
}

#[derive(Debug)]
pub struct Function {
    pub(crate) output: Type,
    pub(crate) input: (String, Type),
    pub(crate) blocks: HashMap<String, Block>,
}

#[derive(Debug)]
pub struct Block {
    pub(crate) ops: Vec<Op>,
    pub(crate) branch: Branch,
}

#[derive(Debug)]
pub struct Local(pub String, pub Type);

#[derive(Debug)]
pub enum Op {
    Unary(Local, OpKind, Local),
    Binary(Local, OpKind, Local, Local),
    Decl(Local, Value),
    In(Local),
    Out(Local),
    Incr(Local),
    Decr(Local),
    Call(Local, String, Local),
}

#[derive(Debug)]
pub enum Branch {
    ReturnVal(Local),
    ReturnNone,
    Exit,
    Goto(String),
    If {
        predicate: Local,
        if_true: String,
        if_false: String,
    },
}

impl Program {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
        }
    }

    pub fn with_function(mut self, name: impl Into<String>, func: Function) -> Self {
        self.funcs.insert(name.into(), func);
        self
    }

    pub fn functions(&self) -> impl Iterator<Item=(&String, &Function)> {
        self.funcs.iter()
    }

    pub fn to_lir(&self) -> Result<lir::Program, Error> {
        lir::Program::from_hir(self)
    }
}

impl Function {
    pub fn new(output: Type, input: (impl Into<String>, Type)) -> Self {
        Self {
            output,
            input: (input.0.into(), input.1),
            blocks: HashMap::new(),
        }
    }

    pub fn with_block(mut self, name: impl Into<String>, block: Block) -> Self {
        self.blocks.insert(name.into(), block);
        self
    }

    pub fn blocks(&self) -> impl Iterator<Item=(&String, &Block)> {
        self.blocks.iter()
    }
}

impl Block {
    pub fn new(branch: Branch) -> Self {
        Self {
            ops: Vec::new(),
            branch,
        }
    }

    pub fn with_op(mut self, op: Op) -> Self {
        self.ops.push(op);
        self
    }
}

impl Op {
    pub fn byte_decl(tgt: impl Into<String>, val: Value) -> Self {
        Op::Decl(
            Local(tgt.into(), Type::Byte),
            val,
        )
    }

    pub fn byte_add(tgt: impl Into<String>, arg0: impl Into<String>, arg1: impl Into<String>) -> Self {
        Op::Binary(
            Local(tgt.into(), Type::Byte),
            OpKind::ByteAdd,
            Local(arg0.into(), Type::Byte),
            Local(arg1.into(), Type::Byte),
        )
    }

    pub fn byte_sub(tgt: impl Into<String>, arg0: impl Into<String>, arg1: impl Into<String>) -> Self {
        Op::Binary(
            Local(tgt.into(), Type::Byte),
            OpKind::ByteSub,
            Local(arg0.into(), Type::Byte),
            Local(arg1.into(), Type::Byte),
        )
    }

    pub fn byte_incr(tgt: impl Into<String>) -> Self {
        Op::Incr(Local(tgt.into(), Type::Byte))
    }

    pub fn byte_decr(tgt: impl Into<String>) -> Self {
        Op::Decr(Local(tgt.into(), Type::Byte))
    }

    pub fn byte_eq(tgt: impl Into<String>, arg0: impl Into<String>, arg1: impl Into<String>) -> Self {
        Op::Binary(
            Local(tgt.into(), Type::Byte),
            OpKind::ByteEq,
            Local(arg0.into(), Type::Byte),
            Local(arg1.into(), Type::Byte),
        )
    }

    pub fn byte_out(arg: impl Into<String>) -> Self {
        Op::Out(Local(arg.into(), Type::Byte))
    }

    pub fn byte_in(arg: impl Into<String>) -> Self {
        Op::In(Local(arg.into(), Type::Byte))
    }

    pub fn generated_local(&self) -> Option<&Local> {
        match self {
            Op::Unary(tgt, _, _) => Some(tgt),
            Op::Binary(tgt, _, _, _) => Some(tgt),
            Op::Decl(tgt, _) => Some(tgt),
            Op::In(tgt) => Some(tgt),
            Op::Out(_) => None,
            Op::Incr(tgt) => Some(tgt),
            Op::Decr(tgt) => Some(tgt),
            Op::Call(tgt, _, _) => Some(tgt),
        }
    }
}

impl Branch {
    pub fn byte_return(arg: impl Into<String>) -> Self {
        Branch::ReturnVal(Local(arg.into(), Type::Byte))
    }

    pub fn if_not_zero(predicate: impl Into<String>, if_true: impl Into<String>, if_false: impl Into<String>) -> Self {
        Branch::If {
            predicate: Local(predicate.into(), Type::Byte),
            if_true: if_true.into(),
            if_false: if_false.into(),
        }
    }
}
