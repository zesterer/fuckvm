use std::fmt;
use crate::ir::{
    OpKind,
    lir,
};

// Memory model
// ------------
// 2 overlapping memory regions
// XYXYXYXYXYXYXY
// X = stack data
// Y = scratch

#[derive(Debug)]
pub struct Program {
    entry_id: usize,
    blocks: Vec<Block>,
}

#[derive(Debug)]
pub struct Block {
    id: usize,
    instrs: Vec<Instr>,
}

#[derive(Debug)]
pub enum Instr {
    ByteAdd {
        tgt: usize,
        arg0: usize,
        arg1: usize,
    },
    ByteSub {
        tgt: usize,
        arg0: usize,
        arg1: usize,
    },
    ByteEq {
        tgt: usize,
        arg0: usize,
        arg1: usize,
    },
    ByteOut(usize),
    ByteIn(usize),
    ByteIncr(usize),
    ByteDecr(usize),
    ByteSet(usize, u8),
    Memcopy {
        from: usize,
        to: usize,
        num: usize,
    },
    Exit,
    Goto(usize),
    If(usize, usize, usize),
    Return,
}

impl Program {
    pub fn from_lir(lir: &lir::Program) -> Self {
        let mut blocks = Vec::new();

        for (id, block) in &lir.blocks {
            let mut instrs = Vec::new();

            for op in &block.ops {
                match op {
                    lir::Op::Binary(tgt, OpKind::ByteAdd, arg0, arg1) =>
                        instrs.push(Instr::ByteAdd {
                            tgt: *tgt,
                            arg0: *arg0,
                            arg1: *arg1,
                        }),
                    lir::Op::Binary(tgt, OpKind::ByteSub, arg0, arg1) =>
                        instrs.push(Instr::ByteSub {
                            tgt: *tgt,
                            arg0: *arg0,
                            arg1: *arg1,
                        }),
                    lir::Op::Binary(tgt, OpKind::ByteEq, arg0, arg1) =>
                        instrs.push(Instr::ByteEq {
                            tgt: *tgt,
                            arg0: *arg0,
                            arg1: *arg1,
                        }),
                    lir::Op::Out(src) => instrs.push(Instr::ByteOut(*src)),
                    lir::Op::In(tgt) => instrs.push(Instr::ByteIn(*tgt)),
                    lir::Op::Incr(tgt) => instrs.push(Instr::ByteIncr(*tgt)),
                    lir::Op::Decr(tgt) => instrs.push(Instr::ByteDecr(*tgt)),
                    lir::Op::Memcopy { from, to, num } =>
                        instrs.push(Instr::Memcopy {
                            from: *from,
                            to: *to,
                            num: *num,
                        }),
                    lir::Op::Decl(tgt, val) => {
                        let bytes = val.to_bytes();
                        for (i, b) in bytes.iter().enumerate() {
                            instrs.push(Instr::ByteSet(*tgt + i, *b));
                        }
                    },
                    _ => unimplemented!(),
                }
            }

            match block.branch {
                lir::Branch::Exit => instrs.push(Instr::Exit),
                lir::Branch::Goto(id) => instrs.push(Instr::Goto(id)),
                lir::Branch::If(pred, if_true, if_false) =>
                    instrs.push(Instr::If(pred, if_true, if_false)),
                lir::Branch::Return(stack_size) =>
                    instrs.push(Instr::Return),
                _ => unimplemented!(),
            }

            blocks.push(Block {
                id: *id,
                instrs,
            });
        }

        Self { entry_id: lir.entry_id, blocks }
    }

    pub fn to_bf(&self) -> String {
        let mut s = String::new();
        s += &set_byte(1, self.entry_id as u8);
        s += ">[<";
        for block in &self.blocks {
            s += &format!("  BLOCK_HEAD({})  ", block.id);
            s += &zero_byte(3);
            s += &zero_byte(5);
            s += &zero_byte(7);
            s += &add_byte_preserve(1, 3, 5);
            s += &set_byte(5, block.id as u8);
            s += &sub_byte_preserve(5, 3, 7);
            s += &not_byte(3, 7);
            s += ">>>[<<<";
            s += &format!("  BLOCK_CODE({})  ", block.id);
            for instr in &block.instrs {
                s += &instr.to_bf();
                s += "    _    ";
            }
            s += &format!("  BLOCK_END({})  ", block.id);
            s += ">>>[-]][-]<<<";
        }
        s += ">]<";
        s
    }
}

struct Repeat(char, usize);
impl fmt::Display for Repeat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.1 {
            write!(f, "{}", self.0)?;
        }
        Ok(())
    }
}

const SKIP: usize = 2;
const SCRATCH_1: usize = 5;
const SCRATCH_2: usize = 7;
const SCRATCH_3: usize = 9;

struct RepeatSkip(char, usize);
impl fmt::Display for RepeatSkip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.1 * SKIP {
            write!(f, "{}", self.0)?;
        }
        Ok(())
    }
}

fn add_byte_zeroing(from: usize, to: usize) -> String {
    format!(
        "{}[-{}{}+{}{}]{}",
        Repeat('>', from),
        Repeat('<', from),
        Repeat('>', to),
        Repeat('<', to),
        Repeat('>', from),
        Repeat('<', from),
    )
}

