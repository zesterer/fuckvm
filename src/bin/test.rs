use fuckvm::{
    ir::{
        Type,
        Value,
        hir::{
            Program,
            Function,
            Block,
            Branch,
            Op,
        },
    },
    bf::bfir,
    vm::Vm,
};

fn main() {
    let hir = Program::new()
        .with_function("main", Function::new(Type::Empty, ("in", Type::Empty))
            .with_block("entry", Block::new(Branch::Goto("say_hi".into()))
                .with_op(Op::byte_decl("zero", Value::Byte(b'0')))
                .with_op(Op::byte_in("a_l"))
                .with_op(Op::byte_in("b_l"))
                .with_op(Op::byte_sub("a", "a_l", "zero"))
                .with_op(Op::byte_sub("b", "b_l", "zero"))
                .with_op(Op::byte_eq("answer", "a", "b"))
                .with_op(Op::byte_add("answer_l", "answer", "zero"))
                .with_op(Op::byte_out("answer_l"))
                .with_op(Op::byte_decl("count", Value::Byte(10)))
            )
            .with_block("say_hi", Block::new(Branch::Goto("say_boo".into()))
                .with_op(Op::byte_decl("hl", Value::Byte(b'h')))
                .with_op(Op::byte_decl("il", Value::Byte(b'i')))
                .with_op(Op::byte_out("hl"))
                .with_op(Op::byte_out("il"))
            )
            .with_block("say_boo", Block::new(Branch::if_not_zero("count", "say_hi", "exit"))
                .with_op(Op::byte_decl("bl", Value::Byte(b'b')))
                .with_op(Op::byte_decl("ol", Value::Byte(b'o')))
                .with_op(Op::byte_decl("newline", Value::Byte(b'\n')))
                .with_op(Op::byte_out("bl"))
                .with_op(Op::byte_out("ol"))
                .with_op(Op::byte_out("ol"))
                .with_op(Op::byte_out("newline"))
                .with_op(Op::byte_decr("count"))
            )
            .with_block("exit", Block::new(Branch::Exit))
        )
        .with_function("print_byte", Function::new(Type::Empty, ("b", Type::Byte))
            .with_block("entry", Block::new(Branch::ReturnNone)
                .with_op(Op::byte_out("b"))
            )
        );
    println!("HIR: {:?}", hir);

    let lir = hir.to_lir().unwrap();
    println!("LIR: {:?}", lir);

    let bfir = bfir::Program::from_lir(&lir);

    println!("BFIR: {:?}", bfir);

    let bf = bfir.to_bf();

    println!("BF: {}", bf);

    let mut vm = Vm::new();

    vm.exec(&bf);

    for i in 0..20 {
        print!("{}, ", vm.get(i))
    }
    print!("\n");
}
