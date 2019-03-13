pub mod hir;
pub mod lir;

struct IdGenerator(usize);

impl IdGenerator {
    pub fn next(&mut self) -> usize {
        let old = self.0;
        self.0 += 1;
        old
    }
}

struct OffsetGenerator(usize);

impl OffsetGenerator {
    pub fn next(&mut self, sz: usize) -> usize {
        let old = self.0;
        self.0 += sz;
        old
    }
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Empty,
    Byte,
    Boolean,
    Array(Box<Type>, usize),
    Struct(Vec<Type>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Empty,
    Byte(u8),
    Boolean(bool),
    Array(Box<Value>, usize),
    Struct(Vec<Value>),
}

#[derive(Copy, Clone, Debug)]
pub enum OpKind {
    ByteCopy,
    ByteAdd,
    ByteSub,
    ByteEq,
}

impl Type {
    pub fn size_of(&self) -> usize {
        match self {
            Type::Empty => 0,
            Type::Byte => 1,
            Type::Boolean => 1,
            Type::Array(ty, n) => ty.size_of() * n,
            Type::Struct(tys) => tys.iter().map(|ty| ty.size_of()).sum(),
        }
    }
}

impl Value {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Value::Empty => vec![],
            Value::Byte(b) => vec![*b],
            Value::Boolean(b) => vec![if *b { 1 } else { 0 }],
            Value::Array(val, n) => {
                let mut v = Vec::new();
                for _ in 0..*n {
                    v.append(&mut val.to_bytes());
                }
                v
            },
            Value::Struct(items) => {
                let mut v = Vec::new();
                for item in items {
                    v.append(&mut item.to_bytes());
                }
                v
            },
        }
    }
}