fn sub_byte_zeroing(from: usize, to: usize) -> String {
    format!(
        "{}[-{}{}-{}{}]{}",
        Repeat('>', from),
        Repeat('<', from),
        Repeat('>', to),
        Repeat('<', to),
        Repeat('>', from),
        Repeat('<', from),
    )
}

fn zero_byte(offs: usize) -> String {
    format!(
        "{}[-]{}",
        Repeat('>', offs),
        Repeat('<', offs),
    )
}

fn not_byte(offs: usize, scratch: usize) -> String {
    format!(
        "{}[-]{}  {}[{}{}+{}{}[-]]+{}  {}[{}{}-{}{}-]{}",
        Repeat('>', scratch),
        Repeat('<', scratch),

        Repeat('>', offs),
        Repeat('<', offs),
        Repeat('>', scratch),
        Repeat('<', scratch),
        Repeat('>', offs),
        Repeat('<', offs),

        Repeat('>', scratch),
        Repeat('<', scratch),
        Repeat('>', offs),
        Repeat('<', offs),
        Repeat('>', scratch),
        Repeat('<', scratch),
    )
}

fn add_byte_preserve(from: usize, to: usize, scratch: usize) -> String {
    zero_byte(scratch)
    + &add_byte_zeroing(from, scratch)
    + &format!(
        "{}[-{}{}+{}{}+{}{}]{}",
        Repeat('>', scratch),
        Repeat('<', scratch),
        Repeat('>', from),
        Repeat('<', from),
        Repeat('>', to),
        Repeat('<', to),
        Repeat('>', scratch),
        Repeat('<', scratch),
    )
}

fn sub_byte_preserve(from: usize, to: usize, scratch: usize) -> String {
    zero_byte(scratch)
    + &add_byte_zeroing(from, scratch)
    + &format!(
        "{}[-{}{}+{}{}-{}{}]{}",
        Repeat('>', scratch),
        Repeat('<', scratch),
        Repeat('>', from),
        Repeat('<', from),
        Repeat('>', to),
        Repeat('<', to),
        Repeat('>', scratch),
        Repeat('<', scratch),
    )
}

fn set_byte(tgt: usize, byte: u8) -> String {
    format!(
        "{}[-]{}{}",
        Repeat('>', tgt),
        Repeat('+', byte as usize),
        Repeat('<', tgt),
    )
}

impl Instr {
    pub fn to_bf(&self) -> String {
        match self {
            Instr::ByteAdd { tgt, arg0, arg1 } => {
                zero_byte(*tgt * 2)
                + &add_byte_preserve(*arg0 * 2, *tgt * 2, SCRATCH_1)
                + &add_byte_preserve(*arg1 * 2, *tgt * 2, SCRATCH_1)
            },
            Instr::ByteSub { tgt, arg0, arg1 } => {
                zero_byte(*tgt * 2)
                + &add_byte_preserve(*arg0 * 2, *tgt * 2, SCRATCH_1)
                + &sub_byte_preserve(*arg1 * 2, *tgt * 2, SCRATCH_1)
            },
            Instr::ByteEq { tgt, arg0, arg1 } => {
                zero_byte(SCRATCH_1)
                + &add_byte_preserve(*arg0 * 2, SCRATCH_1, SCRATCH_2)
                + &sub_byte_preserve(*arg1 * 2, SCRATCH_1, SCRATCH_2)
                + &zero_byte(*tgt * 2) + &add_byte_zeroing(SCRATCH_1, *tgt * 2)
            },
            Instr::ByteOut(src) =>
                format!("{}.{}", RepeatSkip('>', *src), RepeatSkip('<', *src)),
            Instr::ByteIn(tgt) =>
                format!("{},{}", RepeatSkip('>', *tgt), RepeatSkip('<', *tgt)),
            Instr::ByteIncr(tgt) =>
                format!("{}+{}", RepeatSkip('>', *tgt), RepeatSkip('<', *tgt)),
            Instr::ByteDecr(tgt) =>
                format!("{}-{}", RepeatSkip('>', *tgt), RepeatSkip('<', *tgt)),
            Instr::ByteSet(tgt, byte) =>
                set_byte(*tgt * 2, *byte),
            Instr::Memcopy { from, to, num } => {
                let mut s = String::new();
                s += &format!("{}", RepeatSkip('>', *to));
                for _ in 0..*num {
                    s += "[-]>>";
                }
                s += &format!("{}", RepeatSkip('<', *to + *num));
                for i in 0..*num {
                    s += &(
                        zero_byte((*to + i) * 2)
                        + &add_byte_preserve((*from + i) * 2, (*to + i) * 2, SCRATCH_1)
                    );
                }
                s
            },
            Instr::Exit => set_byte(1, 0),
            Instr::Goto(id) => set_byte(1, *id as u8),
            Instr::If(pred, if_true, if_false) => {
                set_byte(SCRATCH_1, 0)
                + &add_byte_preserve(*pred * 2, SCRATCH_1, SCRATCH_2)
                + &set_byte(SCRATCH_2, 0)
                + &format!(
                    "{}{}[{}{}{}]{}",
                    set_byte(1, *if_false as u8),
                    Repeat('>', SCRATCH_1),
                    Repeat('<', SCRATCH_1),
                    set_byte(1, *if_true as u8),
                    Repeat('>', SCRATCH_2),
                    Repeat('<', SCRATCH_2),
                )
            },
            Instr::Return => {
                format!("<+[-[>>->>]<<+]<<<")
            },
        }
    }
}
