#![cfg(test)]

use super::{generate_mark_custom, lir, ty, Session, Target};
use lir::{BinaryOp, Expr, TypedExpr};
use ty::{Function, Type};

use cranelift_object::object::write::{Object, StreamingBuffer};
use std::io::Read;
use std::process::{Command, Stdio};

macro_rules! typed {
    ($e:expr, $t:expr) => {
        TypedExpr(Box::leak(Box::new($e)), $t)
    };
}

fn assert_objdump(name: &str, object: Object<'static>) {
    let mut objdump = Command::new("llvm-objdump")
        .arg("-D")
        .arg("-")
        .stderr(Stdio::inherit())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut input = StreamingBuffer::new(objdump.stdin.unwrap());
    object.emit(&mut input).unwrap();
    drop(input);

    let mut stdout = objdump.stdout.take().unwrap();
    let mut output = String::new();
    stdout.read_to_string(&mut output).unwrap();

    insta::assert_snapshot!(name, output);
}

fn test_compile(name: &str, func: ty::Function, body: lir::TypedExpr) {
    let mut session = Session::new(&Target::new("riscv64"));
    session.compile_function(name, &func, &body);

    assert_objdump(name, session.finish().object);
}

#[test]
fn mark_custom() {
    let mut custom = 3;
    let module = generate_mark_custom(
        &Target::new("riscv64"),
        [Type::Unit].iter().map(|typ| (typ.gc_id(&mut custom), typ)),
    );
    assert_objdump("mark_custom", module.finish().object);
}

#[test]
fn simple_add_sub() {
    let ty_u8 = Type::Uint(8);

    let func = Function {
        params: &[ty_u8, ty_u8],
        result: ty_u8,
    };

    let add = Expr::Binary(
        BinaryOp::Add,
        typed!(Expr::AccessLocal(0), ty_u8),
        typed!(Expr::AccessLocal(1), ty_u8),
    );
    let sub = Expr::Binary(
        BinaryOp::Sub,
        typed!(add, ty_u8),
        typed!(Expr::Literal(10), ty_u8),
    );
    let ret = Expr::Return(typed!(sub, Type::Unit));
    let body = typed!(ret, Type::Never);

    test_compile("simple_add_sub", func, body);
}

#[test]
fn infinite_loop() {
    let func = Function {
        params: &[],
        result: Type::Never,
    };

    let inner = [
        typed!(
            Expr::Assign(
                typed!(Expr::AccessLocal(0), Type::Uint(64)),
                typed!(Expr::Literal(0), Type::Uint(64)),
            ),
            Type::Unit
        ),
        typed!(
            Expr::Assign(
                typed!(Expr::AccessLocal(0), Type::Uint(64)),
                typed!(
                    Expr::Binary(
                        BinaryOp::Add,
                        typed!(Expr::AccessLocal(0), Type::Uint(64)),
                        typed!(Expr::Literal(0), Type::Uint(64))
                    ),
                    Type::Unit
                )
            ),
            Type::Unit
        ),
        typed!(Expr::Literal(0), Type::Uint(64)),
    ];
    let body = typed!(Expr::Loop(&inner), Type::Never);

    test_compile("infinite_loop", func, body);
}
